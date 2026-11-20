# Design

## Context

- `MENUS` is `[(&str, &[(&str, MenuItem)]); 6]`; the dropdown draws labels only.
- `selected_service()` ignores the active tab, so `s`/`S`/`R` can act on a hidden row.
- Alt accelerators are checked before dialogs and filter editing.
- `Dialog::Help` is a unit variant and the panel clips 25 rows on a short terminal.

## Decisions

### One table with per-item metadata

`MenuDef { label, item, shortcut, separator_before }` replaces the tuple, and `MENUS`
stays a fixed array. Separators are flags rather than extra entries, so the highlighted
index still maps one-to-one to an item and arrows need no skipping.

### State and availability are derived, not stored

`MenuItem::checked(app)` returns Some for radio and check items and None otherwise.
`MenuItem::disabled(app)` returns the short reason when the item cannot act on the current
tab. Both read existing `App` state, so a menu cannot drift from the tab underneath it.

Activation checks `disabled` first: the menu stays open and nothing runs. This also covers
mouse clicks, because they dispatch the same activation path.

### Width from both columns

The dropdown width takes the widest `marker + label + gap + right column`, where the right
column is the shortcut or the disabled reason. It is clamped to the terminal as today, so
40 columns still works.

### Key context is resolved in one order

Dialog, menu, and filter take precedence over accelerators. After that, Ctrl maps only to
`d`, `u`, and `f`; any other Ctrl or Alt combination returns nothing. This removes
`Ctrl+S` stopping a service and keeps Alt from leaking into text fields.

### Help scrolls instead of shrinking

`Dialog::Help { offset }` holds the first visible row. Up and Down move one row, PgUp and
PgDn move ten, and the offset clamps to the last page. Sections are rows in the same list,
so grouping costs no extra layout code.

## Risks / Trade-offs

- [Menu width] Long labels plus reasons can crowd a narrow dropdown → The panel clamps and
  the label clips, as the bar already does.
- [Disabled items] Arrowing still stops on them → They stay visible with their reason,
  which explains why the action is unavailable.
- [Help offset] Resizing while scrolled can leave the offset past the end → The draw clamps
  it every frame.
