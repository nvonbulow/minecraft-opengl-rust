
extern crate uuid;

trait Entity {
    fn uuid(&self) -> uuid::Uuid;

}
