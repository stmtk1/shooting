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
use enemy::Enemy;
use bullet::Bullet;
use my_combat::MyCombat;
//use rand::prelude::*;

// #[derive(Clone)]
pub struct App {
    gl: GlGraphics,
    window: Window,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    my_combat: MyCombat,
}

impl App {
    pub fn new() -> App {
        let opengl = OpenGL::V3_2;
        let window = App::new_window(opengl);
        App {
            gl: GlGraphics::new(opengl),
            window: window,
            enemies: Vec::with_capacity(100),
            bullets: Vec::with_capacity(100),
            my_combat: MyCombat::new(),
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
        }
    }
    
    pub fn render(&mut self, args: &RenderArgs){
        let bullet_square: graphics::types::Rectangle = square(0.0, 0.0, BULLET_SIZE);
        let enemy_square: graphics::types::Rectangle = square(0.0, 0.0, ENEMY_SIZE);
        let my_combat_square: graphics::types::Rectangle = square(0.0, 0.0, MY_COMBAT_SIZE);

        //const TRIANGLE:   &[[f32; 2]; 3] = &[[1.0, 0.0], [0.0, 1.732], [2.0, 1.732]];
        let enemies = self.enemies.clone();
        let bullets = self.bullets.clone();
        let my_combat = self.my_combat.clone();

        self.gl.draw(args.viewport(), |c, gl|{
            clear(WHITE, gl);
            
            App::draw_enemies(&c, gl, &enemies, enemy_square);
            App::draw_bullets(&c, gl, &bullets, bullet_square);
            App::draw_my_combat(&c, gl, &my_combat, my_combat_square);
        });
    }
    
    fn draw_enemies(c: &Context, gl: &mut GlGraphics, enemies: &Vec<Enemy>, square: graphics::types::Rectangle) {
        for enemy in enemies {
            let transform = c.transform
                .trans(enemy.position.x, enemy.position.y);
            rectangle(RED, square, transform, gl);
            //polygon(RED, &TRIANGLE, transform, gl);
        }
    }
    
    fn draw_my_combat(c: &Context, gl: &mut GlGraphics, mine: &MyCombat, square: graphics::types::Rectangle) {
        let transform = c.transform
            .trans(mine.position.x, mine.position.y);
        rectangle(BLACK, square, transform, gl);
        //polygon(RED, &TRIANGLE, transform, gl);
    }
    
    fn draw_bullets(c: &Context, gl: &mut GlGraphics, bullets: &Vec<Bullet>, square: graphics::types::Rectangle) {
        for bullet in bullets {
            let transform = c.transform
                .trans(bullet.position.x, bullet.position.y);
            rectangle(BLUE, square, transform, gl);
            //polygon(RED, &TRIANGLE, transform, gl);
        }
    }
    
    pub fn update(&mut self){
        self.bullets = Bullet::update_all(&self.bullets, &self.enemies);
        self.enemies = Enemy::update_all(&self.enemies);
        self.my_combat = self.my_combat.update();
    }
}
