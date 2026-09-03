---
id: "20260903-shadow-repo-grouping-actually-engages-on-mirror-converge-fz6z"
schema: task
title: "the shadow-repo grouping actually engages on mirror converge: mirror cwds become shadow worktrees, the shadow dir is populated, and rows group under one repo"
status: open
section: herdr-mirror
tier: medium
blocked_by: []
paths: []
done_when: "DEFECT on landed oig1 (eda7dda), conductor live read 2026-09-03 16:08 (relay 68ea5b7e): native grouping is NOT engaging. SYMPTOMS on eda7dda after rebuild+daemon restart: both mirrors (threaded-dev-2: skald/main, skald/skald) were recreated but their pane cwd is STILL ~/.local/state/herdr-mirror/.mirror-pane (the marker), the local workspaces carry worktree=null, ~/.local/state/herdr-mirror/shadow is EMPTY, hosts.toml has no shadow/group key, and the README documents neither. DIAGNOSE BY SCAN first: why does the shadow path never run? Candidates — (a) it is gated by a config key that defaults OFF and the README does not document (the card required shadow_repo default ON); (b) it is gated by worktree_metadata being present; (c) the shadow-creation code only runs on a path that fires before the remote connection/first converge, so the mirror cwd is set to the marker before the shadow worktree is ever created. Establish the real gate from the code, then FIX so that on each converge a mirror workspace with a remote worktree gets its cwd pointed at a linked worktree of the per-(host,repo) shadow repo (created/populated under ~/.local/state/herdr-mirror/shadow/<host>/<repo-hash>/), idempotently, or DOCUMENT the required config if it is intentional and default it ON per the card. ADD a status line that names the shadow repo per mirror so the state is observable. TESTS: cargo tests pinning that converge on a worktree-bearing mirror creates/points the shadow worktree (not the marker), config default on, idempotence. herdr untouched, plugin-encapsulated. Lands on fork main (currently eda7dda). Rebase before reporting done. symphony check green at tip, dirty false, Conventional Commits. LIVE PROOF is the conductor's: the skald rows group under one repo with the native chip and the shadow dir is populated. Report OUTCOME, VERIFICATION, NEXT STEP, PATHS, SHA."
accepted_survivors: []
declared_removals: []
created: "2026-09-03"
---
## History

- 2026-09-03 created
