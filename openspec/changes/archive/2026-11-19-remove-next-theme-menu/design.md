# Design

## Context

- `MENUS` in `app.rs` defines the Options items, including `("Next theme", MenuItem::Theme)`.
- `menu_item` maps `MenuItem::Theme` to `Action::ToggleTheme`.
- `t` maps to the same action, and the Appearance dialog changes the theme row directly.

## Decisions

### Delete the menu entry and the variant

Remove the tuple from the Options list, delete `MenuItem::Theme`, and delete its dispatch
arm. No other menu item uses the variant.

*Alternative.* Keep the item but hide it. Rejected: a hidden enum variant is dead code and
a later reader has to work out why.

## Risks / Trade-offs

- [Discoverability] The menu no longer advertises theme switching → The help dialog lists
  `t`, and the Appearance dialog is reachable from the View menu and F2.
