# Spec Delta

## MODIFIED Requirements

### Requirement: Priority

The tab SHALL show and change process priority as a nice value from -20 to 19, seeded
with the current value, and MUST show the matching Windows 10 label for that value while
translating it to the platform's scheduling value.

#### Scenario: Change priority
- **WHEN** the user sets -10 on a process they own
- **THEN** the dialog shows High, the scheduling value changes, the row shows the new priority, and the change survives the next sample

#### Scenario: Precise value
- **WHEN** the user enters 7
- **THEN** the process receives a nice value of 7 and the dialog labels it Below normal

#### Scenario: Step the value
- **WHEN** the user presses Up, Down, Left, or Right
- **THEN** the value moves by one in that direction and stops at -20 or 19

#### Scenario: Page step
- **WHEN** the user presses PgUp or PgDn
- **THEN** the value moves by five and stops at the ends of the range

#### Scenario: Out of range
- **WHEN** the user enters a value above 19 or below -20
- **THEN** the dialog clamps it to the nearest end of the range before applying

#### Scenario: Empty value
- **WHEN** the field is empty and the user confirms
- **THEN** nothing is sent and the dialog reports that a value is required

#### Scenario: Reset
- **WHEN** the user sets 0
- **THEN** the process returns to the default scheduling value

#### Scenario: Not permitted
- **WHEN** the priority cannot be changed for the selected process
- **THEN** a dialog explains the permission requirement and the value is unchanged
