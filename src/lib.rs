#![forbid(unsafe_code)]

//! Journey patterns and pilgrimage routes through fleet rooms.
//!
//! Models how agents develop habitual traversal patterns — recurring routes
//! between rooms, important waypoints, journey logging, and credential-based
//! room crossing. Named after the idea that agents develop "pilgrimages":
//! regular routes they follow, with shrines (important waypoints) along the way.

use std::collections::HashMap;
use std::collections::HashSet;

/// A single waypoint in a journey.
#[derive(Clone, Debug, PartialEq)]
pub struct Waypoint {
    pub room: String,
    /// Order in the journey (0-based).
    pub order: usize,
    /// Optional label for this stop.
    pub label: Option<String>,
}

impl Waypoint {
    pub fn new(room: &str, order: usize) -> Self {
        Self {
            room: room.to_string(),
            order,
            label: None,
        }
    }

    pub fn labeled(room: &str, order: usize, label: &str) -> Self {
        Self {
            room: room.to_string(),
            order,
            label: Some(label.to_string()),
        }
    }
}

/// A repeating journey that an agent makes.
#[derive(Clone, Debug)]
pub struct Pilgrim {
    pub agent: String,
    /// Ordered list of waypoints.
    waypoints: Vec<Waypoint>,
    /// How often this journey repeats (in ticks, 0 = one-time).
    pub repeat_interval: u64,
    /// Total number of completed journeys.
    pub completions: u64,
}

impl Pilgrim {
    pub fn new(agent: &str, repeat_interval: u64) -> Self {
        Self {
            agent: agent.to_string(),
            waypoints: Vec::new(),
            repeat_interval,
            completions: 0,
        }
    }

    /// Add a waypoint to the journey (appended at end).
    pub fn add_waypoint(&mut self, room: &str) {
        let order = self.waypoints.len();
        self.waypoints.push(Waypoint::new(room, order));
    }

    /// Add a labeled waypoint.
    pub fn add_labeled_waypoint(&mut self, room: &str, label: &str) {
        let order = self.waypoints.len();
        self.waypoints.push(Waypoint::labeled(room, order, label));
    }

    /// Get the ordered waypoints.
    pub fn waypoints(&self) -> &[Waypoint] {
        &self.waypoints
    }

    /// Record a completion of this journey.
    pub fn complete(&mut self) {
        self.completions += 1;
    }

    /// Get the full route as room names in order.
    pub fn route(&self) -> Vec<&str> {
        self.waypoints.iter().map(|w| w.room.as_str()).collect()
    }

    /// Does the journey pass through a specific room?
    pub fn passes_through(&self, room: &str) -> bool {
        self.waypoints.iter().any(|w| w.room == room)
    }

    /// Number of waypoints.
    pub fn waypoint_count(&self) -> usize {
        self.waypoints.len()
    }

    /// Is this a round trip (starts and ends in the same room)?
    pub fn is_round_trip(&self) -> bool {
        if self.waypoints.len() < 2 {
            return false;
        }
        self.waypoints.first().map(|w| w.room.as_str())
            == self.waypoints.last().map(|w| w.room.as_str())
    }

    /// Get waypoint by room name.
    pub fn waypoint_for_room(&self, room: &str) -> Option<&Waypoint> {
        self.waypoints.iter().find(|w| w.room == room)
    }
}

/// A sacred route — a named, well-known pilgrimage with mandatory waypoints.
#[derive(Clone, Debug)]
pub struct Pilgrimage {
    pub name: String,
    waypoints: Vec<Waypoint>,
    /// Rooms that MUST be visited (subset of waypoints).
    mandatory: HashSet<String>,
}

impl Pilgrimage {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            waypoints: Vec::new(),
            mandatory: HashSet::new(),
        }
    }

    /// Add a waypoint.
    pub fn add_waypoint(&mut self, room: &str, mandatory: bool) {
        let order = self.waypoints.len();
        if mandatory {
            self.mandatory.insert(room.to_string());
        }
        self.waypoints.push(Waypoint::new(room, order));
    }

    /// Check if a given route (list of rooms visited) satisfies this pilgrimage.
    /// A route satisfies if it visits all mandatory waypoints in order.
    pub fn is_satisfied_by(&self, visited: &[&str]) -> bool {
        let mandatory_order: Vec<&str> = self
            .waypoints
            .iter()
            .filter(|w| self.mandatory.contains(&w.room))
            .map(|w| w.room.as_str())
            .collect();

        let mut mand_idx = 0;
        for room in visited {
            if mand_idx < mandatory_order.len() && *room == mandatory_order[mand_idx] {
                mand_idx += 1;
            }
        }
        mand_idx == mandatory_order.len()
    }

    pub fn waypoint_count(&self) -> usize {
        self.waypoints.len()
    }

    pub fn mandatory_count(&self) -> usize {
        self.mandatory.len()
    }

    pub fn route(&self) -> Vec<&str> {
        self.waypoints.iter().map(|w| w.room.as_str()).collect()
    }
}

/// An important destination in the fleet — a "shrine" agents may visit.
#[derive(Clone, Debug, PartialEq)]
pub struct Shrine {
    pub name: String,
    pub room: String,
    pub significance: ShrineSignificance,
}

/// How important a shrine is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShrineSignificance {
    /// A nice-to-visit waypoint.
    Minor,
    /// A standard important location.
    Major,
    /// A critical must-visit destination.
    Sacred,
}

impl Shrine {
    pub fn new(name: &str, room: &str, significance: ShrineSignificance) -> Self {
        Self {
            name: name.to_string(),
            room: room.to_string(),
            significance,
        }
    }
}

/// Log of journeys taken by agents.
#[derive(Clone, Debug)]
pub struct PilgrimLog {
    entries: Vec<LogEntry>,
}

#[derive(Clone, Debug)]
struct LogEntry {
    agent: String,
    from_room: String,
    to_room: String,
    tick: u64,
}

impl PilgrimLog {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Log a movement.
    pub fn log(&mut self, agent: &str, from: &str, to: &str, tick: u64) {
        self.entries.push(LogEntry {
            agent: agent.to_string(),
            from_room: from.to_string(),
            to_room: to.to_string(),
            tick,
        });
    }

    /// Total number of logged movements.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Get movements by a specific agent.
    pub fn movements_by(&self, agent: &str) -> Vec<(String, String, u64)> {
        self.entries
            .iter()
            .filter(|e| e.agent == agent)
            .map(|e| (e.from_room.clone(), e.to_room.clone(), e.tick))
            .collect()
    }

    /// Get most visited rooms (by arrivals).
    pub fn most_visited(&self, limit: usize) -> Vec<(String, usize)> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for entry in &self.entries {
            *counts.entry(entry.to_room.clone()).or_insert(0) += 1;
        }
        let mut v: Vec<_> = counts.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v.truncate(limit);
        v
    }

    /// Count movements between two specific rooms.
    pub fn trips_between(&self, from: &str, to: &str) -> usize {
        self.entries
            .iter()
            .filter(|e| e.from_room == from && e.to_room == to)
            .count()
    }
}

impl Default for PilgrimLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Assists navigation on known routes.
#[derive(Clone, Debug)]
pub struct PilgrimGuide {
    /// Known routes: name → ordered rooms.
    routes: HashMap<String, Vec<String>>,
}

impl PilgrimGuide {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Register a named route.
    pub fn register_route(&mut self, name: &str, rooms: Vec<String>) {
        self.routes.insert(name.to_string(), rooms);
    }

    /// Get the next room after `current` on a named route.
    pub fn next_room(&self, route_name: &str, current: &str) -> Option<&str> {
        let route = self.routes.get(route_name)?;
        let idx = route.iter().position(|r| r == current)?;
        route.get(idx + 1).map(|s| s.as_str())
    }

    /// Get the previous room before `current` on a named route.
    pub fn prev_room(&self, route_name: &str, current: &str) -> Option<&str> {
        let route = self.routes.get(route_name)?;
        let idx = route.iter().position(|r| r == current)?;
        if idx > 0 {
            route.get(idx - 1).map(|s| s.as_str())
        } else {
            None
        }
    }

    /// How many steps remain from `current` to the end of the route.
    pub fn steps_remaining(&self, route_name: &str, current: &str) -> Option<usize> {
        let route = self.routes.get(route_name)?;
        let idx = route.iter().position(|r| r == current)?;
        Some(route.len() - idx - 1)
    }

    /// Find all routes that pass through a room.
    pub fn routes_through(&self, room: &str) -> Vec<&str> {
        self.routes
            .iter()
            .filter(|(_, rooms)| rooms.iter().any(|r| r == room))
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// Find a direct route between two rooms (any registered route containing both, in order).
    pub fn find_route(&self, from: &str, to: &str) -> Option<&str> {
        self.routes.iter().find_map(|(name, rooms)| {
            let from_idx = rooms.iter().position(|r| r == from)?;
            let to_idx = rooms.iter().position(|r| r == to)?;
            if from_idx < to_idx {
                Some(name.as_str())
            } else {
                None
            }
        })
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }
}

impl Default for PilgrimGuide {
    fn default() -> Self {
        Self::new()
    }
}

/// Credentials for crossing room boundaries.
#[derive(Clone, Debug, PartialEq)]
pub struct PilgrimPassport {
    pub agent: String,
    /// Rooms this agent is authorized to enter.
    stamps: HashSet<String>,
    /// Rooms explicitly denied.
    denied: HashSet<String>,
}

impl PilgrimPassport {
    pub fn new(agent: &str) -> Self {
        Self {
            agent: agent.to_string(),
            stamps: HashSet::new(),
            denied: HashSet::new(),
        }
    }

    /// Grant access to a room (stamp the passport).
    pub fn stamp(&mut self, room: &str) {
        self.stamps.insert(room.to_string());
        self.denied.remove(room); // stamp overrides denial
    }

    /// Deny access to a room.
    pub fn deny(&mut self, room: &str) {
        self.denied.insert(room.to_string());
        self.stamps.remove(room);
    }

    /// Check if the agent can enter a room.
    pub fn can_enter(&self, room: &str) -> bool {
        self.stamps.contains(room) && !self.denied.contains(room)
    }

    /// List all authorized rooms.
    pub fn authorized_rooms(&self) -> Vec<&str> {
        self.stamps.iter().map(|s| s.as_str()).collect()
    }

    /// Number of stamps.
    pub fn stamp_count(&self) -> usize {
        self.stamps.len()
    }

    /// Revoke access (remove both stamp and denial).
    pub fn revoke(&mut self, room: &str) {
        self.stamps.remove(room);
        self.denied.remove(room);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pilgrim_add_waypoints() {
        let mut p = Pilgrim::new("agent-1", 100);
        p.add_waypoint("room-a");
        p.add_waypoint("room-b");
        p.add_waypoint("room-c");
        assert_eq!(p.waypoint_count(), 3);
        assert_eq!(p.route(), vec!["room-a", "room-b", "room-c"]);
    }

    #[test]
    fn test_pilgrim_labeled_waypoint() {
        let mut p = Pilgrim::new("agent-1", 0);
        p.add_labeled_waypoint("room-a", "start");
        let wp = &p.waypoints()[0];
        assert_eq!(wp.label, Some("start".to_string()));
    }

    #[test]
    fn test_pilgrim_passes_through() {
        let mut p = Pilgrim::new("a", 0);
        p.add_waypoint("x");
        p.add_waypoint("y");
        assert!(p.passes_through("x"));
        assert!(!p.passes_through("z"));
    }

    #[test]
    fn test_pilgrim_round_trip() {
        let mut p = Pilgrim::new("a", 0);
        p.add_waypoint("start");
        p.add_waypoint("mid");
        p.add_waypoint("start");
        assert!(p.is_round_trip());
    }

    #[test]
    fn test_pilgrim_not_round_trip() {
        let mut p = Pilgrim::new("a", 0);
        p.add_waypoint("a");
        p.add_waypoint("b");
        assert!(!p.is_round_trip());
    }

    #[test]
    fn test_pilgrim_completions() {
        let mut p = Pilgrim::new("a", 50);
        p.complete();
        p.complete();
        assert_eq!(p.completions, 2);
    }

    #[test]
    fn test_pilgrim_waypoint_for_room() {
        let mut p = Pilgrim::new("a", 0);
        p.add_waypoint("x");
        p.add_waypoint("y");
        let wp = p.waypoint_for_room("y").unwrap();
        assert_eq!(wp.order, 1);
    }

    #[test]
    fn test_pilgrimage_satisfied() {
        let mut pil = Pilgrimage::new("sacred-route");
        pil.add_waypoint("room-a", true);
        pil.add_waypoint("room-b", false);
        pil.add_waypoint("room-c", true);
        let visited = vec!["room-a", "room-x", "room-c"];
        assert!(pil.is_satisfied_by(&visited));
    }

    #[test]
    fn test_pilgrimage_not_satisfied() {
        let mut pil = Pilgrimage::new("route");
        pil.add_waypoint("room-a", true);
        pil.add_waypoint("room-b", true);
        let visited = vec!["room-a", "room-x"]; // missing room-b
        assert!(!pil.is_satisfied_by(&visited));
    }

    #[test]
    fn test_pilgrimage_out_of_order() {
        let mut pil = Pilgrimage::new("route");
        pil.add_waypoint("a", true);
        pil.add_waypoint("b", true);
        let visited = vec!["b", "a"]; // wrong order
        assert!(!pil.is_satisfied_by(&visited));
    }

    #[test]
    fn test_pilgrimage_counts() {
        let mut pil = Pilgrimage::new("r");
        pil.add_waypoint("a", true);
        pil.add_waypoint("b", false);
        pil.add_waypoint("c", true);
        assert_eq!(pil.waypoint_count(), 3);
        assert_eq!(pil.mandatory_count(), 2);
    }

    #[test]
    fn test_shrine_creation() {
        let s = Shrine::new("core-room", "room-1", ShrineSignificance::Sacred);
        assert_eq!(s.name, "core-room");
        assert_eq!(s.significance, ShrineSignificance::Sacred);
    }

    #[test]
    fn test_pilgrim_log_movements() {
        let mut log = PilgrimLog::new();
        log.log("agent-1", "room-a", "room-b", 10);
        log.log("agent-1", "room-b", "room-c", 20);
        log.log("agent-2", "room-a", "room-c", 15);
        assert_eq!(log.entry_count(), 3);
        let moves = log.movements_by("agent-1");
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_pilgrim_log_most_visited() {
        let mut log = PilgrimLog::new();
        log.log("a1", "x", "room-a", 1);
        log.log("a1", "room-a", "room-a", 2); // visiting room-a again
        log.log("a2", "y", "room-b", 3);
        let top = log.most_visited(1);
        assert_eq!(top[0].0, "room-a");
    }

    #[test]
    fn test_pilgrim_log_trips_between() {
        let mut log = PilgrimLog::new();
        log.log("a1", "x", "y", 1);
        log.log("a1", "x", "y", 2);
        log.log("a1", "y", "x", 3);
        assert_eq!(log.trips_between("x", "y"), 2);
        assert_eq!(log.trips_between("y", "x"), 1);
    }

    #[test]
    fn test_guide_next_prev() {
        let mut guide = PilgrimGuide::new();
        guide.register_route("main-corridor", vec![
            "lobby".to_string(),
            "hall-a".to_string(),
            "hall-b".to_string(),
        ]);
        assert_eq!(guide.next_room("main-corridor", "lobby"), Some("hall-a"));
        assert_eq!(guide.next_room("main-corridor", "hall-b"), None);
        assert_eq!(guide.prev_room("main-corridor", "lobby"), None);
        assert_eq!(guide.prev_room("main-corridor", "hall-b"), Some("hall-a"));
    }

    #[test]
    fn test_guide_steps_remaining() {
        let mut guide = PilgrimGuide::new();
        guide.register_route("r", vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(guide.steps_remaining("r", "a"), Some(2));
        assert_eq!(guide.steps_remaining("r", "c"), Some(0));
    }

    #[test]
    fn test_guide_routes_through() {
        let mut guide = PilgrimGuide::new();
        guide.register_route("r1", vec!["a".to_string(), "b".to_string()]);
        guide.register_route("r2", vec!["b".to_string(), "c".to_string()]);
        let through_b = guide.routes_through("b");
        assert_eq!(through_b.len(), 2);
    }

    #[test]
    fn test_guide_find_route() {
        let mut guide = PilgrimGuide::new();
        guide.register_route("r1", vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(guide.find_route("a", "c"), Some("r1"));
        assert_eq!(guide.find_route("c", "a"), None); // wrong direction
    }

    #[test]
    fn test_passport_stamp_and_check() {
        let mut pp = PilgrimPassport::new("agent-1");
        pp.stamp("room-a");
        pp.stamp("room-b");
        assert!(pp.can_enter("room-a"));
        assert!(!pp.can_enter("room-c"));
        assert_eq!(pp.stamp_count(), 2);
    }

    #[test]
    fn test_passport_deny() {
        let mut pp = PilgrimPassport::new("agent-1");
        pp.stamp("room-a");
        pp.deny("room-a");
        assert!(!pp.can_enter("room-a"));
    }

    #[test]
    fn test_passport_stamp_overrides_deny() {
        let mut pp = PilgrimPassport::new("agent-1");
        pp.deny("room-a");
        pp.stamp("room-a");
        assert!(pp.can_enter("room-a"));
    }

    #[test]
    fn test_passport_revoke() {
        let mut pp = PilgrimPassport::new("agent-1");
        pp.stamp("room-a");
        pp.revoke("room-a");
        assert!(!pp.can_enter("room-a"));
        assert_eq!(pp.stamp_count(), 0);
    }

    #[test]
    fn test_empty_pilgrim_no_round_trip() {
        let p = Pilgrim::new("a", 0);
        assert!(!p.is_round_trip());
    }
}
