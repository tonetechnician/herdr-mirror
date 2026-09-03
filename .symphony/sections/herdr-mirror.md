---
id: herdr-mirror
schema: section
repo: "/Users/threaded-dev-1/git-repos/herdr-mirror"
default_harness: claude-code
check: "cargo build --release && cargo test"
store_branch: symphony/store
trunk: main
deps: []
artifacts:
  - target
---
## History

- 2026-09-03 bound to /Users/threaded-dev-1/git-repos/herdr-mirror
- 2026-09-03 artifacts: none -> "/target"
- 2026-09-03 artifacts: "/target" -> "target"
