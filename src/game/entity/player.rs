extern crate uuid;

use ::game::entity::Entity;

struct Player {
    position: [f32; 3],

}

impl Entity for Player {
    fn uuid(&self) -> uuid::Uuid {
        unimplemented!()
    }
}