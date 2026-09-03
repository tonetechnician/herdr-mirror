---
id: "20260903-20260903-sidebar-git-preserves-native-branch-and-git-status-tokens-vyjo"
schema: task
title: "sidebar-git preserves herdr's native branch and git_status tokens: append $rgit to the last row rather than replacing the defaults"
status: done
section: herdr-mirror
tier: low
blocked_by: []
paths: []
done_when: "DEFECT (composer saw it): kj04's sidebar-git wrote [ui.sidebar.spaces] rows that REPLACED herdr's defaults and dropped the native branch and git_status tokens, so every LOCAL workspace lost its git chip. herdr default is rows = [[state_icon, workspace], [branch, git_status]] (src/config/sidebar.rs:415-421). FIX: sidebar-git prints and WRITES [[state_icon, workspace], [branch, git_status, $rgit]] — native chip first, $rgit appended (empty on locals, filled on mirrors). When an existing [ui.sidebar.spaces] is already present, APPEND $rgit to its LAST row instead of refusing. README updated to show the corrected row. TEST pins that the written rows contain both branch and git_status (guard both sides: a row missing them fails). herdr untouched, plugin-encapsulated; config off byte-for-byte today. symphony check green at tip, dirty false, Conventional Commits, rebase onto fork main (168a16f) before reporting done. The conductor hand-fixed the composer config to the correct row and reloaded; the plugin must now produce it itself. Report OUTCOME, VERIFICATION, NEXT STEP, PATHS, SHA."
accepted_survivors: []
declared_removals: []
created: "2026-09-03"
---
## History

- 2026-09-03 created
- 2026-09-03T08:58:38.199Z landed task/20260903-20260903-sidebar-git-preserves-native-branch-and-git-status-tokens-vyjo on main as fe308afb7911f7dc087341b779c63a41492bc392
