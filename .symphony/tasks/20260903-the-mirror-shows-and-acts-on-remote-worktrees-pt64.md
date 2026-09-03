---
id: "20260903-the-mirror-shows-and-acts-on-remote-worktrees-pt64"
schema: task
title: "worktree support in the mirror, plugin-encapsulated: remote worktree tokens and labels on mirror rows, and remote-worktree open, create and remove actions through the mirrored host"
status: open
section: herdr-mirror
tier: medium
blocked_by: []
paths: []
done_when: "Composer ask 2026-09-03 09:51 (conductor relay 542e2db6); herdr itself untouched, plugin-encapsulated; blocked_by kj04 and NOT seated until kj04 lands on the fork main. TWO PARTS. (1) SHOW: the remote herdr workspace list already carries worktree {checkout_path, repo_root, repo_name, is_linked_worktree}; forward them as tokens rrepo, rworktree, rcwd, rlinked on the mirror workspace and its agent rows via the existing report_metadata paths, and label mirror workspaces <host>: <repo>/<worktree> (fallback to the remote workspace label when it has no worktree); config default on, off is byte-for-byte today; README states plainly that native repo GROUPING in the sidebar cannot be had because herdr derives it from the local cwd, the same limit as the chip. (2) ACT: actions remote-worktree-open, remote-worktree-create, remote-worktree-remove that run herdr worktree ... on the mirrored host through the existing remote-invoke path, inheriting host and cwd from the focused mirror, with the new remote workspace mirroring back; outside a mirror each degrades to the local action like the other remote-* actions; a failure toasts by name. Failure modes named (D7 spirit): no worktree field gives label fallback and no tokens; ssh failure keeps or clears as the kj04 branch tokens do, consistently. TESTS: cargo tests for label and token derivation (with and without worktree, linked and main), action routing inside and outside a mirror, config off. LIVE PROOF: a treehouse lane workspace on dev-2 shows as threaded-dev-2: skald/<lane> here, and a worktree created from here appears on dev-2 and mirrors back; the conductor reads both after plugin-link and reload. Commit Conventional; symphony check at tip dirty false; lands on the fork main after kj04; PR to upstream with README. Report OUTCOME, VERIFICATION (the sidebar read-backs), NEXT STEP, PATHS, SHA."
accepted_survivors: []
declared_removals: []
created: "2026-09-03"
---
## History

- 2026-09-03 created
