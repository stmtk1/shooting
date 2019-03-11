mod mine;
mod enemy;

use pvector::PVector;
//use plane::Enemy;
//use plane::MyPlane;

#[derive(Clone)]
pub struct MyPlane {
    pub position: PVector,
    velocity: PVector,
    bullet_interval: u64,
}

#[derive(Clone)]
pub struct Enemy {
    pub position: PVector,
    velocity: PVector,
    bullet_interval: u64,
    life: u32,
}

pub trait Plane{
    fn new() -> Self;
    fn size(&self) -> f64;
    fn bullet_size(&self) -> f64;
    fn position(&self) -> PVector;
    fn interval_update(&self) -> Self;
}

pub fn bullet_pos<T: Plane>(plane: &T) -> PVector{
    let offset = PVector::new((plane.size() - plane.bullet_size()).abs() / 2.0, 0.0);
    plane.position().add(&offset)
}
