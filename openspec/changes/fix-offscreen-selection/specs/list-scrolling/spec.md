# Spec Delta

## ADDED Requirements

### Requirement: A visible selection stays visible across samples

The system SHALL keep the selected row in view after a sample re-sorts the list, when
that row was in view before the sample. It MUST NOT move the viewport when the selection
was already out of view.

#### Scenario: Selection sinks after a re-sort
- **WHEN** the selected row is visible and a sample moves it below the viewport
- **THEN** the viewport scrolls just enough to show it

#### Scenario: Pointer scrolled away
- **WHEN** the user scrolled the selection out of view and a sample arrives
- **THEN** the viewport stays where the user left it
