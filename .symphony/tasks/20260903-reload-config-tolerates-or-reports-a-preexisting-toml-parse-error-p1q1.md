---
id: "20260903-reload-config-tolerates-or-reports-a-preexisting-toml-parse-error-p1q1"
schema: task
title: the plugin config writers and reload path handle a pre-existing TOML parse error by name instead of a silent reload-config refusal
status: open
section: herdr-mirror
tier: low
blocked_by: []
paths: []
done_when: "CAVEAT from live acceptance h6fp (2026-09-03): 'herdr server reload-config' refused because of a PRE-EXISTING TOML parse error at line 21 of the herdr config; the plugin manifest/action and daemon still operated, but a config reload is blocked until that line is fixed. DIAGNOSE BY SCAN: which config file and line 21 (is it a user hand-edit, or did a plugin config writer — sidebar-git --write / worktree-keys --write — emit a line that herdr's parser rejects?). If a plugin writer produced it, FIX the writer to emit parseable TOML and add a test pinning the written block parses; if it is a user/pre-existing edit, DOCUMENT that reload-config requires a valid config and have the writers refuse-by-name rather than corrupt an already-broken file. Report which it is. herdr untouched, plugin-encapsulated. Lands on fork main if code changes. symphony check green, dirty false, Conventional Commits. Report OUTCOME, VERIFICATION, NEXT STEP, PATHS, SHA."
accepted_survivors: []
declared_removals: []
created: "2026-09-03"
---
## History

- 2026-09-03 created
