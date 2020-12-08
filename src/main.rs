use glutin_window::GlutinWindow as Window;
use graphics;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    left_pos: f64,
    right_pos: f64,
    ball_x: f64,
    ball_y: f64,
    ball_x_dir: f64,
    ball_y_dir: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let bg: graphics::types::Color = [0.0, 0.5, 0.5, 1.0];
        let fg: graphics::types::Color = [0.0, 0.0, 1.0, 1.0];
        let red: graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
        let left_player = graphics::rectangle::square(0.0, self.left_pos, 10.0);
        let right_player = graphics::rectangle::square(290.0, self.right_pos, 10.0);
        let ball = graphics::rectangle::square(self.ball_x, self.ball_y, 10.0);

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(bg, gl);

            let transform = c.transform;

            graphics::rectangle(fg, left_player, transform, gl);
            graphics::rectangle(fg, right_player, transform, gl);
            graphics::rectangle(red, ball, transform, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.ball_y == 290.0 {
            println!("y dir -1: {} {}", self.ball_x, self.ball_y);
            self.ball_y_dir = -1.0;
        } else if self.ball_x == 290.0 {
            println!(
                "x dir -1: {} {} {}",
                self.ball_x, self.ball_y, self.right_pos
            );
            self.ball_x_dir = -1.0;
            if self.right_pos == self.ball_y {
                println!("Hit");
                self.ball_y -= 10.0;
            }
        } else if self.ball_y == 10.0 {
            println!("y dir 1: {} {}", self.ball_x, self.ball_y);
            self.ball_y_dir = 1.0;
        } else if self.ball_x == 10.0 {
            println!("x dir 1: {} {} {}", self.ball_x, self.ball_y, self.left_pos);
            self.ball_x_dir = 1.0;
            if self.left_pos == self.ball_y {
                println!("Hit");
                self.ball_y += 10.0;
            }
        }

        self.ball_x += self.ball_x_dir;
        self.ball_y += self.ball_y_dir;
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W => {
                    self.left_pos -= 1.0;
                }
                Key::S => {
                    self.left_pos += 1.0;
                }
                Key::Up => {
                    self.right_pos -= 1.0;
                }
                Key::Down => {
                    self.right_pos += 1.0;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("pinged-ponged", [300, 300])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        left_pos: 0.0,
        right_pos: 0.0,
        ball_x: 12.0,
        ball_y: 15.0,
        ball_x_dir: 1.0,
        ball_y_dir: 1.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.press_args() {
            app.press(&args);
        }
    }
}
