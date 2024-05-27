use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use std::time::Duration;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

struct Size {
    width: u32,
    height: u32,
}

enum EntityType {
    Turret,
    Conveyor,
    Bullet,
    Enemy,
}

struct Game {
    screen_size: Size,
    camera_position: Position,
    positions: Vec<Position>,
    entity_types: Vec<EntityType>,
}

impl Game {
    fn new() -> Game {
        Game {
            screen_size: Size { width: 0, height: 0 },
            camera_position: Position::new(0, 0),
            positions: Vec::new(),
            entity_types: Vec::new(),
        }
    }

    fn add_entity(&mut self, position: Position, entity_type: EntityType) {
        self.positions.push(position);
        self.entity_types.push(entity_type);
    }

    fn screen_to_world(&self, screen_position: &Position) -> Position {
        let sx = screen_position.x;
        let sy = screen_position.y;

        Position {
            // lb - left bottom ;; c - camera ;; w - screen width ;; h - screen height ;; wo - world
            // lbx = cx - w/2
            // wox = lbx + sx
            x: self.camera_position.x - (self.screen_size.width as i32 / 2) + sx,
            // lby = cy - h/2
            // h - sy -> on screen, higher pixel has lower y.
            // woy = lby + (h - sy) = cy + h/2 - sy 
            y: self.camera_position.y + (self.screen_size.height as i32 / 2) + sy,
        }
    }

    fn world_to_screen(&self, world_position: &Position) -> Position {
        let wx = world_position.x;
        let wy = world_position.y;

        Position {
            x: wx - self.camera_position.x + (self.screen_size.width as i32 / 2),
            y: wy - self.camera_position.y - (self.screen_size.height as i32 / 2),
        }
    }
}

fn render(canvas: &mut WindowCanvas, texture: &Texture, game: &Game) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    for i in 0..game.positions.len() {
        let position = &game.positions[i];
        let entity_type = &game.entity_types[i];

        let screen_position = game.world_to_screen(position);
        let screen_position = Point::new(screen_position.x, screen_position.y);
        let screen_rect = Rect::from_center(screen_position, 32, 32);
        let sprite = Rect::new(0, 0, 32, 32);

        canvas.copy(texture, sprite, screen_rect)?;
    }

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("Defend Factory", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/test-tile.png")?;

    let mut game = Game::new();

    let (width, height) = canvas.output_size()?;
    game.screen_size = Size { width, height, };

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    println!("Mouse x: {}, y: {} mouse_btn: {:?}", x, y, mouse_btn);

                    let world_position = game.screen_to_world(&Position::new(x, y));

                    game.add_entity(world_position, EntityType::Turret);
                },
                _ => {}
            }
        }

        render(&mut canvas, &texture, &game)?;
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
