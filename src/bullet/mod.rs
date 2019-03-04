mod mine;
mod enemy;

use pvector::PVector;
use consts::*;

#[derive(Clone)]
pub struct EnemyBullet {
    pub position: PVector,
    velocity: PVector,
}

#[derive(Clone)]
pub struct MyBullet {
    pub position: PVector,
    velocity: PVector,
}

pub trait Bullet{
    fn new(pvector: &PVector) -> Self;
    fn size(&self) -> f64;
    fn position(&self) -> PVector;
    fn move_self(&self) -> Self;
}

pub fn is_in_screen<T: Bullet>(bullet: &T) -> bool {
    bullet.position().y < bullet.size() + HEIGHT
}

pub fn remove_not_use<T: Bullet>(bullets: Vec<T>) -> Vec<T> {
    bullets
        .into_iter()
        .filter(|bullet| is_in_screen::<T>(bullet))
        .collect()
}
