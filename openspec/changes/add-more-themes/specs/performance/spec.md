# Spec Delta

## ADDED Requirements

### Requirement: Per-resource chart colours come from the theme

The Per resource chart colour SHALL take five distinct colours from the active theme, one
per resource, and MUST update when the theme changes.

#### Scenario: Distinct resources
- **WHEN** Per resource is active
- **THEN** CPU, Memory, Disk, Network, and GPU each draw with a different colour from the palette

#### Scenario: Theme change
- **WHEN** the user switches theme while Per resource is active
- **THEN** every chart adopts the new theme's five resource colours immediately

#### Scenario: Visibility
- **WHEN** any theme is active
- **THEN** each resource colour stays visible against that theme's chart surface

#### Scenario: Other chart colours
- **WHEN** the user chooses Teal, Blue, Orange, Magenta, Mono, or Theme
- **THEN** the fixed colour still overrides the per-resource palette
