# Proposal

## Why

The active tab's outline is drawn in the accent colour while the rule it sits on is drawn
in the border colour. The two tones meet at the bottom corners and make the tab look
detached from the line.

## What Changes

- Draw the active tab's top border, sides, and bottom corners in the same colour as the
  horizontal rule.
- Keep the active label in the accent colour.
- No shape or geometry change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: the active tab outline uses the rule colour.

## Impact

- `src/ui/chrome.rs`: the box style.
- `README.md`: the tab colour wording.
