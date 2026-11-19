# Spec Delta

## ADDED Requirements

### Requirement: Stable status bar layout

The status bar SHALL give each metric a fixed-width field and keep the metric block
anchored at the right edge, so changing values MUST NOT move any other part of the bar.

#### Scenario: Value grows a digit
- **WHEN** a metric crosses from 9.9% to 10.0% or from 999 to 1.0 of a larger unit
- **THEN** the metric block keeps the same width and every other element stays in place

#### Scenario: Position counter
- **WHEN** the selected row changes or the list length gains a digit
- **THEN** the counter keeps a fixed field width and the action label does not move

#### Scenario: Notice
- **WHEN** a notice is shown
- **THEN** it is truncated inside the middle zone and neither the action nor the metrics move

#### Scenario: Narrow terminal
- **WHEN** the terminal is too narrow for every metric
- **THEN** metrics are dropped in a fixed order using their widest possible widths, so the set does not change between samples
