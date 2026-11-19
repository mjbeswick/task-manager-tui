# Tasks

## 1. Theme foundations

- [x] 1.1 Add `surface_alt` to both themes and a `Theme::row_style(selected, visible_index)` helper, and verify a unit test that selected rows ignore parity and adjacent rows differ
- [x] 1.2 Retune `HEAT_STEPS` to 1, 3, 8, 20, and 40 percent and move the top two heat entries toward orange and red, and verify the existing heat-step test plus updated step expectations
- [x] 1.3 Add `Tone` with `Status::tone()` and `ServiceState::tone()` plus `Theme::tone_style`, and verify a unit test maps Not responding to Danger, Suspended and Stopped to Warn, Running and Sleeping to Normal, and the two service states to Good and Muted
- [x] 1.4 Extend the contrast tests to text on `surface_alt` and every heat entry at truecolor, 256, and 16 colours, and verify they pass in both themes
- [x] 1.5 Verify 16-colour mapping collapses `surface_alt` to the same colour as `surface` and keeps the red heat step readable

## 2. Stripes in every table

- [x] 2.1 Replace the per-row base style in the Processes tab with `row_style`, and verify a render test that adjacent body rows differ and the selected row keeps the selection background
- [x] 2.2 Apply the same helper in Details, Services, Startup, Users, and App history, and verify a render test per tab finds alternating body backgrounds
- [x] 2.3 Verify headers, the sort rule, hints, and the empty state never use the stripe colour

## 3. Tab strip

- [x] 3.1 Prefix each tab with its digit from 1 to 7 and draw the active tab as a filled accent block, and verify a render test for the numbers and the block background
- [x] 3.2 Keep the compact strip readable below 90 columns, and verify the narrow render test still names the tabs

## 4. Status tones and documentation

- [x] 4.1 Colour the Processes status word through `Status::tone()`, leaving selected rows on the selection style, and verify a render test for Not responding and Running
- [x] 4.2 Colour the Services status word through `ServiceState::tone()`, and verify a render test that Running and Stopped use different foregrounds
- [x] 4.3 Add a short README note explaining the stripes, the heat escalation, the status colours, and the tab numbers, and verify the rendered markdown reads correctly in the key table section

## 5. Integration checks

- [x] 5.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-table-readability --strict`, and record the output

## Workflow follow-up

- Archive the change after its checks pass and the work is reviewed.
