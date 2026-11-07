//! teamspeak model — voice chat server/client.

#[derive(Debug, Clone)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub slots: u32,
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub parent: Option<String>,
    pub users: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub servers: Vec<Server>,
    pub channels: Vec<Channel>,
}

impl Default for Model {
    fn default() -> Self { Self { servers: vec![], channels: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_server(&mut self, server: Server) { self.servers.push(server); }

    pub fn create_channel(&mut self, name: &str, parent: Option<&str>) -> bool {
        if !self.channels.iter().any(|c| c.name == name) {
            self.channels.push(Channel { name: name.into(), parent: parent.map(String::from), users: vec![] });
            true
        } else { false }
    }

    pub fn join(&mut self, channel: &str, user: &str) -> bool {
        if let Some(c) = self.channels.iter_mut().find(|c| c.name == channel) {
            c.users.push(user.into());
            true
        } else { false }
    }

    pub fn list_channels(&self) -> &[Channel] { &self.channels }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn server_channel_join() {
        let mut t = Model::new();
        t.add_server(Server { name: "gaming".into(), address: "ts.example.com".into(), port: 9987, slots: 32 });
        assert!(t.create_channel("lobby", None));
        assert!(t.join("lobby", "alice"));
    }
}
