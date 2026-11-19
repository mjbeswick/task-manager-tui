## ADDED Requirements

### Requirement: Resource cells are shaded by load

The tab SHALL shade CPU, Memory, Disk, and Network cells by the row's share of the
system total, in five steps from a pale tint to a deep tint. A higher share MUST
never get a lighter step than a lower share. Text on every step MUST keep 4.5:1
contrast.

#### Scenario: Busy process
- **WHEN** a process uses 40% of the CPU
- **THEN** its CPU cell is drawn in a deeper step than a process using 2%

#### Scenario: Idle value
- **WHEN** a cell's share is below the first step
- **THEN** the cell keeps the plain row background

#### Scenario: Unavailable metric
- **WHEN** a cell shows a dash
- **THEN** the cell is never shaded

#### Scenario: Selected row
- **WHEN** a shaded row is selected
- **THEN** the selection background replaces the shading across the whole row

#### Scenario: Group rows
- **WHEN** a group heading row is drawn
- **THEN** its cells are not shaded

### Requirement: Header shows system totals

The column header SHALL show the system total above each resource label, as a
percentage for CPU and Memory and as a rate for Disk and Network.

#### Scenario: Totals line
- **WHEN** the tab is drawn with a sample
- **THEN** the header shows `27%` above `CPU` when total CPU is 27 percent

#### Scenario: Unknown total
- **WHEN** a total cannot be read on this platform
- **THEN** a dash is shown above that label

#### Scenario: Short terminal
- **WHEN** the content area is shorter than twelve rows
- **THEN** the totals line is dropped and the header takes one row

## MODIFIED Requirements

### Requirement: Filter by name

The tab SHALL filter rows by a case-insensitive substring of the process name, SHALL
show the active filter, and MUST restore all rows when the filter is cleared.

#### Scenario: Filter
- **WHEN** the user types a filter string
- **THEN** only matching rows and their groups remain visible, the filter field shows the text, and the match in each name is highlighted

#### Scenario: No match
- **WHEN** the filter matches nothing
- **THEN** the table shows an empty-state message rather than a blank panel

#### Scenario: Clear
- **WHEN** the user clears the filter
- **THEN** every row returns and the previous selection is restored if it still exists
