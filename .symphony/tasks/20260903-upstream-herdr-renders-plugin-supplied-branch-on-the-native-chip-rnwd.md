---
id: "20260903-upstream-herdr-renders-plugin-supplied-branch-on-the-native-chip-rnwd"
schema: task
title: "upstream herdr patch: the workspace chip falls back to plugin tokens branch, ahead and behind when cached git is None; fork, PR and issue"
status: withdrawn
section: herdr-mirror
tier: low
blocked_by: []
paths: []
done_when: "Conductor 2026-09-03 09:43, from its scan of herdrdev/herdr; NOT SEATED until the composer decides whether to run a patched herdr. Deliverable is a fork plus PR plus an issue, not a running binary here. In herdr src/server/client_shell.rs:64-65 ClientShellWorkspace is built from the cached git state; when that cached git is None, fall back to the workspace metadata tokens branch, ahead and behind (as published by a plugin via workspace.report_metadata, the shape herdr-mirror kj04 part B emits as $rbranch, $rahead, $rbehind) so a plugin-supplied branch renders on the NATIVE workspace chip without a git cwd. Steps: fork herdrdev/herdr under tonetechnician, implement the fallback with cargo tests for cached-git-present (unchanged), cached-git-None with tokens (renders), cached-git-None without tokens (no chip, byte-for-byte today), open a PR referencing the herdr-mirror use, and an issue asking for it since herdr accepts PRs from approved contributors only; name the PR and issue URLs. Both machines stay on herdr 0.8.0 stable unless the composer rules otherwise. Report OUTCOME, VERIFICATION, NEXT STEP, PATHS, SHA."
accepted_survivors: []
declared_removals: []
created: "2026-09-03"
---
## History

- 2026-09-03 created
- 2026-09-03T07:48:27.477Z withdrawn by symphony-concertmaster: composer: herdr itself is not modified
