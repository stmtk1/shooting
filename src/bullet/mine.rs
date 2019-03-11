use pvector::PVector;
use plane::{MyPlane, Enemy, Plane};
use consts::*;
use bullet::{MyBullet, Bullet};

impl MyBullet {
    pub fn manage(bullets: &Vec<Self>, mine: &MyPlane) -> Vec<Self> {
        let mut ret = bullets.clone();
        if let Some(new_bullet) = mine.shoot() {
            ret.push(new_bullet);
        }
        super::remove_not_use(ret)
    }
    
    pub fn update(&self) -> Self {
        self
            .move_self()
    }
    
    pub fn update_all(bullets: &Vec<Self>, mine: &MyPlane) -> Vec<Self> {
        let ret = bullets
            .into_iter()
            .map(|bullet| bullet.update())
            .collect();
        Self::manage(&ret, mine)
    }
    
    pub fn is_attack(&self, enemy: &Enemy) -> bool {
        enemy.position.x < self.position.x &&
        self.position.x + self.size() < enemy.position.x + enemy.size() &&
        enemy.position.y < self.position.y &&
        self.position.y + self.size() < enemy.position.y + enemy.size()
    }
}

impl Bullet for MyBullet{
    fn size(&self) -> f64 {
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
            y: -3.0,
        };
        
        Self {
            position: position.clone(),
            velocity: velocity,
        }
    }
}
