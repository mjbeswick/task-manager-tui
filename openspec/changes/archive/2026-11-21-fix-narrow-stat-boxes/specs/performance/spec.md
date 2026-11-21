# Spec Delta

## ADDED Requirements

### Requirement: Stat boxes always show their values

Every stat box on a resource panel SHALL show its value under its label. The stat area
MUST be tall enough for the number of box rows its width produces. When the panel is too
short for a row of boxes, that row MUST be left out rather than drawn as labels alone.

#### Scenario: Narrow CPU panel
- **WHEN** the CPU panel is shown at 40x16 or 60x20
- **THEN** Utilization, Speed, Processes, Threads, Handles, and Up time each show a value

#### Scenario: Narrow Memory panel
- **WHEN** the Memory panel is shown at 40x16
- **THEN** In use, Available, Total, Cached, Compressed, and Swap used each show a value

#### Scenario: Too short for a row
- **WHEN** the panel cannot fit a row of stat boxes with their values
- **THEN** that row is omitted and no label appears without its value
