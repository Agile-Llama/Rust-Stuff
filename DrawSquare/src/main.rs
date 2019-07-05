//snake
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        //allows us the use of graphics in this scope.
        use graphics::*;
        // this creates the background of the square/shape 4 fields are Red, Green, Blue and Opacity.
        //each must be between 0 and 1.
        // const GREEN: [f32; 4] = [245.0, 191.0, 66.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];

        //will call this render method in the main function in order to draw things to the screen.
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(GREEN, gl);
        });
    }
}

fn main() {
//create an open gl variable first.
    let opengl = OpenGL::V3_2;

    //create a new window. 2 parameters a 'size' and a 'string' Size is the size of the window.
    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [200, 200])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    //with just the above code the function will run, then instantly close. Need an 'Event Loop'.

    //calls the method to make a box on the screen.
    let mut game = Game{
        gl: GlGraphics::new(opengl),
    };

    //events.next is an enum, essentially checks if something is an event then we can do something based of what it is.
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        // if the even is a render event then we will do somethinbg
        if let Some(u) = e.update_args() {
            //game.update(&u);
        }
    }
}