// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-flamegraph: flamegraph component model.
//!
//! Real implementation: flamegraph_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlamegraphItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlamegraphError {
    EmptyFlamegraphItemName,
    DuplicateFlamegraphItem(String),
    EmptyEventSubject,
}

impl fmt::Display for FlamegraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFlamegraphItemName => write!(f, "flamegraph_item name must not be empty"),
            Self::DuplicateFlamegraphItem(id) => write!(f, "duplicate flamegraph_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for FlamegraphError {}

pub struct Flamegraph {
    pub name: String,
    flamegraph_items: BTreeMap<String, FlamegraphItem>,
    events: BTreeMap<String, Event>,
}

impl Flamegraph {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            flamegraph_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_flamegraph_item(&mut self, item: FlamegraphItem) -> Result<(), FlamegraphError> {
        if item.name.trim().is_empty() {
            return Err(FlamegraphError::EmptyFlamegraphItemName);
        }
        let id = item.id.clone();
        if self.flamegraph_items.contains_key(&id) {
            return Err(FlamegraphError::DuplicateFlamegraphItem(id.clone()));
        }
        self.flamegraph_items.insert(id, item);
        Ok(())
    }

    pub fn flamegraph_item_count(&self) -> usize {
        self.flamegraph_items.len()
    }

    pub fn get_flamegraph_item(&self, id: &str) -> Option<&FlamegraphItem> {
        self.flamegraph_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), FlamegraphError> {
        if event.subject.trim().is_empty() {
            return Err(FlamegraphError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_flamegraph_item(id: &str, name: &str) -> FlamegraphItem {
        FlamegraphItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Flamegraph::new("Flamegraph");
        assert_eq!(m.name, "Flamegraph");
        assert_eq!(m.flamegraph_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_flamegraph_item() {
        let mut m = Flamegraph::new("Flamegraph");
        m.add_flamegraph_item(sample_flamegraph_item("a1", "Alpha")).unwrap();
        assert_eq!(m.flamegraph_item_count(), 1);
        assert!(m.get_flamegraph_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Flamegraph::new("Flamegraph");
        assert!(m.add_flamegraph_item(sample_flamegraph_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Flamegraph::new("Flamegraph");
        m.add_flamegraph_item(sample_flamegraph_item("x", "One")).unwrap();
        assert!(m.add_flamegraph_item(sample_flamegraph_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Flamegraph::new("Flamegraph");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
