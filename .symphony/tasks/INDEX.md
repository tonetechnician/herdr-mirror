# tasks

Derived from the documents in this directory; do not edit by hand. `symphony fmt` rewrites it.

| id | title | status | section | blocked_by |
| --- | --- | --- | --- | --- |
| 20260903-20260903-sidebar-git-preserves-native-branch-and-git-status-tokens-vyjo | sidebar-git preserves herdr's native branch and git_status tokens: append $rgit to the last row rather than replacing the defaults | open | herdr-mirror | - |
| 20260903-mirror-workspace-rows-show-the-remote-git-branch-kj04 | mirror workspace rows show the remote git branch via a local shadow repo, plus forwarded rbranch, rahead and rbehind tokens; PR to upstream | done | herdr-mirror | - |
| 20260903-new-worktree-works-on-mirrored-repos-like-herdr-local-3o7j | New Worktree works on mirrored repos exactly like herdr local: intercept the native new_worktree flow (prefix+shift+g) so it creates the worktree on the mirrored host | open | herdr-mirror | 20260903-20260903-sidebar-git-preserves-native-branch-and-git-status-tokens-vyjo |
| 20260903-the-mirror-shows-and-acts-on-remote-worktrees-pt64 | worktree support in the mirror, plugin-encapsulated: remote worktree tokens and labels on mirror rows, and remote-worktree open, create and remove actions through the mirrored host | done | herdr-mirror | 20260903-mirror-workspace-rows-show-the-remote-git-branch-kj04 |
| 20260903-upstream-herdr-renders-plugin-supplied-branch-on-the-native-chip-rnwd | upstream herdr patch: the workspace chip falls back to plugin tokens branch, ahead and behind when cached git is None; fork, PR and issue | withdrawn | herdr-mirror | - |
