use std::sync::Arc;
use std::{ops::DerefMut, time::Duration};

use crate::renderer::Renderer;

use self::{
    agent::{Agent, AgentWrapper},
    message::Message,
};
mod agent;
pub mod message;

pub struct Swarm {
    pub agents: Vec<AgentWrapper>,
}

impl Swarm {
    pub fn new(size: usize) -> Self {
        let mut agents: Vec<AgentWrapper> = vec![];
        let index = 0 as f32;
        while agents.len() != size {
            let (tx, rx) = std::sync::mpsc::channel::<Message>();
            let wrapper = AgentWrapper {
                pointer: Agent::new(index),
                rx: Some(rx),
                tx,
            };

            for agent in agents.iter_mut() {
                {
                    let mut agent = agent.pointer.lock().unwrap();
                    agent.update_table(&wrapper);
                }
                {
                    let mut ag = wrapper.pointer.lock().unwrap();
                    ag.update_table(agent);
                }
            }
            agents.push(wrapper);
        }
        Self { agents }
    }

    pub fn start(&mut self) {
        for agent in self.agents.iter_mut() {
            let a = Arc::clone(&agent.pointer);
            let b = Arc::clone(&agent.pointer);
            let rx = agent.rx.take();
            std::thread::spawn(move || {
                println!("hello world");
                rx.map(|receiver| loop {
                    let message = receiver.recv().unwrap();
                    let mut agent = a.lock().unwrap();
                    agent.on_received(message);
                });
            });
            std::thread::spawn(move || loop {
                {
                    let agent = b.lock().unwrap();
                    agent.send_update_message();
                }
                std::thread::sleep(Duration::from_secs(2));
            });
        }
    }

    pub fn update(&mut self) {
        for agent in self.agents.iter() {
            let mut agent = agent.pointer.lock().unwrap();
            agent.update();
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        for agent in self.agents.iter() {
            let mut agent = agent.pointer.lock().unwrap();
            renderer.subscribe(agent.deref_mut());
        }
    }
}
