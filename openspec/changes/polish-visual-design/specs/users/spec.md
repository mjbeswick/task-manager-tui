## ADDED Requirements

### Requirement: Filter by user name

The tab SHALL filter sessions by a case-insensitive substring of the user name
and MUST restore every session when the filter is cleared.

#### Scenario: Filter
- **WHEN** the user types part of an account name into the filter field
- **THEN** only sessions for matching accounts remain, with the match highlighted

#### Scenario: No match
- **WHEN** the filter matches no session
- **THEN** an empty-state message is shown instead of a blank table

#### Scenario: Clear
- **WHEN** the user clears the filter
- **THEN** every session returns and the previous selection is restored if it still exists
