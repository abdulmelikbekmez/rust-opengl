use glam::Vec3;

use crate::{entity::RenderType, renderer::Drawable, transform::Transform};

use super::message::{Message, Payload};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};
pub type SharedAgent = Arc<Mutex<Agent>>;

static mut ID: u32 = 1;

pub struct AgentWrapper {
    pub pointer: SharedAgent,
    pub rx: Option<Receiver<Message>>,
    pub tx: Sender<Message>,
}

pub struct Agent {
    pub id: u32,
    pub transform: Transform,
    pub velocity: Vec3,
    acceleration: Vec3,
    render_type: RenderType,
    table: HashMap<u32, Table>,
}

struct Table {
    sender: Sender<Message>,
    position: Vec3,
    velocity: Vec3,
}

impl Agent {
    pub fn new(x: f32) -> SharedAgent {
        let id = unsafe {
            let tmp = ID;
            ID += 1;
            tmp
        };
        let agent = Self {
            id,
            transform: Transform::with_pos(Vec3::new(x, 0., 0.)),
            velocity: Vec3::new(0.00001, 0.000001, 0.),
            acceleration: Vec3::default(),
            render_type: RenderType::DYNAMIC,
            table: HashMap::new(),
        };
        Arc::new(Mutex::new(agent))
    }

    pub fn update_table(&mut self, wrapper: &AgentWrapper) {
        let agent = wrapper.pointer.lock().unwrap();
        self.table.insert(
            agent.id,
            Table {
                sender: wrapper.tx.clone(),
                position: agent.transform.get_pos(),
                velocity: agent.velocity,
            },
        );
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.transform.update_pos(self.velocity);
        self.acceleration = Vec3::default();
    }

    pub fn send_update_message(&self) {
        // println!("table checking me => {} start", self.id);
        for (id, table) in self.table.iter() {
            // println!("table checking iterated => {}", id);
            if *id == self.id {
                panic!("HATAAA! tablodaki id ler aynı!! Ayni id nin tabloda olmamsi lazim!!");
            }
            table
                .sender
                .send(Message::UPDATE {
                    from: self.id,
                    received_list: vec![self.id],
                    payload: vec![Payload::from_agent(self)],
                })
                .unwrap();
        }
        // println!("table checking me => {} end", self.id);
    }

    pub fn on_received(&mut self, msg: Message) {
        match msg {
            Message::UPDATE {
                from,
                mut received_list,
                mut payload,
            } if from != self.id && !received_list.contains(&self.id) => {
                // Update velocity and posiitons
                for p in payload.iter() {
                    self.table.get_mut(&p.id).map(|table| {
                        table.position = p.position;
                        table.velocity = p.velocity;
                    });
                }

                payload.push(Payload::from_agent(self));
                received_list.push(self.id);

                for (id, table) in self.table.iter() {
                    if received_list.contains(id) {
                        continue;
                    }
                    table
                        .sender
                        .send(Message::UPDATE {
                            from: self.id.clone(),
                            received_list: received_list.clone(),
                            payload: payload.clone(),
                        })
                        .unwrap();
                }
            }
            _ => println!("I {} already got that message bro", self.id),
        }
    }
}

impl Drawable for Agent {
    fn get_model_matrix(&self) -> [f32; 16] {
        self.transform.get_matrix().to_cols_array()
    }

    fn get_render_type(&mut self) -> &mut crate::entity::RenderType {
        &mut self.render_type
    }
}
