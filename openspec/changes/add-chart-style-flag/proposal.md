# Proposal

## Why

The Blocks chart style is selectable only from the Appearance dialog. Scripts, smoke
tests, and users who start the app with flags cannot choose a chart style for the run,
even though theme, speed, colour depth, and tab all have flags.

## What Changes

- Add `--chart-style filled|line|dots|blocks`.
- The flag overrides the saved chart style for that run, like `--theme`.
- The error message lists every style key from the style table.
- F2 and the saved setting behave as before.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance`: chart style can be chosen from the command line as well as Appearance.

## Impact

- `src/cli.rs`: flag, parser, tests.
- `src/main.rs`: apply the override before `App::new`.
- `README.md`: flags table.
