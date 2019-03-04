mod mine;
mod enemy;

use pvector::PVector;
use enemy::Enemy;
use my_combat::MyCombat;

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
