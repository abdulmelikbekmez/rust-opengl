use std::sync::Arc;
use std::{ops::DerefMut, time::Duration};

use glam::Vec3;

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
    pub fn new(count: i32) -> Self {
        let padding = 5;
        let mut agents: Vec<AgentWrapper> = vec![];
        for i in -(count / 2)..(count / 2) {
            for j in -(count / 2)..(count / 2) {
                for k in -(count / 2)..(count / 2) {
                    let (tx, rx) = std::sync::mpsc::channel::<Message>();
                    let wrapper = AgentWrapper {
                        pointer: Agent::new(Vec3::new(
                            (i * padding) as f32,
                            (j * padding) as f32,
                            (k * padding) as f32,
                        )),
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
            }
        }
        Self { agents }
    }

    pub fn set_target(&mut self, target: Vec3) {
        for agent in self.agents.iter_mut() {
            agent.tx.send(Message::TASK { target }).unwrap();
            println!("main set task message with target => {}", target);
        }
    }

    pub fn start(&mut self) {
        for agent in self.agents.iter_mut() {
            let a = Arc::clone(&agent.pointer);
            let b = Arc::clone(&a);
            let rx = agent.rx.take();
            std::thread::spawn(move || {
                println!("Agent initialized");
                rx.map(|receiver| loop {
                    let message = receiver.recv().unwrap();
                    {
                        let mut agent = a.lock().unwrap();
                        agent.on_received(message);
                    }
                    std::thread::sleep(Duration::from_millis(5));
                });
            });
            std::thread::spawn(move || loop {
                {
                    let agent = b.lock().unwrap();
                    agent.send_update_message();
                }
                std::thread::sleep(Duration::from_millis(100));
            });
        }
        println!("all agents started!!");
    }

    pub fn update(&mut self) {
        for agent in self.agents.iter() {
            let mut agent = agent.pointer.lock().unwrap();
            // agent.set_align_force();
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
