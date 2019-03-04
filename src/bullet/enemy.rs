use pvector::PVector;
use plane::Enemy;
use consts::*;
use bullet::{EnemyBullet, Bullet};

impl EnemyBullet {
    pub fn manage(bullets: &Vec<Self>, enemies: &Vec<Enemy>) -> Vec<Self> {
        ;
        let mut new_bullets = Enemy::shoot(enemies);
        let mut ret = bullets.clone();
        ret.append(&mut new_bullets);
        super::remove_not_use(ret)
    }
    
    pub fn update(&self) -> Self {
        self
            .move_self()
    }
    
    pub fn update_all(bullets: &Vec<Self>, enemies: &Vec<Enemy>) -> Vec<Self> {
        let ret = bullets
            .into_iter()
            .map(|bullet| bullet.update())
            .collect();
        Self::manage(&ret, enemies)
    }
}

impl Bullet for EnemyBullet {
    fn size(&self) -> f64{
        BULLET_SIZE
    }
    
    fn position(&self) -> PVector{
        self.position.clone()
    }
    
    fn move_self(&self) -> Self {
        let mut ret = self.clone();
        ret.position = ret.position.add(&ret.velocity);
        ret
    }
    
    fn new(position: &PVector) -> Self {
        let velocity = PVector{
            x: 0.0,
            y: 3.0,
        };
        
        Self {
            position: position.clone(),
            velocity: velocity,
        }
    }
}
