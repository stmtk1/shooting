//use rand::prelude::*;
use pvector::PVector;
use consts::*;
//use bullet::Bullet;

#[derive(Clone)]
pub struct MyCombat {
    pub position: PVector,
    velocity: PVector,
    bullet_interval: u64,
}

impl MyCombat {
    pub fn new() -> MyCombat {
        let position = PVector::new(WIDTH * 0.5, HEIGHT * 0.7);
        let velocity = PVector::new(0.0, 0.0);
        MyCombat {
            position: position,
            velocity: velocity,
            bullet_interval: BULLET_INTERVAL_MAX - 1,
        }
    }
    
    pub fn update(&self)  -> MyCombat {
        self
            .clone()
            /*
            .move_self()
            .interval_update()
            */
    }
    
    /*
    pub fn move_self(&self) -> Enemy {
        let mut ret = self.clone();
        ret.position = self.position.add(&ret.velocity);
        ret
    }
    
    pub fn shoot(enemies: &Vec<Enemy>) -> Vec<Bullet> {
        let ret: Vec<Bullet> =  enemies
            .into_iter()
            .filter(|enemy| enemy.bullet_interval >= BULLET_INTERVAL_MAX)
            .map(|enemy| Bullet::new(&(enemy.bullet_pos())))
            .collect();
        ret
    }
    pub fn bullet_pos(&self) -> PVector{
        self.position.add(&PVector::new((ENEMY_SIZE - BULLET_SIZE).abs() / 2.0, 0.0))
    }
    pub fn interval_update(&self) -> Enemy {
        let mut ret = self.clone();
        if ret.bullet_interval >= BULLET_INTERVAL_MAX {
            ret.bullet_interval = 0;
        }else{
            ret.bullet_interval += 1;
        }
        ret
    }
    
    fn create_enemy(enemies: &Vec<Enemy>) -> Vec<Enemy> {
        let mut ret = enemies.clone();
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < 0.01 {
            ret.push(Enemy::new());
        }
        ret
    }
    */
}
