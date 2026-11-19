# Proposal

## Why

An open menu stays open when the pointer clicks somewhere else, so the click either
does nothing visible or acts underneath a menu that still covers the window. The menu
bar and tab strip also share one background, which makes the two bands read as a single
header, and the filled active-tab block is heavier than the rest of the chrome.

## What Changes

- A left click outside an open menu closes it, and the click is consumed so it cannot
  activate the row, tab, header, or field underneath.
- A click on the dropdown itself, including its border, keeps the menu open.
- The menu strip keeps a solid background and its own inset filter field.
- The tab strip loses its background and sits on the window background.
- A one-row rule appears under the tab strip. Each tab gets a centered line segment in
  the border colour, and the active tab's segment uses the accent colour.
- The active tab is no longer a filled block; its label uses the accent colour only.
- The minimum terminal height stays 12 rows; the new rule takes one row from the content.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the window layout gains the tab rule row and a menu-bar background, the
  tab strip distinguishes itself with a centred accent segment, and mouse support closes
  an open menu on a click away.

## Impact

- `src/ui/chrome.rs`: menu and tab backgrounds, the rule row, underline segments.
- `src/ui/hits.rs`: a dropdown-panel target so clicks inside the menu are distinguishable.
- `src/main.rs`: click-away dismissal before the normal click path.
- `src/ui/mod.rs` or `chrome.rs`: one extra layout row.
- `README.md` and the help dialog: the new click-away behaviour.
- Coordination: `add-theme-collection` and `add-mouse-scrolling` are applied but not
  archived. This change archives after them so the final tab-strip wording wins.
