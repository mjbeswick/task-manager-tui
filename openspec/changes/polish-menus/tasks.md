# Tasks

## 1. Menu data

- [x] 1.1 Replace the menu tuples with `MenuDef` entries carrying a shortcut and a separator flag, regroup File, Options, and View, and verify a test for the six menu layouts
- [x] 1.2 Add checked and disabled-state derivation for every item, and verify tests for speed, toggles, and the no-process case

## 2. Menu rendering

- [x] 2.1 Draw the state marker, right-aligned shortcut or reason, separators, and muted disabled items, and verify a render test at 120 and 40 columns
- [x] 2.2 Refuse activation of a disabled item from the keyboard and the mouse, and verify the menu stays open

## 3. Shortcut context

- [x] 3.1 Scope `s`, `S`, and `R` to the Services tab, and verify they change nothing on Processes
- [x] 3.2 Move accelerator handling behind dialog, menu, and filter focus, block Ctrl and Alt on plain letters, and accept `Ctrl+F` for the filter, and verify input tests
- [x] 3.3 Close the open menu when its own label is clicked, and verify the second click closes it

## 4. Help

- [x] 4.1 Add a scroll offset to the help dialog with Up, Down, PgUp, and PgDn, group the keys, and verify every row is reachable at 40x12

## 5. Hints and documentation

- [x] 5.1 Add End tree to the Processes and Details hint rows, and update the help list and README for the new menus and keys
- [x] 5.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release`, and `openspec validate polish-menus --strict`, and record the output

## Workflow follow-up

- Archive this change after its checks pass and the work is reviewed.
