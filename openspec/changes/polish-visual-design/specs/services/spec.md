## MODIFIED Requirements

### Requirement: Filter and refresh

The tab SHALL filter by service name and description, SHALL refresh with the sample
cadence, and MUST reflect changes made outside the application after a refresh.

#### Scenario: Filter
- **WHEN** the user types a filter
- **THEN** only matching services remain and the filter field shows the text

#### Scenario: External change
- **WHEN** a service is started outside the application
- **THEN** the next refresh shows it Running with its PID

#### Scenario: No match
- **WHEN** the filter matches no service
- **THEN** an empty-state message is shown instead of a blank table
