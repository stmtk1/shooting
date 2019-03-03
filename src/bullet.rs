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

impl EnemyBullet {
    pub fn new(position: &PVector) -> Self {
        let velocity = PVector{
            x: 0.0,
            y: 3.0,
        };
        
        Self {
            position: position.clone(),
            velocity: velocity,
        }
    }
    
    pub fn manage(bullets: &Vec<Self>, enemies: &Vec<Enemy>) -> Vec<Self> {
        ;
        let mut new_bullets = Enemy::shoot(enemies);
        let mut ret = bullets.clone();
        ret.append(&mut new_bullets);
        ret
    }
    
    pub fn move_self(&self) -> Self {
        let mut ret = self.clone();
        ret.position = ret.position.add(&ret.velocity);
        ret
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

impl MyBullet {
    pub fn new(position: &PVector) -> Self {
        let velocity = PVector{
            x: 0.0,
            y: -3.0,
        };
        
        Self {
            position: position.clone(),
            velocity: velocity,
        }
    }
    
    pub fn manage(bullets: &Vec<Self>, mine: &MyCombat) -> Vec<Self> {
        let mut ret = bullets.clone();
        if let Some(new_bullet) = mine.shoot() {
            ret.push(new_bullet);
        }
        ret
    }
    
    pub fn move_self(&self) -> Self {
        let mut ret = self.clone();
        ret.position = ret.position.add(&ret.velocity);
        ret
    }
    
    pub fn update(&self) -> Self {
        self
            .move_self()
    }
    
    pub fn update_all(bullets: &Vec<Self>, mine: &MyCombat) -> Vec<Self> {
        let ret = bullets
            .into_iter()
            .map(|bullet| bullet.update())
            .collect();
        Self::manage(&ret, mine)
    }
}
