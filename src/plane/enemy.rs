use rand::prelude::*;
use pvector::PVector;
use consts::*;
use bullet::{EnemyBullet, Bullet, MyBullet};
use plane::{Enemy, Plane, MyPlane};

impl Enemy {
    pub fn update_all(enemies: &Vec<Enemy>, bullets: &Vec<MyBullet>) -> Vec<Enemy> {
        let ret = enemies
            .into_iter()
            .map(|enemy| enemy.update())
            .collect();
        Self::manage(&ret, bullets)
    }
    
    fn attacked(&self, bullets: &Vec<MyBullet>) -> Self {
        let dec_life = bullets
            .into_iter()
            .any(|bullet| bullet.is_attack(self));
        let mut ret = self.clone();
        if dec_life {
            ret.life -= 1;
        }
        ret
    }
    
    fn is_dead(&self) -> bool {
        self.life <= 0
    }
    
    fn remove_dead(enemies: &Vec<Enemy>, bullets: &Vec<MyBullet>) -> Vec<Enemy>{
        enemies
            .into_iter()
            .filter(|enemy| !enemy.is_dead())
            .map(|enemy| enemy.attacked(bullets))
            .collect()
    }
    
    pub fn update(&self)  -> Enemy {
        self
            .move_self()
            .interval_update()
    }
    
    pub fn shoot(enemies: &Vec<Enemy>) -> Vec<EnemyBullet> {
        let ret: Vec<EnemyBullet> =  enemies
            .into_iter()
            .filter(|enemy| enemy.bullet_interval >= ENEMY_BULLET_INTERVAL_MAX)
            .map(|enemy| <EnemyBullet as Bullet>::new(&super::bullet_pos(enemy)))
            .collect();
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
    
    fn remove_not_use(enemies: &Vec<Self>, bullets: &Vec<MyBullet>) -> Vec<Self> {
        let ret = enemies
            .into_iter()
            .filter(|enemy| enemy.is_in_screen())
            .map(|enemy| enemy.clone())
            .collect();
        Enemy::remove_dead(&ret, bullets)
    }
    
    fn is_in_screen(&self) -> bool {
        self.position.y < self.size() + HEIGHT
    }
    
    fn manage(enemies: &Vec<Self>, bullets :&Vec<MyBullet>) -> Vec<Self> {
        let ret = Self::create_enemy(enemies);
        Self::remove_not_use(&ret, bullets)
    }
    
    fn move_self(&self) -> Enemy {
        let mut ret = self.clone();
        ret.position = self.position.add(&ret.velocity);
        ret
    }
}

impl Plane for Enemy {
    fn new() -> Enemy {
        let mut rng = rand::thread_rng();
        let position = PVector::new(rng.gen::<f64>() * WIDTH, 0.0);
        let velocity = PVector::new(0.0, 1.0);
        
        Enemy{
            position: position,
            velocity: velocity,
            bullet_interval: rng.gen::<u64>() % ENEMY_BULLET_INTERVAL_MAX,
            life: ENEMY_LIFE_MAX,
        }
    }
    
    fn size(&self) -> f64 {
        ENEMY_SIZE
    }
    
    fn bullet_size(&self) -> f64 {
        BULLET_SIZE
    }
    
    fn position(&self) -> PVector{
        self.position.clone()
    }
    
    fn interval_update(&self) -> Enemy {
        let mut ret = self.clone();
        if ret.bullet_interval >= ENEMY_BULLET_INTERVAL_MAX {
            ret.bullet_interval = 0;
        }else{
            ret.bullet_interval += 1;
        }
        ret
    }
}
