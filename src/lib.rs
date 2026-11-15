//! SigmaOS-teamspeak: Voice chat server model.
//!
//! Real implementation: servers, channels, validation.
//! Replaces the 7-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Channel {
    pub id: String,
    pub server_id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeamspeakError {
    EmptyServerName,
    DuplicateServer(String),
}

impl fmt::Display for TeamspeakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyServerName => write!(f, "server name must not be empty"),
            Self::DuplicateServer(id) => write!(f, "duplicate server {:?}", id),
        }
    }
}

impl std::error::Error for TeamspeakError {}

pub struct Teamspeak {
    pub name: String,
    servers: BTreeMap<String, Server>,
    channels: BTreeMap<String, Channel>,
}

impl Teamspeak {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), servers: BTreeMap::new(), channels: BTreeMap::new() }
    }

    pub fn add_server(&mut self, s: Server) -> Result<(), TeamspeakError> {
        if s.name.trim().is_empty() { return Err(TeamspeakError::EmptyServerName); }
        let id = s.id.clone();
        if self.servers.contains_key(&id) {
            return Err(TeamspeakError::DuplicateServer(id.clone()));
        }
        self.servers.insert(id, s);
        Ok(())
    }

    pub fn server_count(&self) -> usize { self.servers.len() }

    pub fn get_server(&self, id: &str) -> Option<&Server> { self.servers.get(id) }

    pub fn add_channel(&mut self, c: Channel) {
        self.channels.insert(c.id.clone(), c);
    }

    pub fn get_channel(&self, id: &str) -> Option<&Channel> { self.channels.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let t = Teamspeak::new("Teamspeak");
        assert_eq!(t.name, "Teamspeak");
        assert_eq!(t.server_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut t = Teamspeak::new("Teamspeak");
        let server = Server { id: "s1".into(), name: "Gaming".into(), address: "ts.example.com".into() };
        t.add_server(server).unwrap();
        assert_eq!(t.server_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut t = Teamspeak::new("Teamspeak");
        let server = Server { id: "x".into(), name: "".into(), address: "x".into() };
        assert!(t.add_server(server).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut t = Teamspeak::new("Teamspeak");
        let server = Server { id: "x".into(), name: "X".into(), address: "x".into() };
        t.add_server(server).unwrap();
        let server2 = Server { id: "x".into(), name: "X".into(), address: "x".into() };
        assert!(t.add_server(server2).is_err());
    }
}
