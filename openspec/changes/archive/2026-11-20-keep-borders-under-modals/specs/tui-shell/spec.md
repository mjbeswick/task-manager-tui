# Spec Delta

## MODIFIED Requirements

### Requirement: Dialog model

The application SHALL present confirmations, errors, the run dialog, and help as
modal panels drawn over the window. A modal panel MUST capture all keys until it
closes, and opening one MUST dim text without changing the colours of borders, rules, or
frames.

#### Scenario: Confirmation
- **WHEN** an action needs confirmation
- **THEN** a panel names the target and the action, offers a default and a cancel choice, and stays open until one is chosen

#### Scenario: Error
- **WHEN** an action fails
- **THEN** a panel shows the failure reason and a hint about privilege, and the underlying table stays unchanged

#### Scenario: Dismiss
- **WHEN** the user presses Esc in any dialog
- **THEN** the dialog closes and no action is taken

#### Scenario: Dimming
- **WHEN** any dialog opens
- **THEN** text behind it turns muted and every box-drawing border, rule, and frame keeps its existing foreground colour
