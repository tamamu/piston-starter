extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;

extern crate rand;

use std::f64;
use rand::{thread_rng, Rng, ThreadRng};
use piston_window::*;
use gfx_device_gl::Resources;

const HALF: f64 = f64::consts::PI / 2.0;

#[derive(Clone)]
struct Object {
    x: f64,
    y: f64,
    rot: f64,
    deg: f64,
    spd: f64,
}

struct GameButton {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl GameButton {
    fn new() -> Self {
        GameButton {
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }
}

struct App {
    window: PistonWindow,
    obj: Vec<Object>,
    texture: Option<Texture<Resources>>,
    pressed: GameButton,
}

impl App {
    fn load(&mut self) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let d_man = assets.join("go.png");
        let d_man = Texture::from_path(&mut self.window.factory,
                                       &d_man,
                                       Flip::None,
                                       &TextureSettings::new())
            .unwrap();
        self.texture = Some(d_man);
        self.obj.push(Object {
            x: 400.0,
            y: 300.0,
            rot: 0.0,
            deg: f64::consts::PI / 4.0,
            spd: 600.0,
        });
    }


    fn duplicate(&mut self, rng: &mut ThreadRng) {
        let mut o = self.obj.first().expect("There is no Gopher!").clone();
        let rnd = rng.gen::<f64>() % 10.0;
        o.deg += HALF / 3.14 * rnd;
        o.rot += HALF * 3.14 / rnd;
        o.spd *= 1.8;
        if o.spd > 18.0 {
            o.spd -= 18.0;
        }
        self.obj.push(o);
    }

    fn update(&mut self, args: &UpdateArgs) {
        let (w, h) = self.texture.as_ref().unwrap().get_size();
        let (w, h) = (w as f64 / 3.0, h as f64 / 3.0);
        let wsize = self.window.draw_size();
        let (ww, wh) = (wsize.width as f64, wsize.height as f64);
        for o in self.obj.iter_mut() {
            // o.rot += 6.0 * args.dt;
            // o.x += o.deg.cos() * o.spd * args.dt;
            // o.y += o.deg.sin() * o.spd * args.dt;

            if o.x < w || o.x > ww - w || o.y < h || o.y > wh - h {
                // o.deg += HALF;
            }
        }
    }
    fn draw(&mut self, e: &Event) {
        let ref obj = self.obj;
        let texture = self.texture.as_ref().unwrap();
        let (w, h) = self.texture.as_ref().unwrap().get_size();
        let (w, h) = (-(w as f64 / 2.0), -(h as f64 / 2.0));
        self.window.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            for o in obj {
                let transform = c.transform.trans(o.x, o.y).rot_rad(o.rot).trans(w, h);
                image(texture, transform, g);
            }
        });
    }
}



fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Gophers", [800, 600])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    window.set_position((300, 200));

    let mut app = App {
        window: window,
        obj: Vec::new(),
        texture: None,
        pressed: GameButton::new(),
    };

    let mut acc_time = 0f64;
    let mut rng = thread_rng();

    app.load();

    // for i in 0..10000 {
    // app.duplicate(&mut rng);
    // }

    while let Some(e) = app.window.next() {
        if let Some(p) = e.press_args() {
            match p {
                Button::Keyboard(key) => {
                    match key {
                        Key::Left => {
                            app.pressed.left = true;
                        }
                        Key::Right => {
                            app.pressed.right = true;
                        }
                        _ => {
                            println!("Press other!");
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(r) = e.release_args() {
            match r {
                Button::Keyboard(key) => {
                    match key {
                        Key::Left => {
                            app.pressed.left = false;
                        }
                        Key::Right => {
                            app.pressed.right = false;
                        }
                        _ => {
                            println!("Release other!");
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(u) = e.update_args() {
            acc_time += u.dt;
            if acc_time > 1.0 / 60.0 {

                acc_time = 0.0;
                {
                    let mut o = app.obj.first_mut().unwrap();
                    if app.pressed.left {
                        o.x -= o.spd * u.dt;
                    } else if app.pressed.right {
                        o.x += o.spd * u.dt;
                    }
                }
                app.update(&u);

                // if acc_time > 0.2 {
                // acc_time = 0.0;
                // app.duplicate(&mut rng);
                // }
                app.window.set_title(format!("There are {} Gophers Left:{} Right:{}",
                                             app.obj.len(),
                                             app.pressed.left,
                                             app.pressed.right));
            }

        }
        if let Some(r) = e.render_args() {

            app.draw(&e);
        }

    }
}
