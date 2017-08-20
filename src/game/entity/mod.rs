
extern crate uuid;

mod player;

trait Entity {
    fn uuid(&self) -> uuid::Uuid;

}
