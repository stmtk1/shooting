use pvector::PVector;
use enemy::Enemy;
use my_combat::MyCombat;
use consts::*;
use bullet::MyBullet;

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
        Self::remove_not_use(&ret)
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
    
    fn remove_not_use(bullets: &Vec<Self>) -> Vec<Self> {
        bullets
            .into_iter()
            .filter(|bullet| bullet.is_in_screen())
            .map(|bullet| bullet.clone())
            .collect()
    }
    
    fn is_in_screen(&self) -> bool {
        self.position.y < self.size() + HEIGHT && self.position.y > -1.0 * self.size()
    }
    
    fn size(&self) -> f64 {
        BULLET_SIZE
    }
}
