// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-cryptsetup: systemd cryptsetup component model.
//!
//! Real implementation: cryptsetups, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cryptsetup {
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
pub enum SystemdCryptsetupError {
    EmptyCryptsetupName,
    DuplicateCryptsetup(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdCryptsetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCryptsetupName => write!(f, "cryptsetup name must not be empty"),
            Self::DuplicateCryptsetup(id) => write!(f, "duplicate cryptsetup {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdCryptsetupError {}

pub struct SystemdCryptsetup {
    pub name: String,
    cryptsetups: BTreeMap<String, Cryptsetup>,
    events: BTreeMap<String, Event>,
}

impl SystemdCryptsetup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cryptsetups: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_cryptsetup(&mut self, item: Cryptsetup) -> Result<(), SystemdCryptsetupError> {
        if item.name.trim().is_empty() {
            return Err(SystemdCryptsetupError::EmptyCryptsetupName);
        }
        let id = item.id.clone();
        if self.cryptsetups.contains_key(&id) {
            return Err(SystemdCryptsetupError::DuplicateCryptsetup(id.clone()));
        }
        self.cryptsetups.insert(id, item);
        Ok(())
    }

    pub fn cryptsetup_count(&self) -> usize {
        self.cryptsetups.len()
    }

    pub fn get_cryptsetup(&self, id: &str) -> Option<&Cryptsetup> {
        self.cryptsetups.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdCryptsetupError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdCryptsetupError::EmptyEventSubject);
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

    fn sample_cryptsetup(id: &str, name: &str) -> Cryptsetup {
        Cryptsetup {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdCryptsetup::new("SystemdCryptsetup");
        assert_eq!(m.name, "SystemdCryptsetup");
        assert_eq!(m.cryptsetup_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_cryptsetup() {
        let mut m = SystemdCryptsetup::new("SystemdCryptsetup");
        m.add_cryptsetup(sample_cryptsetup("a1", "Alpha")).unwrap();
        assert_eq!(m.cryptsetup_count(), 1);
        assert!(m.get_cryptsetup("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdCryptsetup::new("SystemdCryptsetup");
        assert!(m.add_cryptsetup(sample_cryptsetup("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdCryptsetup::new("SystemdCryptsetup");
        m.add_cryptsetup(sample_cryptsetup("x", "One")).unwrap();
        assert!(m.add_cryptsetup(sample_cryptsetup("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdCryptsetup::new("SystemdCryptsetup");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
