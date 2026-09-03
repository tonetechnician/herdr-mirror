---
id: "20260903-live-acceptance-mirror-worktrees-end-to-end-h6fp"
schema: task
title: "live acceptance: mirror groups worktrees end to end on the running build — rebuild to d3fcbd4, group read-back, remote worktree create then remove, read-backs only"
status: open
section: herdr-mirror
tier: low
blocked_by: []
paths: []
done_when: "COMPOSER ASK 2026-09-03 16:26 (conductor relay 81121bbe): verify the mirror WORKS WITH WORKTREES end to end, LIVE, now that dev-2 is back (its Skald lane worktree is being removed and the tree returned, so start clean). This is a LIVE acceptance — report READ-BACKS from the actual sidebar/herdr state, NOT exit codes. NOTE: the grouping engage-fix already LANDED on fork main at d3fcbd4; the running daemon here is the older eda7dda, so FIRST rebuild the local herdr plugin from fork main d3fcbd4 and restart the daemon (establish by scan how the plugin is built/installed here — plugin-link to this checkout or the fork clone, cargo build --release, restart the daemon; do not change herdr itself). STEPS: (1) confirm the skald/main mirror now GROUPS under one repo with the native branch chip and that ~/.local/state/herdr-mirror/shadow/<host>/<repo-hash>/ is POPULATED (not the marker cwd, not empty) — read it back; if it still does not group, STOP and report the exact state (cwd, worktree field, shadow dir contents) as a defect. (2) create a worktree on dev-2 through the plugin path (remote-worktree-create from the mirror, or as the remote half: ssh threaded-dev-2 herdr worktree create <name> in skald) and READ BACK that it mirrors INSIDE the group with its own branch chip within one poll. (3) remove it through remote-worktree-remove and READ BACK that the mirror row is gone here AND the dev-2 worktree is gone. (4) leave prefix+shift+g for the composer to press himself — do not press it. Report each read-back verbatim (sidebar rows, chips, shadow dir listing, dev-2 worktree list before/after). No code landing is expected; if a defect is found, report it precisely for a follow-up card. Report OUTCOME, VERIFICATION (the read-backs), NEXT STEP, PATHS, SHA of the build you ran."
accepted_survivors: []
declared_removals: []
created: "2026-09-03"
---
## History

- 2026-09-03 created
