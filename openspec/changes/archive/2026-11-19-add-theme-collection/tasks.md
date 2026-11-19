# Tasks

## 1. Theme data and contrast

- [x] 1.1 Replace `ThemeName` with a six-variant list and derive `label`, `key`, `parse`, `next`, and `previous` from `ThemeName::ALL`, and verify unit tests cover every key and the wrapping order
- [x] 1.2 Write the six palettes with every role, including `chrome`, `status`, `surface_alt`, and `on_accent`, and verify `Theme::from_name` returns a complete theme for each name
- [x] 1.3 Replace every hardcoded `White` on an accent fill with `on_accent`, and verify a search finds no remaining `Color::White` in the UI
- [x] 1.4 Key the 16-colour mapping off background luminance instead of the theme name, and verify the existing reduced-depth test passes for all six themes
- [x] 1.5 Extend the contrast suite to every theme in `ALL` for text, muted, selection, chrome, status, and every heat step at all three depths

## 2. Picker and persistence

- [x] 2.1 Make `t`, the Appearance dialog, and the Options menu cycle through all six themes, and verify action tests advance from Light to Dark and back from Light
- [x] 2.2 Parse `--theme` through the theme list and reject unknown values with all six keys, and verify CLI tests for a valid key and a typo
- [x] 2.3 Keep the settings string form, fall back to Light for an unknown saved value, and verify the fallback notice and the persisted key

## 3. Chrome and status bands

- [x] 3.1 Fill the menu row and tab strip with `chrome`, keep the filter field on a `surface` inset, and verify a render test checks the three backgrounds
- [x] 3.2 Fill the status bar with `status`, and verify its background differs from chrome and surface in both themes
- [x] 3.3 Move the `Task Manager` label into the chrome band for every tab, and verify a render test finds it on Processes as well as Performance
- [x] 3.4 Set the terminal title to `Task Manager - <tab>` at startup and on tab changes, and verify by running the binary and switching tabs

## 4. Documentation and integration checks

- [x] 4.1 Add the theme keys to the in-app help and the README theme section, and verify the help panel still fits at 120 columns
- [x] 4.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate add-theme-collection --strict`, and record the output

## Workflow follow-up

- Archive the change after its checks pass and the work is reviewed.
