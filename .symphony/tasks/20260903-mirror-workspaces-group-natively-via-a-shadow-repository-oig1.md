---
id: "20260903-mirror-workspaces-group-natively-via-a-shadow-repository-oig1"
schema: task
title: mirror workspaces group as children and show the native chip and worktree dropdown via a per-remote-repo shadow repository whose linked worktrees are the mirror cwds
status: done
section: herdr-mirror
tier: medium
blocked_by: []
paths: []
done_when: "Composer ruling 2026-09-03 ~11:45, reversing the morning hold on part A after the grouping probe: try the shadow-repository shape, plugin-encapsulated, herdr untouched. PROBLEM, observed by symphony-concertmaster: a worktree created on dev-2 (herdr worktree create in skald) mirrored back as a FLAT sibling row threaded-dev-2: skald/probe-cm-grouping with worktree null; herdr derives sidebar grouping (shared worktree.repo_key), the native branch chip and the New worktree dropdown from a real git checkout at the LOCAL workspace cwd, and mirror cwds are marker dirs (mirror.rs mirror_pane_cwd). SHAPE: one bare SHADOW repository per (host, remote repo_key) under the plugin state dir ~/.local/state/herdr-mirror/shadow/<host>/<repo-hash>/; each mirror workspace cwd becomes a linked worktree of that shadow (git worktree add --detach or an unborn branch, no content), with HEAD pointed at the remote branch name (git symbolic-ref HEAD refs/heads/<branch>; detached remote shows the short sha), updated only when it changes (idempotent, herdr must not see churn each poll). Then herdr groups all mirrors of one remote repo as children of one repository natively, renders the branch chip, and shows the dropdown; the routed worktree key (worktree-keys --write) already sends create to the remote, so the dropdown creates on the host; establish by scan what else the dropdown and git status do against the shadow (git status clean by construction; say what happens on remove and open). Remote workspace with no worktree keeps the marker cwd, never a wrong group. Config: shadow_repo = true default on, off is byte-for-byte today (tokens from kj04 and pt64 stay). Failure modes named: shadow init fails gives marker cwd and one status note; remote branch unreadable keeps last HEAD with age or clears after 2x poll, consistent with the token rule. TESTS: cargo tests for shadow path derivation, HEAD update idempotence, detached, no-worktree fallback, config off. LIVE PROOF: the threaded-dev-2: skald rows group as children of one skald entry here, the chip shows main and the lane branch, a worktree created from the dropdown on a mirror row appears on dev-2 and mirrors back INSIDE the group; the conductor or composer reads the sidebar. Commit Conventional; symphony check at tip dirty false; lands on the fork main; PR to upstream with README updated (the limitation line rewritten). Report OUTCOME, VERIFICATION (sidebar read-back), NEXT STEP, PATHS, SHA."
accepted_survivors: []
declared_removals: []
created: "2026-09-03"
---
## History

- 2026-09-03 created
- 2026-09-03T09:55:15.012Z landed task/20260903-mirror-workspaces-group-natively-via-a-shadow-repository-oig1 on main as eda7dda9d529aa215e9cf11d96c73ec704e35ceb
