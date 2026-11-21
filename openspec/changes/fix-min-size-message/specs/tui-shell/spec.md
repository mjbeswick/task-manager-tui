# Spec Delta

## ADDED Requirements

### Requirement: The too-small message is readable

The message shown below the minimum size SHALL wrap to the terminal width so that the
required size and the current size are both visible.

#### Scenario: One column short
- **WHEN** the terminal is 39 columns by 12 rows
- **THEN** the message shows 40 columns, 12 rows, and 39x12 in full
