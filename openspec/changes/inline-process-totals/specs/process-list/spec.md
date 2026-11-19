# Spec Delta

## ADDED Requirements

### Requirement: System totals on the Apps row

The Processes tab SHALL show the system CPU, memory, disk, network, and GPU totals in the
corresponding numeric columns of the Apps heading row, right-aligned beneath the column
they summarise, and MUST omit them when that row is not visible.

#### Scenario: Apps visible
- **WHEN** the Apps heading is the first visible row
- **THEN** the totals appear on that row, aligned under CPU, Memory, Disk, Network, and GPU, and the heading text stays readable

#### Scenario: Scrolled away
- **WHEN** the viewport no longer includes the Apps heading
- **THEN** no totals are drawn on any process row

#### Scenario: Filtered out
- **WHEN** the filter hides the Apps heading
- **THEN** no totals are drawn

#### Scenario: Dropped columns
- **WHEN** a narrow terminal drops optional columns
- **THEN** totals are drawn only for the columns that remain

#### Scenario: Live values
- **WHEN** a new sample arrives while the Apps row is visible
- **THEN** the totals update in place without moving the heading text

#### Scenario: Header height
- **WHEN** the terminal is tall
- **THEN** the column header still uses one row and the table starts one row higher than before
