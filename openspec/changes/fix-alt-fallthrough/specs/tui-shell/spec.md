# Spec Delta

## ADDED Requirements

### Requirement: Unbound Alt chords do nothing

The system SHALL ignore an Alt plus letter chord that opens no menu. Such a chord MUST NOT
trigger the action bound to the plain letter.

#### Scenario: Alt+E
- **WHEN** the user presses Alt+E on a process list
- **THEN** no process is signalled and nothing changes

#### Scenario: Alt+T
- **WHEN** the user presses Alt+T
- **THEN** the theme does not change and no list opens

#### Scenario: Menu chords
- **WHEN** the user presses Alt+F, Alt+O, Alt+V, Alt+P, or Alt+H
- **THEN** the matching menu opens
