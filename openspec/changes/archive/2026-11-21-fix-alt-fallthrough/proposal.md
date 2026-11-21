# Proposal

## Why

An Alt chord with no binding falls through to its plain key. Alt+T cycles the theme and
Alt+E ends the selected task. A user reaching for a menu can end a process by mistake.

## What Changes

- An Alt chord that opens no menu does nothing.
- Alt+F, Alt+O, Alt+V, Alt+P, and Alt+H still open their menus.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tui-shell`: unbound Alt chords are ignored.

## Impact

- `src/input.rs`: the Alt branch of `translate` and its test.
