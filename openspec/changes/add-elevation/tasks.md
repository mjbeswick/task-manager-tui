# Tasks

## 1. Platform

- [x] 1.1 Add `ElevatedAction` and command construction for system services and signal lists, and verify argv tests for both platforms
- [x] 1.2 Add the macOS osascript runner and the Linux pkexec and sudo runners, and verify failures map to a password request or a message

## 2. Interface

- [x] 2.1 Add the elevated effects and route system services through a confirmation that emits them
- [x] 2.2 Route privileged signals and process trees through the elevated signal path
- [x] 2.3 Add the masked password dialog and its input, and verify it closes on Enter or Esc and never stores the value
- [x] 2.4 Handle a password request by opening the dialog and failures by showing the platform message

## 3. Documentation and checks

- [x] 3.1 Update help and README for elevation, and verify the wording
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-elevation --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
