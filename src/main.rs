extern crate rand;
extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

mod app;
mod consts;
mod enemy;
mod pvector;
mod bullet;
mod my_combat;

use app::App;

fn main(){
    let mut app = App::new();
    for _ in 1..100 {
        app.show_window();
    }
}
