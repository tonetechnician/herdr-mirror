// herdr-mirror: one binary, multiple modes — dispatched on the first argument,
// exactly like herdr itself.
//
//   herdr-mirror daemon                 # control plane (foreground; `start` spawns this)
//   herdr-mirror pane <host> <target>   # data plane: one per mirror pane
//   herdr-mirror start|pause|ensure|status|once|restore|teardown
//   herdr-mirror hide|show [host]       # toggle a connection's mirrors out of view
//   herdr-mirror pick-workspace [--menu]            # popup host picker
//   herdr-mirror remote-workspace|remote-tab|remote-split <right|down>
//   herdr-mirror remote-worktree-open|remote-worktree-create|remote-worktree-remove
//   herdr-mirror remote-invoke <plugin>.<action>
//   herdr-mirror remote-actions [host]              # discovery
//   herdr-mirror bind|unbind ...                    # keybinding setup
//   herdr-mirror sidebar-git [--write]              # workspace Git token row

mod api;
mod binding;
mod closes;
mod config;
mod daemon;
mod docker;
mod foreground;
mod grid;
mod layout_sync;
mod mirror;
mod pane;
mod paste;
mod pick;
mod predict;
mod remote;
mod remote_action;
mod select;
mod ssh_relay;
mod state;
mod util;

use util::{Env, Result};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("status");
    let code = match run(cmd, &args[1..]) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            if e.to_string().starts_with("unknown command") || e.to_string().starts_with("usage") {
                2
            } else {
                1
            }
        }
    };
    std::process::exit(code);
}

fn run(cmd: &str, rest: &[String]) -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    let result = run_on(&rt, cmd, rest);
    // pane mode's blocking stdin read would hang a plain Runtime::drop forever
    rt.shutdown_background();
    result
}

fn run_on(rt: &tokio::runtime::Runtime, cmd: &str, rest: &[String]) -> Result<()> {
    match cmd {
        "daemon" | "run" => rt.block_on(daemon::cmd_run(Env::resolve()?)),
        "start" => {
            let env = Env::resolve()?;
            daemon::set_paused(&env, false); // explicit start lifts a manual pause
            // explicit start is also the sanctioned moment to repair the CLI
            // link (the daemon and the ensure hook only ever report it broken)
            if let Some(msg) = util::repair_cli_link() {
                println!("{msg}");
            }
            daemon::cmd_start(&env)
        }
        "pause" | "stop" => {
            daemon::cmd_pause(&Env::resolve()?);
            Ok(())
        }
        "ensure" => {
            // workspace.focused hook — must be cheap and silent
            daemon::cmd_ensure(&Env::resolve()?);
            Ok(())
        }
        "status" => daemon::cmd_status(&Env::resolve()?),
        "once" => rt.block_on(daemon::cmd_once(Env::resolve()?)),
        "restore" => daemon::cmd_restore(
            &Env::resolve()?,
            rest.get(1).map(String::as_str),
            rest.get(2).map(String::as_str),
        ),
        "teardown" => rt.block_on(daemon::cmd_teardown(Env::resolve()?)),
        "hide" => rt.block_on(remote_action::hide_cmd(Env::resolve()?, rest.get(1).map(String::as_str))),
        "show" => rt.block_on(remote_action::show_cmd(Env::resolve()?, rest.get(1).map(String::as_str))),
        "pane" => {
            let args = pane::parse_args(&rest[1..])?;
            rt.block_on(pane::run(args))
        }
        "pick-workspace" => {
            if rest.iter().any(|a| a == "--menu") {
                pick::menu(rt, Env::resolve()?)
            } else {
                rt.block_on(pick::summon(Env::resolve()?))
            }
        }
        "pick-worktree" => pick::worktree_menu(rt, Env::resolve()?),
        "intercept-new" => {
            // creation hooks — cheap, silent no-op when nothing matches
            let what = rest.get(1).map(String::as_str).unwrap_or("workspace");
            rt.block_on(pick::intercept(Env::resolve()?, what))
        }
        "remote-workspace" => rt.block_on(remote_action::run_cmd(Env::resolve()?, "workspace", None)),
        "remote-worktree-open" => rt.block_on(remote_action::worktree_cmd(Env::resolve()?, "open")),
        "remote-worktree-create" => rt.block_on(remote_action::worktree_cmd(Env::resolve()?, "create")),
        "remote-worktree-remove" => rt.block_on(remote_action::worktree_cmd(Env::resolve()?, "remove")),
        "remote-tab" => rt.block_on(remote_action::run_cmd(Env::resolve()?, "tab", None)),
        "remote-split" => rt.block_on(remote_action::run_cmd(
            Env::resolve()?,
            "split",
            rest.get(1).map(String::as_str),
        )),
        "remote-invoke" => {
            let spec = rest
                .get(1)
                .ok_or_else(|| util::err("usage: herdr-mirror remote-invoke <plugin>.<action>"))?;
            rt.block_on(remote_action::invoke_cmd(Env::resolve()?, spec))
        }
        "remote-actions" => rt.block_on(binding::remote_actions(
            Env::resolve()?,
            rest.get(1).map(String::as_str),
        )),
        "bind" => match (rest.get(1), rest.get(2)) {
            (Some(spec), Some(key)) => rt.block_on(binding::bind(Env::resolve()?, spec, key)),
            _ => Err(util::err("usage: herdr-mirror bind <plugin>.<action> <key>")),
        },
        "sidebar-git" => match rest.get(1).map(String::as_str) {
            None => rt.block_on(binding::sidebar_git(Env::resolve()?, false)),
            Some("--write") => rt.block_on(binding::sidebar_git(Env::resolve()?, true)),
            _ => Err(util::err("usage: herdr-mirror sidebar-git [--write]")),
        },
        "worktree-keys" => match rest.get(1).map(String::as_str) {
            None => rt.block_on(binding::worktree_keys(Env::resolve()?, false)),
            Some("--write") => rt.block_on(binding::worktree_keys(Env::resolve()?, true)),
            _ => Err(util::err("usage: herdr-mirror worktree-keys [--write]")),
        },
        "unbind" => {
            let what = rest
                .get(1)
                .ok_or_else(|| util::err("usage: herdr-mirror unbind <plugin>.<action> | <key>"))?;
            rt.block_on(binding::unbind(Env::resolve()?, what))
        }
        other => Err(util::err(format!(
            "unknown command: {other} (daemon|pane|start|pause|ensure|status|once|restore|teardown|hide|show|pick-workspace|pick-worktree|remote-workspace|remote-worktree-open|remote-worktree-create|remote-worktree-remove|remote-tab|remote-split|remote-invoke|remote-actions|bind|sidebar-git|worktree-keys|unbind)"
        ))),
    }
}
