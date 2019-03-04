//use rand::prelude::*;
use pvector::PVector;
use consts::*;
use piston::input::Key;
use bullet::{MyBullet, Bullet};
use plane::{MyPlane, Plane};

impl MyPlane {
    pub fn update(&self)  -> MyPlane {
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
    
    
    pub fn shoot(&self) ->  Option<MyBullet> {
        if self.bullet_interval >= MY_BULLET_INTERVAL_MAX {
            Some(<MyBullet as Bullet>::new(&super::bullet_pos(self)))
        }else{
            None
        }
    }
    
}

impl Plane for MyPlane {
    fn new() -> MyPlane {
        let position = PVector::new(WIDTH * 0.5, HEIGHT * 0.7);
        let velocity = PVector::new(0.0, 0.0);
        MyPlane {
            position: position,
            velocity: velocity,
            bullet_interval: MY_BULLET_INTERVAL_MAX - 1,
        }
    }
    
    fn size(&self) -> f64 {
        MY_COMBAT_SIZE
    }
    
    fn bullet_size(&self) -> f64 {
        BULLET_SIZE
    }
    
    fn position(&self) -> PVector {
        self.position.clone()
    }
    
    fn interval_update(&self) -> Self {
        let mut ret = self.clone();
        if ret.bullet_interval >= MY_BULLET_INTERVAL_MAX {
            ret.bullet_interval = 0;
        }else{
            ret.bullet_interval += 1;
        }
        ret
    }
}
