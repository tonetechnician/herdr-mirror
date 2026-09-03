use std::fs;
use std::hash::Hasher;
use std::path::{Path, PathBuf};
use std::process::Command;

use fnv::FnvHasher;

use crate::util::sane_component;

fn repo_hash(repo_key: &str) -> String {
    let mut hasher = FnvHasher::default();
    hasher.write(repo_key.as_bytes());
    format!("{:016x}", hasher.finish())
}

pub(crate) fn worktree_path(
    state_dir: &Path,
    host: &str,
    repo_key: &str,
    workspace_id: &str,
) -> PathBuf {
    state_dir
        .join("shadow")
        .join(sane_component(host))
        .join(repo_hash(repo_key))
        .join("worktrees")
        .join(sane_component(workspace_id))
}

fn run_git(args: &[&std::ffi::OsStr]) -> Result<(), String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| format!("could not run git: {e}"))?;
    if output.status.success() {
        return Ok(());
    }
    let detail = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Err(if detail.is_empty() {
        "git failed without an error message".into()
    } else {
        detail
    })
}

fn unreadable_branch_path(worktree: &Path) -> PathBuf {
    worktree.with_extension("branch-unreadable")
}

fn setup_error_path(worktree: &Path) -> PathBuf {
    worktree.with_extension("setup-error")
}

pub(crate) fn first_setup_error(worktree: &Path) -> bool {
    let path = setup_error_path(worktree);
    let Some(parent) = path.parent() else {
        return true;
    };
    if fs::create_dir_all(parent).is_err() {
        return true;
    }
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .is_ok()
}

pub(crate) fn note_unreadable_branch(worktree: &Path) -> Option<usize> {
    if !worktree.join(".git").exists() {
        return None;
    }
    let path = unreadable_branch_path(worktree);
    let age = fs::read_to_string(&path)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(0)
        + 1;
    fs::write(path, age.to_string()).ok()?;
    Some(age)
}

fn clear_unreadable_branch(worktree: &Path) {
    let _ = fs::remove_file(unreadable_branch_path(worktree));
}

fn head_ref(worktree: &Path) -> Option<String> {
    let output = Command::new("git")
        .args([
            "-C",
            &worktree.display().to_string(),
            "symbolic-ref",
            "--quiet",
            "HEAD",
        ])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Ensure a content-free linked worktree whose Git metadata gives herdr a
/// native repository, branch chip, and worktree menu without touching the remote.
pub(crate) fn ensure_worktree(
    state_dir: &Path,
    host: &str,
    repo_key: &str,
    workspace_id: &str,
    branch: &str,
) -> Result<PathBuf, String> {
    if branch.is_empty() {
        return Err("remote branch is empty".into());
    }
    let worktree = worktree_path(state_dir, host, repo_key, workspace_id);
    let repo_dir = worktree
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "could not derive shadow repository path".to_string())?;
    let bare = repo_dir.join("repo.git");
    if !bare.exists() {
        fs::create_dir_all(repo_dir)
            .map_err(|e| format!("could not create shadow repository directory: {e}"))?;
        run_git(&["init".as_ref(), "--bare".as_ref(), bare.as_os_str()])?;
    }
    if !worktree.join(".git").exists() {
        let parent = worktree
            .parent()
            .ok_or_else(|| "could not derive shadow worktree parent".to_string())?;
        fs::create_dir_all(parent)
            .map_err(|e| format!("could not create shadow worktree directory: {e}"))?;
        run_git(&[
            "--git-dir".as_ref(),
            bare.as_os_str(),
            "worktree".as_ref(),
            "add".as_ref(),
            "--orphan".as_ref(),
            worktree.as_os_str(),
        ])?;
    }

    let desired = format!("refs/heads/{branch}");
    if head_ref(&worktree).as_deref() != Some(desired.as_str()) {
        run_git(&[
            "-C".as_ref(),
            worktree.as_os_str(),
            "symbolic-ref".as_ref(),
            "HEAD".as_ref(),
            desired.as_ref(),
        ])?;
    }
    clear_unreadable_branch(&worktree);
    let _ = fs::remove_file(setup_error_path(&worktree));
    Ok(worktree)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("herdr-mirror-shadow-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn path_is_per_host_repo_and_workspace() {
        let root = Path::new("/state");
        let one = worktree_path(root, "dev-2", "/trees/skald", "w1");
        assert_eq!(one.parent().unwrap().file_name().unwrap(), "worktrees");
        assert_ne!(one, worktree_path(root, "dev-3", "/trees/skald", "w1"));
        assert_ne!(one, worktree_path(root, "dev-2", "/trees/other", "w1"));
        assert_ne!(one, worktree_path(root, "dev-2", "/trees/skald", "w2"));
    }

    #[test]
    fn head_update_is_idempotent_and_clean() {
        let dir = temp_dir("head");
        let first = ensure_worktree(&dir, "dev-2", "/trees/skald", "w1", "main").unwrap();
        let head = head_ref(&first).unwrap();
        assert_eq!(head, "refs/heads/main");
        ensure_worktree(&dir, "dev-2", "/trees/skald", "w1", "main").unwrap();
        let status = Command::new("git")
            .args(["-C", first.to_str().unwrap(), "status", "--porcelain"])
            .output()
            .unwrap();
        assert!(status.status.success());
        assert!(status.stdout.is_empty());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn detached_remote_sha_is_a_native_short_chip() {
        let dir = temp_dir("detached");
        let worktree = ensure_worktree(&dir, "dev-2", "/trees/skald", "w1", "a1b2c3d").unwrap();
        assert_eq!(head_ref(&worktree).as_deref(), Some("refs/heads/a1b2c3d"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn unreadable_branch_keeps_the_last_head_with_poll_age() {
        let dir = temp_dir("unreadable");
        let worktree = ensure_worktree(&dir, "dev-2", "/trees/skald", "w1", "main").unwrap();
        assert_eq!(note_unreadable_branch(&worktree), Some(1));
        assert_eq!(note_unreadable_branch(&worktree), Some(2));
        assert_eq!(head_ref(&worktree).as_deref(), Some("refs/heads/main"));
        ensure_worktree(&dir, "dev-2", "/trees/skald", "w1", "main").unwrap();
        assert!(!unreadable_branch_path(&worktree).exists());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn setup_failure_is_not_noted_again() {
        let worktree = temp_dir("failure").join("worktrees/w1");
        assert!(first_setup_error(&worktree));
        assert!(!first_setup_error(&worktree));
        let _ = fs::remove_dir_all(worktree.parent().unwrap().parent().unwrap());
    }
}
