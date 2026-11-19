# Design

## Context

- `Cli` already parses theme, interval, tab, colour depth, ascii, and once.
- `main` copies CLI overrides into `Settings` before `App::new`.
- `ChartStyle::parse`, `key`, and `ALL` already exist.

## Decisions

### Mirror the theme flag

Add `chart_style: Option<String>` with a `parse_chart_style` validator that accepts only
`ChartStyle::parse` values and stores the canonical key. `main` assigns the override to
`loaded.chart_style` before constructing the app.

The error list is built from `ChartStyle::ALL`, so a new style cannot leave it stale.

*Alternative.* Accept free text and let the app fall back silently. Rejected: a typo would
be ignored, which is how flags become unreliable.

## Risks / Trade-offs

- [Precedence] The flag overrides the saved setting but does not save itself → Same rule as
  `--theme`; a one-off run should not rewrite preferences.
