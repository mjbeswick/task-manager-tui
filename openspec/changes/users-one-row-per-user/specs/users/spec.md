## MODIFIED Requirements

### Requirement: User columns

The tab SHALL show one row per user with User, Session, Status, CPU, Memory, and
Disk, and MUST mark the user running this application. The Session column MUST name
the user's first session and count any others.

#### Scenario: Values
- **WHEN** a row is shown
- **THEN** the Session column names the tty or console, and CPU and memory are the sum of that user's processes, formatted like the Processes tab

#### Scenario: Several sessions
- **WHEN** one user has four terminal sessions
- **THEN** the tab shows one row for that user, its totals appear once, and the Session column reads like `pts/0 +3`

#### Scenario: Current session
- **WHEN** the tab lists the user this application runs as
- **THEN** that row is marked as the current session

#### Scenario: Root
- **WHEN** a session belongs to the root user
- **THEN** the row shows it as a system session with its resources

### Requirement: Session status

The Status column SHALL distinguish an active user from a disconnected or idle one
using the system's own session state.

#### Scenario: Active
- **WHEN** any of a user's sessions has a live terminal or graphical login
- **THEN** that user's status reads Active

#### Scenario: Disconnected
- **WHEN** the user's sessions exist but none has an attached terminal
- **THEN** its status reads Disconnected
