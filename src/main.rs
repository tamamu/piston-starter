extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;

use std::f64;
use piston_window::*;
use gfx_device_gl::Resources;

const HALF: f64 = f64::consts::PI / 2.0;

struct Object {
    texture: Texture<Resources>,
    x: f64,
    y: f64,
    rot: f64,
    deg: f64,
    spd: f64,
}

struct App {
    window: PistonWindow,
    obj: Option<Object>,
}

impl App {
    fn load(&mut self) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let d_man = assets.join("d.png");
        let d_man = Texture::from_path(&mut self.window.factory,
                                       &d_man,
                                       Flip::None,
                                       &TextureSettings::new())
            .unwrap();
        self.obj = Some(Object {
            texture: d_man,
            x: 400.0,
            y: 300.0,
            rot: 0.0,
            deg: f64::consts::PI / 4.0,
            spd: 600.0,
        });
    }
    fn update(&mut self, args: &UpdateArgs) {
        let mut obj = self.obj.as_mut().unwrap();
        obj.rot += 6.0 * args.dt;
        obj.x += obj.deg.cos() * obj.spd * args.dt;
        obj.y += obj.deg.sin() * obj.spd * args.dt;
        let (x, y) = (obj.x, obj.y);
        let (w, h) = obj.texture.get_size();
        let (w, h) = (w as f64, h as f64);
        let wsize = self.window.draw_size();
        if x < w / 3.0 || x > wsize.width as f64 - w / 3.0 || y < h / 3.0 ||
           y > wsize.height as f64 - h / 3.0 {
            obj.deg += HALF;
        }
    }
    fn draw(&mut self, e: &Event) {
        let obj = self.obj.as_mut().unwrap();
        self.window.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            let (w, h) = obj.texture.get_size();
            let (w, h) = (w as f64, h as f64);
            let transform =
                c.transform.trans(obj.x, obj.y).rot_rad(obj.rot).trans(-w / 2.0, -h / 2.0);
            image(&obj.texture, transform, g);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let window: PistonWindow = WindowSettings::new("D-Man", [800, 600])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut app = App {
        window: window,
        obj: None,
    };

    app.load();

    while let Some(e) = app.window.next() {
        app.draw(&e);
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
