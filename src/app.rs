use piston::input::RenderArgs;
use consts::*;
use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::{rectangle, clear};
use graphics::rectangle::square;
use graphics::Transformed;
use graphics::context::Context;
use piston::event_loop::*;
use piston::input::*;
use bullet::{EnemyBullet, MyBullet, Bullet};
use plane::{MyPlane, Enemy, Plane};
//use rand::prelude::*;

// #[derive(Clone)]
pub struct App {
    gl: GlGraphics,
    window: Window,
    enemies: Vec<Enemy>,
    enemy_bullets: Vec<EnemyBullet>,
    my_bullets: Vec<MyBullet>,
    my_combat: MyPlane,
}

impl App {
    pub fn new() -> App {
        let opengl = OpenGL::V3_2;
        let window = App::new_window(opengl);
        App {
            gl: GlGraphics::new(opengl),
            window: window,
            enemies: Vec::with_capacity(100),
            enemy_bullets: Vec::with_capacity(100),
            my_bullets: Vec::with_capacity(100),
            my_combat: <MyPlane as Plane>::new(),
        }
    }
    
    fn new_window(opengl: OpenGL) -> Window{
        WindowSettings::new(
                "spinning-square",
                [WIDTH as u32, HEIGHT as u32]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap()
    }
    
    pub fn show_window(&mut self){
        let mut events = Events::new(EventSettings::new());
        
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args(){ 
                self.render(&r);
            }
            
            if let Some(_) = e.update_args() {
                self.update();
            }
            
            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.my_combat = self.my_combat.apply_key(key);
            }
        }
    }
    
    pub fn render(&mut self, args: &RenderArgs){
        let bullet_square: graphics::types::Rectangle = square(0.0, 0.0, BULLET_SIZE);
        let enemy_square: graphics::types::Rectangle = square(0.0, 0.0, ENEMY_SIZE);
        let my_combat_square: graphics::types::Rectangle = square(0.0, 0.0, MY_COMBAT_SIZE);

        //const TRIANGLE:   &[[f32; 2]; 3] = &[[1.0, 0.0], [0.0, 1.732], [2.0, 1.732]];
        let enemies = self.enemies.clone();
        let enemy_bullets = self.enemy_bullets.clone();
        let my_bullets = self.my_bullets.clone();
        let my_combat = self.my_combat.clone();

        self.gl.draw(args.viewport(), |c, gl|{
            clear(WHITE, gl);
            
            App::draw_enemies(&c, gl, &enemies, enemy_square);
            App::draw_enemy_bullets(&c, gl, &enemy_bullets, bullet_square);
            App::draw_my_bullets(&c, gl, &my_bullets, bullet_square);
            App::draw_my_combat(&c, gl, &my_combat, my_combat_square);
        });
    }
    
    fn draw_enemies(c: &Context, gl: &mut GlGraphics, enemies: &Vec<Enemy>, square: graphics::types::Rectangle) {
        for enemy in enemies {
            let transform = c.transform
                .trans(enemy.position().x, enemy.position().y);
            rectangle(RED, square, transform, gl);
            //polygon(RED, &TRIANGLE, transform, gl);
        }
    }
    
    fn draw_my_combat(c: &Context, gl: &mut GlGraphics, mine: &MyPlane, square: graphics::types::Rectangle) {
        let transform = c.transform
            .trans(mine.position().x, mine.position().y);
        rectangle(BLACK, square, transform, gl);
        //polygon(RED, &TRIANGLE, transform, gl);
    }
    
    fn draw_enemy_bullets(c: &Context, gl: &mut GlGraphics, bullets: &Vec<EnemyBullet>, square: graphics::types::Rectangle) {
        for bullet in bullets {
            let transform = c.transform
                .trans(bullet.position().x, bullet.position().y);
            rectangle(BLUE, square, transform, gl);
            //polygon(RED, &TRIANGLE, transform, gl);
        }
    }
    
    fn draw_my_bullets(c: &Context, gl: &mut GlGraphics, bullets: &Vec<MyBullet>, square: graphics::types::Rectangle) {
        for bullet in bullets {
            let transform = c.transform
                .trans(bullet.position().x, bullet.position().y);
            rectangle(RED, square, transform, gl);
            //polygon(RED, &TRIANGLE, transform, gl);
        }
    }
    
    pub fn update(&mut self){
        self.enemy_bullets = EnemyBullet::update_all(&self.enemy_bullets, &self.enemies);
        self.my_bullets = MyBullet::update_all(&self.my_bullets, &self.my_combat);
        self.enemies = Enemy::update_all(&self.enemies, &self.my_bullets);
        self.my_combat = self.my_combat.update(&self.enemy_bullets);
    }
}
