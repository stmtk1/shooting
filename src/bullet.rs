use pvector::PVector;
use enemy::Enemy;

#[derive(Clone)]
pub struct Bullet {
    pub position: PVector,
    velocity: PVector,
}

impl Bullet {
    pub fn new(position: &PVector) -> Bullet {
        let velocity = PVector{
            x: 0.0,
            y: 3.0,
        };
        
        Bullet {
            position: position.clone(),
            velocity: velocity,
        }
    }
    
    pub fn manage(bullets: &Vec<Bullet>, enemies: &Vec<Enemy>) -> Vec<Bullet> {
        let mut new_bullets = Enemy::shoot(enemies);
        let mut ret = bullets.clone();
        ret.append(&mut new_bullets);
        ret
    }
    
    pub fn move_self(&self) -> Bullet {
        let mut ret = self.clone();
        ret.position = ret.position.add(&ret.velocity);
        ret
    }
    
    pub fn update(&self) -> Bullet {
        self
            .move_self()
    }
    
    pub fn update_all(bullets: &Vec<Bullet>, enemies: &Vec<Enemy>) -> Vec<Bullet> {
        let ret = bullets
            .into_iter()
            .map(|bullet| bullet.update())
            .collect();
        Bullet::manage(&ret, enemies)
    }
}
