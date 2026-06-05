# ternary-pilgrim: Journey patterns and pilgrimage routes through fleet rooms

Models how agents develop habitual traversal patterns — recurring routes between rooms, important waypoints, journey logging, and credential-based room access.

## Why This Exists

Agents don't move randomly. Over time they develop habitual routes: every morning they check room A, then B, then C. Some rooms are important waypoints ("shrines") that every agent visits. Some routes are sacred — mandatory sequences that must be followed in order. And crossing room boundaries requires credentials. This crate gives you the vocabulary to model all of that.

## Core Concepts

- **Pilgrim**: An agent's repeating journey — a list of waypoints, a repeat interval, and a completion counter.
- **Pilgrimage**: A named sacred route with mandatory waypoints. A journey "satisfies" a pilgrimage if it visits all mandatory stops in order.
- **Shrine**: An important destination with a significance level (Minor, Major, Sacred).
- **PilgrimLog**: A record of all movements across all agents. Query most-visited rooms, trip counts, per-agent histories.
- **PilgrimGuide**: A navigation assistant. Register named routes and get next/previous room, steps remaining, and route-finding.
- **PilgrimPassport**: An agent's room access credentials. Rooms are "stamped" (granted) or "denied." Stamps override denials.

## Quick Start

```toml
[dependencies]
ternary-pilgrim = "0.1"
```

```rust
use ternary_pilgrim::*;

let mut pilgrim = Pilgrim::new("agent-7", 100);
pilgrim.add_waypoint("engine-room");
pilgrim.add_waypoint("bridge");
pilgrim.add_waypoint("engine-room");
assert!(pilgrim.is_round_trip());

let mut guide = PilgrimGuide::new();
guide.register_route("main-corridor", vec![
    "lobby".to_string(), "hall-a".to_string(), "hall-b".to_string(),
]);
assert_eq!(guide.next_room("main-corridor", "lobby"), Some("hall-a"));

let mut passport = PilgrimPassport::new("agent-7");
passport.stamp("engine-room");
passport.stamp("bridge");
assert!(passport.can_enter("engine-room"));
```

## API Overview

| Type | Description |
|------|-------------|
| `Pilgrim` | A repeating agent journey with waypoints and completion tracking |
| `Pilgrimage` | A sacred route with mandatory waypoints; validates journey compliance |
| `Shrine` | A named important destination with significance level |
| `PilgrimLog` | Movement history with most-visited and trip-count queries |
| `PilgrimGuide` | Named route registry with navigation helpers (next, prev, find) |
| `PilgrimPassport` | Room access credentials with stamp/deny/revoke |
| `Waypoint` | A single stop: room name, order, optional label |

## How It Works

`Pilgrim` stores waypoints as a `Vec<Waypoint>` in order. Routes are just the room names extracted from waypoints. Round-trip detection compares first and last room names — simple but sufficient.

`Pilgrimage` separates mandatory from optional waypoints using a `HashSet`. Satisfaction checking scans the visited rooms list in order, advancing through mandatory waypoints when each is found. This means mandatory waypoints must appear in the visited list in order, but optional rooms can appear between them.

`PilgrimLog` is a flat `Vec<LogEntry>` with linear-scan queries. For fleets with millions of movements, you'd want indexing. For normal use (thousands of movements), it's fine.

`PilgrimGuide` stores named routes in a `HashMap`. Navigation is positional: find the current room's index, return the adjacent one. Route finding checks if two rooms appear on the same named route in the right order — no pathfinding, just direct route matching.

`PilgrimPassport` uses two `HashSet`s (stamps and denials). A stamp overrides a denial for the same room. Revoke removes from both sets.

## Known Limitations

- **No pathfinding**: PilgrimGuide only knows registered routes. It can't compute new paths between rooms.
- **Linear log queries**: Most-visited and trip-count queries scan all entries. Not indexed.
- **No time-based analysis**: PilgrimLog records ticks but doesn't provide time-windowed queries (e.g., "movements in the last hour").
- **No journey matching**: Can't automatically detect that an agent's movements match a known pilgrimage — you must check manually with `is_satisfied_by`.
- **Binary passport**: Rooms are either accessible or not. No role-based or conditional access.

## Use Cases

- **Agent routine modeling**: Track which rooms agents visit regularly and detect when routines change.
- **Route compliance**: Verify that agents followed mandatory procedures (pilgrimage satisfaction).
- **Capacity planning**: Use PilgrimLog to find which rooms get the most traffic.
- **Access control**: PilgrimPassport manages which agents can enter which rooms.
- **Navigation assistance**: PilgrimGuide helps new agents learn established routes.

## Ecosystem Context

Part of the SuperInstance ternary fleet. Related crates:
- `ternary-room`: The rooms that pilgrims traverse.
- `ternary-registry-v2`: Skill dependencies may affect which rooms an agent needs to visit.
- `ternary-navigator`: More general pathfinding (vs. pilgrim's fixed routes).

## License

MIT
