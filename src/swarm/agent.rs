use glam::Vec3;

use crate::{entity::RenderType, renderer::Drawable, transform::Transform};

use super::message::*;
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
    target: Option<Vec3>,
    render_type: RenderType,
    table: HashMap<u32, Table>,
}

struct Table {
    sender: Sender<Message>,
    position: Vec3,
    velocity: Vec3,
}

impl Agent {
    const PERSEPTON_RADIUS: f32 = 10.;
    const MAX_SPEED: f32 = 0.001;
    const MAX_ACCELERATION: f32 = 0.005;
    pub fn new(pos: Vec3) -> SharedAgent {
        let id = unsafe {
            let tmp = ID;
            ID += 1;
            tmp
        };
        let agent = Self {
            id,
            transform: Transform::with_pos(pos),
            velocity: Vec3::default(),
            acceleration: Vec3::default(),
            target: None,
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
        self.acceleration += self.get_align_force() * 1.1;
        self.acceleration += self.get_cohesion_force() * 1.;
        self.acceleration += self.get_seperation_force() * 100.;
        self.acceleration += self.get_target_force() * 0.01;

        self.velocity += self.acceleration;

        self.velocity = self.velocity.clamp_length_max(Self::MAX_SPEED);
        // println!("before pos => {}", self.transform.get_pos());
        self.transform.update_pos(self.velocity);
        // println!("after pos => {}", self.transform.get_pos());
        self.acceleration = Vec3::ZERO;
    }

    pub fn send_update_message(&self) {
        let mut received_list = vec![self.id];
        let payload = vec![Payload::from_agent(self)];

        let send_list: Vec<_> = self
            .table
            .iter()
            .filter(|(id, table)| {
                !received_list.contains(id)
                    && self.transform.get_pos().distance(table.position) < Self::PERSEPTON_RADIUS
            })
            .map(|(&id, _)| id)
            .collect();
        received_list.extend(send_list.iter());

        for (id, table) in self.table.iter().filter(|(id, _)| send_list.contains(id)) {
            if *id == self.id {
                panic!("HATAAA! tablodaki id ler aynı!! Ayni id nin tabloda olmamsi lazim!!");
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

    pub fn on_received(&mut self, msg: Message) {
        match msg {
            Message::TASK { target } => {
                println!(" {} new target received {}", self.id, target);
                self.target = Some(target);
            }
            Message::UPDATE {
                from,
                mut received_list,
                mut payload,
            } if from != self.id => {
                // Update velocity and positions
                for p in payload.iter() {
                    self.table.get_mut(&p.id).map(|table| {
                        table.position = p.position;
                        table.velocity = p.velocity;
                    });
                }

                payload.push(Payload::from_agent(self));

                let send_list: Vec<_> = self
                    .table
                    .iter()
                    .filter(|(id, table)| {
                        !received_list.contains(id)
                            && self.transform.get_pos().distance(table.position)
                                < Self::PERSEPTON_RADIUS
                    })
                    .map(|(&id, _)| id)
                    .collect();
                received_list.extend(send_list.iter());

                for (_, table) in self.table.iter().filter(|(id, _)| send_list.contains(id)) {
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

    fn get_cohesion_force(&self) -> Vec3 {
        let mut length = 0 as f32;

        let mut average_location = self
            .table
            .iter()
            .filter(|(_, table)| {
                (table.position - self.transform.get_pos()).length() < Self::PERSEPTON_RADIUS
            })
            .fold(Vec3::default(), |a, (_, table)| {
                length += 1.;
                a + table.position
            });

        if length > 0. {
            average_location /= length;
        }
        // println!("center loc => {}", average_location);
        let mut desired_velocity = average_location - self.transform.get_pos();
        desired_velocity = desired_velocity.normalize_or_zero() * Self::MAX_SPEED;
        // if desired_velocity.length() > Self::MAX_SPEED {
        // desired_velocity
        //     .try_normalize()
        //     .map(|n| desired_velocity = n * Self::MAX_SPEED);
        // }
        let force = desired_velocity - self.velocity;
        return force.clamp_length_max(Self::MAX_ACCELERATION);
    }

    fn get_seperation_force(&self) -> Vec3 {
        let mut length = 0 as f32;
        let mut desired_velocity = self
            .table
            .iter()
            .filter(|(_, table)| {
                (table.position - self.transform.get_pos()).length() < Self::PERSEPTON_RADIUS
            })
            .fold(Vec3::default(), |a, (_, table)| {
                length += 1.;
                let mut dif = self.transform.get_pos() - table.position;
                if dif.length() > 0. {
                    dif /= dif.length();
                }
                a + dif
            });

        if length > 0. {
            desired_velocity /= length;
        }
        // println!("center loc => {}", average_location);
        // if tmp.length() > Self::MAX_SPEED {
        // desired_velocity
        //     .try_normalize()
        //     .map(|x| desired_velocity = x * Self::MAX_SPEED);
        // }
        desired_velocity = desired_velocity.normalize_or_zero() * Self::MAX_SPEED;
        let force = desired_velocity - self.velocity;
        // if force.length() > Self::MAX_ACCELERATION {
        //     force
        //         .try_normalize()
        //         .map(|x| force = x * Self::MAX_ACCELERATION);
        // }
        return force.clamp_length_max(Self::MAX_ACCELERATION);
    }

    fn get_target_force(&self) -> Vec3 {
        self.target.map_or(Vec3::ZERO, |x| {
            let desired = x - self.transform.get_pos();
            let mut steering = desired - self.velocity;
            // if steering.length() > Self::MAX_ACCELERATION {
            //     steering
            //         .try_normalize()
            //         .map(|n| steering = n * Self::MAX_ACCELERATION);
            // }
            steering
        })
    }

    fn get_align_force(&self) -> Vec3 {
        let mut length = 0 as f32;
        let mut desired_velocity = self
            .table
            .iter()
            .filter(|(_, table)| {
                (table.position - self.transform.get_pos()).length() < Self::PERSEPTON_RADIUS
            })
            .fold(Vec3::default(), |a, (_, table)| {
                length += 1.;
                a + table.velocity
            });

        if length > 0. {
            desired_velocity /= length;
        }
        // if desired_velocity.length() > Self::MAX_SPEED {
        // desired_velocity
        //     .try_normalize()
        //     .map(|n| desired_velocity = n * Self::MAX_SPEED);
        // }
        desired_velocity = desired_velocity.normalize_or_zero() * Self::MAX_SPEED;
        let force = desired_velocity - self.velocity;

        // if force.length() > Self::MAX_ACCELERATION {
        //     force
        //         .try_normalize()
        //         .map(|n| force = n * Self::MAX_ACCELERATION);
        // }
        return force.clamp_length_max(Self::MAX_ACCELERATION);
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
