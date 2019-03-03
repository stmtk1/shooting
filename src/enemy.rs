use rand::prelude::*;
use pvector::PVector;
use consts::*;
use bullet::{EnemyBullet, MyBullet};

#[derive(Clone)]
pub struct Enemy {
    pub position: PVector,
    velocity: PVector,
    bullet_interval: u64,
}

impl Enemy {
    pub fn new() -> Enemy {
        let mut rng = rand::thread_rng();
        let position = PVector::new(rng.gen::<f64>() * WIDTH, 0.0);
        let velocity = PVector::new(0.0, 1.0);
        
        Enemy{
            position: position,
            velocity: velocity,
            bullet_interval: rng.gen::<u64>() % BULLET_INTERVAL_MAX,
        }
    }
    
    pub fn update_all(enemies: &Vec<Enemy>) -> Vec<Enemy> {
        let ret = enemies
            .into_iter()
            .map(|enemy| enemy.update())
            .collect();
        Enemy::create_enemy(&ret)
    }
    
    pub fn update(&self)  -> Enemy {
        self
            .move_self()
            .interval_update()
    }
    
    pub fn move_self(&self) -> Enemy {
        let mut ret = self.clone();
        ret.position = self.position.add(&ret.velocity);
        ret
    }
    
    pub fn shoot(enemies: &Vec<Enemy>) -> Vec<EnemyBullet> {
        let ret: Vec<EnemyBullet> =  enemies
            .into_iter()
            .filter(|enemy| enemy.bullet_interval >= BULLET_INTERVAL_MAX)
            .map(|enemy| EnemyBullet::new(&enemy.bullet_pos()))
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
}
