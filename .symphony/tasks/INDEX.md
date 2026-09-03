# tasks

Derived from the documents in this directory; do not edit by hand. `symphony fmt` rewrites it.

| id | title | status | section | blocked_by |
| --- | --- | --- | --- | --- |
| 20260903-20260903-sidebar-git-preserves-native-branch-and-git-status-tokens-vyjo | sidebar-git preserves herdr's native branch and git_status tokens: append $rgit to the last row rather than replacing the defaults | done | herdr-mirror | - |
| 20260903-live-acceptance-mirror-worktrees-end-to-end-h6fp | live acceptance: mirror groups worktrees end to end on the running build — rebuild to d3fcbd4, group read-back, remote worktree create then remove, read-backs only | done | herdr-mirror | - |
| 20260903-mirror-workspace-rows-show-the-remote-git-branch-kj04 | mirror workspace rows show the remote git branch via a local shadow repo, plus forwarded rbranch, rahead and rbehind tokens; PR to upstream | done | herdr-mirror | - |
| 20260903-mirror-workspaces-group-natively-via-a-shadow-repository-oig1 | mirror workspaces group as children and show the native chip and worktree dropdown via a per-remote-repo shadow repository whose linked worktrees are the mirror cwds | done | herdr-mirror | - |
| 20260903-new-worktree-works-on-mirrored-repos-like-herdr-local-3o7j | New Worktree works on mirrored repos exactly like herdr local: intercept the native new_worktree flow (prefix+shift+g) so it creates the worktree on the mirrored host | done | herdr-mirror | 20260903-20260903-sidebar-git-preserves-native-branch-and-git-status-tokens-vyjo |
| 20260903-reload-config-tolerates-or-reports-a-preexisting-toml-parse-error-p1q1 | the plugin config writers and reload path handle a pre-existing TOML parse error by name instead of a silent reload-config refusal | open | herdr-mirror | - |
| 20260903-remote-worktree-remove-prunes-its-shadow-worktree-pwpe | remote-worktree-remove prunes the mirror's shadow worktree so no stale shadow markers remain after a remove | open | herdr-mirror | - |
| 20260903-shadow-repo-grouping-actually-engages-on-mirror-converge-fz6z | the shadow-repo grouping actually engages on mirror converge: mirror cwds become shadow worktrees, the shadow dir is populated, and rows group under one repo | done | herdr-mirror | - |
| 20260903-the-mirror-shows-and-acts-on-remote-worktrees-pt64 | worktree support in the mirror, plugin-encapsulated: remote worktree tokens and labels on mirror rows, and remote-worktree open, create and remove actions through the mirrored host | done | herdr-mirror | 20260903-mirror-workspace-rows-show-the-remote-git-branch-kj04 |
| 20260903-upstream-herdr-renders-plugin-supplied-branch-on-the-native-chip-rnwd | upstream herdr patch: the workspace chip falls back to plugin tokens branch, ahead and behind when cached git is None; fork, PR and issue | withdrawn | herdr-mirror | - |
| 20260903-upstream-pr-of-the-four-mirror-landings-0ijo | one upstream PR to nikok6/herdr-mirror from the fork: the four mirror landings squashed into reviewable commits, README limitation line rewritten, linked issue explaining the token approach | done | herdr-mirror | - |
