use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use std::time::Duration;

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

enum EntityType {
    Turret,
    Conveyor,
    Bullet,
    Enemy,
}

struct Game {
    positions: Vec<Position>,
    entity_types: Vec<EntityType>,
}

impl Game {
    fn new() -> Game {
        Game {
            positions: Vec::new(),
            entity_types: Vec::new(),
        }
    }

    fn add_entity(&mut self, position: Position, entity_type: EntityType) {
        self.positions.push(position);
        self.entity_types.push(entity_type);
    }
}

fn render(canvas: &mut WindowCanvas, texture: &Texture, game: &Game) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    // let (width, height) = canvas.output_size()?;

    for i in 0..game.positions.len() {
        let position = &game.positions[i];
        let entity_type = &game.entity_types[i];

        let screen_position = Point::new(position.x, position.y);
        let screen_rect = Rect::from_center(screen_position, 32, 32);
        let sprite = Rect::new(0, 0, 32, 32);

        canvas.copy(texture, sprite, screen_rect)?;
    }

    // let screen_position = Point::new(width as i32 / 2, height as i32 / 2);
    // let screen_rect = Rect::from_center(screen_position, 32, 32);

    // let sprite = Rect::new(0, 0, 32, 32);
    // canvas.copy(texture, sprite, screen_rect)?;

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

                    game.add_entity(Position::new(x, y), EntityType::Turret);
                },
                _ => {}
            }
        }

        render(&mut canvas, &texture, &game)?;
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
