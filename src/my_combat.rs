//use rand::prelude::*;
use pvector::PVector;
use consts::*;
use piston::input::Key;
use bullet::{EnemyBullet, MyBullet};
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
            .interval_update()
    }
    
    pub fn apply_key(&self, key: Key) -> Self{
        let mut ret = self.clone();
        let mut position = ret.position;
        if key == Key::Left {
            position = position.add(&PVector::new(-5.0, 0.0));
        }else if key == Key::Right {
            position = position.add(&PVector::new(5.0, 0.0));
        }else if key == Key::Up {
            position = position.add(&PVector::new(0.0, -5.0));
        }else if key == Key::Down {
            position = position.add(&PVector::new(0.0, 5.0));
        }
        ret.position = position;
        ret
    }
    
    pub fn interval_update(&self) -> Self {
        let mut ret = self.clone();
        if ret.bullet_interval >= BULLET_INTERVAL_MAX {
            ret.bullet_interval = 0;
        }else{
            ret.bullet_interval += 1;
        }
        ret
    }
    
    pub fn shoot(&self) ->  Option<MyBullet> {
        if self.bullet_interval >= BULLET_INTERVAL_MAX {
            Some(MyBullet::new(&self.bullet_pos()))
        }else{
            None
        }
        /*
        let ret: Vec<MyBullet> =  enemies
            .into_iter()
            .filter(|enemy| )
            .map(|enemy| Bullet::new(&(enemy.bullet_pos())))
            .collect();
        ret
        */
    }
    
    pub fn bullet_pos(&self) -> PVector{
        self.position.add(&PVector::new((ENEMY_SIZE - BULLET_SIZE).abs() / 2.0, 0.0))
    }
    /*
    pub fn move_self(&self) -> Enemy {
        let mut ret = self.clone();
        ret.position = self.position.add(&ret.velocity);
        ret
    }
    
    pub fn bullet_pos(&self) -> PVector{
        self.position.add(&PVector::new((ENEMY_SIZE - BULLET_SIZE).abs() / 2.0, 0.0))
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
