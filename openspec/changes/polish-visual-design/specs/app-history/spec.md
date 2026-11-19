## ADDED Requirements

### Requirement: Filter by application name

The tab SHALL filter rows by a case-insensitive substring of the application name
and MUST restore every row when the filter is cleared.

#### Scenario: Filter
- **WHEN** the user types `code` into the filter field
- **THEN** only applications whose name contains `code` remain, with the match highlighted

#### Scenario: No match
- **WHEN** the filter matches no application
- **THEN** an empty-state message is shown instead of a blank table

#### Scenario: Totals ignore the filter
- **WHEN** a filter hides some rows
- **THEN** stored history is unchanged and every row returns when the filter is cleared
