//snake
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::Rng;

//directions the snake can go.
#[derive(Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

pub struct Game {
    gl: GlGraphics,
    // OpenGL drawing backend.
    snake: Snake,
    //create a snake struct will will hold the data about our name.
    food: Food,  //the food the snake eats will need to be rendered in the game.
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

        //this will actually draw the snake on the background.
        //Note the order is after the background as been rendered. So this will be on top of the background.
        self.snake.render(&mut self.gl, args);

        //render the food.
        self.food.render(&mut self.gl, args, 10);
    }

    //update the position of the snake.
    fn update(&mut self) {
        self.snake.update();
    }

    //changes the direction of the snake based of button presses.
    // Check that the snake cannot reverse direction. Cannot have the snake going from North to South
    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();
        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            _ => last_direction,
        };
    }
}


pub struct Snake {
    //body: LinkedList<(i32, i32)>,
    position_x: i32,
    position_y: i32,
    dir: Direction,  //Gives the snake the ability to move. Through the direction Enum.
}

//our snake is very similiar to out background, a small square. SO the impl. will be very similiar.
impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        //allows us the use of graphics in this scope.
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            (self.position_x * 20) as f64,
            (self.position_y * 20) as f64,
            20_f64);

        //will call this render method in the main function in order to draw things to the screen.
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(RED, square, transform, gl);
        });
    }
    //update the Direction of the snake.
    fn update(&mut self) {
        match self.dir {
            Direction::UP => self.position_y = self.position_y - 1,
            Direction::DOWN => self.position_y = self.position_y + 1,
            Direction::RIGHT => self.position_x = self.position_x + 1,
            Direction::LEFT => self.position_x = self.position_x - 1,
        }
    }
}

pub struct Food {
    x: u32,
    y: u32,
}

impl Food {

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        use graphics;

        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(BLACK, square, transform, gl)
        });
    }
    // Return true if snake ate food this update
    fn update(&mut self, s: &Snake) -> bool {
        //update based of if the snake ate da foods.
        return true;
    }
}


fn random() -> u8{
    let mut rng = rand::thread_rng();
    return rng.gen_range(0, 10) as u8;
}

fn main() {
//create an open gl variable first.
    let opengl = OpenGL::V3_2;

//create a new window. 2 parameters a 'size' and a 'string' Size is the size of the window.
    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [400, 400])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

//with just the above code the function will run, then instantly close. Need an 'Event Loop'.

//calls the method to make a box on the screen.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        //initialize the snake struct int he game struvt. 0,0 is the top left of the screen.
        snake: Snake { position_x: 0, position_y: 0, dir: Direction::RIGHT },
        food: Food { x: 1, y: 1 },
    };

//events.next is an enum, essentially checks if something is an event then we can do something based of what it is.
    let mut events = Events::new(EventSettings::new()).ups(8);  //'ups' means updates per second. Restricts the amount of updates the snake can do.
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        // if the even is a render event then we will do something
        if let Some(u) = e.update_args() {
            game.update();
        }

        //need to capture the button press event now.
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {  //note the state
                game.pressed(&k.button);
            }
        }
    }
}