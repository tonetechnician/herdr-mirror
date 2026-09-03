---
id: "20260903-remote-worktree-remove-prunes-its-shadow-worktree-pwpe"
schema: task
title: "remote-worktree-remove prunes the mirror's shadow worktree so no stale shadow markers remain after a remove"
status: open
section: herdr-mirror
tier: low
blocked_by: []
paths: []
done_when: "CAVEAT from live acceptance h6fp (2026-09-03): after remote-worktree-remove correctly removed the mirror row here and the worktree on dev-2, the per-mirror SHADOW worktrees under ~/.local/state/herdr-mirror/shadow/<host>/<repo-hash>/worktrees/ were RETAINED (stale markers, populated, not pruned). Fix: when a mirror workspace goes away (remote-worktree-remove, or the remote worktree disappearing on converge), prune its linked shadow worktree (git worktree remove / prune the shadow entry) idempotently, so the shadow dir reflects only live mirrors; leave the bare shadow repo. TEST: a converge/remove that drops a mirror prunes exactly its shadow worktree and no live one. herdr untouched, plugin-encapsulated. Lands on fork main. symphony check green, dirty false, Conventional Commits. Report OUTCOME, VERIFICATION, NEXT STEP, PATHS, SHA."
accepted_survivors: []
declared_removals: []
created: "2026-09-03"
---
## History

- 2026-09-03 created
