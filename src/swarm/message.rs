use glam::Vec3;

use super::agent::Agent;

#[derive(Clone, Debug)]
pub enum Message {
    UPDATE {
        from: u32,
        received_list: Vec<u32>,
        payload: Vec<Payload>,
    },
}

impl Message {}

#[derive(Clone, Debug)]
pub struct Payload {
    pub id: u32,
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Payload {
    pub fn from_agent(agent: &Agent) -> Self {
        Self {
            id: agent.id,
            position: agent.transform.get_pos(),
            velocity: agent.velocity,
        }
    }
}
