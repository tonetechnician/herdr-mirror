---
id: "20260903-new-worktree-works-on-mirrored-repos-like-herdr-local-3o7j"
schema: task
title: "New Worktree works on mirrored repos exactly like herdr local: intercept the native new_worktree flow (prefix+shift+g) so it creates the worktree on the mirrored host"
status: open
section: herdr-mirror
tier: medium
blocked_by:
  - "20260903-20260903-sidebar-git-preserves-native-branch-and-git-status-tokens-vyjo"
paths: []
done_when: "COMPOSER ASK 2026-09-03 (conductor relay e4c2ed6b); herdr untouched, plugin-encapsulated; seated AFTER the sidebar-git fix lands. PROBLEM: the plugin already has remote-worktree-create/open/remove actions but they are UNBOUND and absent from the native flow, so prefix+shift+g (herdr default new_worktree, src/config/model.rs:1028) on a mirror does nothing useful. DO IT LIKE tabs/splits already work: the native New Worktree invoked while focused on a mirror must create the worktree on the mirrored HOST — intercept the native flow the same way intercept-new handles tab/split/workspace; OR if herdr exposes no worktree event, bind prefix+shift+g to remote-worktree-create with the LOCAL action as the fallback, and ship 'herdr-mirror worktree-keys --write' that moves the native key aside and writes the block, exactly like sidebar-git. SCAN FIRST: establish how remote_action::worktree_cmd(create) gets its branch/name today (src/main.rs:114) — it must PROMPT the same way native does (herdr popup pane like pick-host) and pass the name to 'herdr worktree create' on the remote; the new workspace mirrors back labelled '<host>: <repo>/<branch>'. TESTS: cargo tests for the interception/binding path and the worktree-keys --write block; config off byte-for-byte today. LIVE PROOF (conductor/composer): press prefix+shift+g on the threaded-dev-2: skald/main mirror, name a branch, the lane appears on dev-2 and in the sidebar within a poll. symphony check green at tip, dirty false, Conventional Commits, rebase onto fork main after the sidebar fix. Report OUTCOME, VERIFICATION (the live sidebar read-back), NEXT STEP, PATHS, SHA."
accepted_survivors: []
declared_removals: []
created: "2026-09-03"
---
## History

- 2026-09-03 created
