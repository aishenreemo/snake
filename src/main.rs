use std::collections::VecDeque;
use std::time::Duration;
use std::time::SystemTime;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

type GameError = Box<dyn ::std::error::Error>;

fn main() -> Result<(), GameError> {
    let sdl_context = sdl2::init()?;
    // let ttf_context = sdl2::ttf::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("snek geym", 600, 700)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem.");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas.");

    // let font_mgr = core::font::FontManager::new(&ttf_context)?;
    let mut game = Game::init();

    // traditional game loop
    let mut event_pump = sdl_context.event_pump()?;

    loop {
        // process input
        let mut commands = vec![];
        for event in event_pump.poll_iter() {
            if let Err(error_msg) = listen(&game, &mut commands, event) {
                eprintln!("Encountered error while processing input:\n{error_msg:?}");
            }
        }

        // update
        if let Err(error_msg) = update(&mut game, commands) {
            eprintln!("Encountered error while updating data:\n{error_msg:?}");
        }

        // render
        if let Err(error_msg) = render(&game, &mut canvas) {
            eprintln!("Encountered error while rendering canvas:\n{error_msg:?}");
        }

        // 30 fps
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}

struct Game {
    score: u8,
    body: VecDeque<(u8, u8)>,
    food: (u8, u8),
    columns: u8,
    rows: u8,
    directions: VecDeque<Direction>,
    direction: Direction,
    offset: f32,
    last_update: SystemTime,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Idle,
}

enum Command {
    Quit,
    GoUp,
    GoLeft,
    GoRight,
    GoDown,
}

impl Game {
    fn init() -> Self {
        Self {
            score: 0,
            body: VecDeque::from([(3, 3), (4, 3)]),
            food: (10, 10),
            columns: 20,
            rows: 20,
            directions: VecDeque::new(),
            direction: Direction::Idle,
            offset: 0.0,
            last_update: SystemTime::now(),
        }
    }

    fn restart(&mut self) {
        self.score = 0;
        self.body = VecDeque::from([(2, 3), (3, 3)]);
        self.food = (10, 10);
        self.direction = Direction::Idle;
        self.offset = 0.0;
        self.last_update = SystemTime::now();
        self.directions.clear();
    }
}

fn render(game: &Game, canvas: &mut WindowCanvas) -> Result<(), GameError> {
    let size = canvas.output_size()?;

    let window_width = size.0 as f32;
    let padding_percentage = 20.0 / 600.0;
    let padding = (window_width * padding_percentage) as i32;
    let game_x = padding;
    let game_y = padding;

    let game_length = (window_width - padding as f32 * 2.0) as u32;
    let game_width = game_length;
    let game_height = game_length;

    let cell_width = (game_width as f32 / game.columns as f32) as u32;
    let cell_height = (game_height as f32 / game.rows as f32) as u32;

    // clear background
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    // draw game rect
    canvas.set_draw_color(Color::GRAY);
    canvas.draw_rect(Rect::new(game_x, game_y, game_width, game_height))?;

    // render food
    let food_x = (game.food.0 as u32 * cell_width) as i32 + game_x;
    let food_y = (game.food.1 as u32 * cell_height) as i32 + game_y;

    canvas.set_draw_color(Color::RED);
    canvas.fill_rect(Rect::new(food_x, food_y, cell_width, cell_height))?;

    // calculate the direction of each part of the snek
    let mut directions = vec![];
    for chunk in Vec::from(game.body.clone()).windows(2) {
        let current = chunk[0];
        let next = chunk[1];

        let current_x = current.0 as u32 * cell_width;
        let current_y = current.1 as u32 * cell_height;

        let next_x = next.0 as u32 * cell_width;
        let next_y = next.1 as u32 * cell_height;

        directions.push(match [(current_x, current_y), (next_x, next_y)] {
            _ if game.direction == Direction::Idle => Direction::Idle,
            [(x0, _), (x1, _)] if x0 < x1 => Direction::Right,
            [(x0, _), (x1, _)] if x0 > x1 => Direction::Left,
            [(_, y0), (_, y1)] if y0 < y1 => Direction::Down,
            [(_, y0), (_, y1)] if y0 > y1 => Direction::Up,
            _ => Direction::Idle,
        });
    }

    // render snek
    canvas.set_draw_color(Color::GREEN);
    for (i, part) in game.body.iter().enumerate() {
        let direction = directions.get(i).unwrap_or(&game.direction);
        let mut x = (part.0 as u32 * cell_width) as i32 + game_x;
        let mut y = (part.1 as u32 * cell_height) as i32 + game_y;

        let offset_x = (game.offset * cell_width as f32) as i32;
        let offset_y = (game.offset * cell_height as f32) as i32;

        match direction {
            Direction::Right => x += offset_x,
            Direction::Left => x -= offset_x,
            Direction::Down => y += offset_y,
            Direction::Up => y -= offset_y,
            Direction::Idle => (),
        }

        canvas.fill_rect(Rect::new(x, y, cell_width, cell_height))?;
    }

    canvas.present();
    Ok(())
}

fn listen(game: &Game, commands: &mut Vec<Command>, event: Event) -> Result<(), GameError> {
    match event {
        Event::Quit { .. } => commands.push(Command::Quit),
        Event::KeyUp { keycode, .. } => listen_on_keyrelease(game, commands, keycode)?,
        _ => (),
    }

    Ok(())
}

fn listen_on_keyrelease(
    _game: &Game,
    commands: &mut Vec<Command>,
    keycode: Option<Keycode>,
) -> Result<(), GameError> {
    match keycode {
        Some(Keycode::Escape) => commands.push(Command::Quit),
        Some(Keycode::Up) => commands.push(Command::GoUp),
        Some(Keycode::Down) => commands.push(Command::GoDown),
        Some(Keycode::Left) => commands.push(Command::GoLeft),
        Some(Keycode::Right) => commands.push(Command::GoRight),
        Some(Keycode::H) => commands.push(Command::GoLeft),
        Some(Keycode::J) => commands.push(Command::GoDown),
        Some(Keycode::K) => commands.push(Command::GoUp),
        Some(Keycode::L) => commands.push(Command::GoRight),
        _ => (),
    }
    Ok(())
}

fn update(game: &mut Game, commands: Vec<Command>) -> Result<(), GameError> {
    use Direction::*;
    for cmd in commands.into_iter() {
        match cmd {
            Command::Quit => std::process::exit(0),
            Command::GoUp => game.directions.push_back(Up),
            Command::GoDown => game.directions.push_back(Down),
            Command::GoLeft => game.directions.push_back(Left),
            Command::GoRight => game.directions.push_back(Right),
        }
    }

    let speed = Duration::from_millis(200);
    let elapsed = game.last_update.elapsed()?;

    let head = game.body.iter().last().unwrap();
    let head = (head.0 as i16, head.1 as i16);

    let offsets = match game.direction {
        Up => (0, -1),
        Down => (0, 1),
        Left => (-1, 0),
        Right => (1, 0),
        Idle => (0, 0),
    };

    let new_pos = (head.0 + offsets.0, head.1 + offsets.1);

    if !(0..game.columns as i16).contains(&new_pos.0) || !(0..game.rows as i16).contains(&new_pos.1)
    {
        game.restart();
        return Ok(());
    }

    if elapsed >= speed {
        let handle_direction = |v| match (game.direction, v) {
            (Up, Down) | (Down, Up) | (Right, Left) | (Left, Right) => game.direction,
            _ => v,
        };

        game.direction = game
            .directions
            .pop_front()
            .map(handle_direction)
            .unwrap_or(game.direction);

        if head == new_pos {
            return Ok(());
        }

        let new_pos = (new_pos.0 as u8, new_pos.1 as u8);
        if game.body.range(1..).any(|x| x == &new_pos) {
            game.restart();
            return Ok(());
        }

        if game.food == new_pos {
            game.score += 1;
            game.food = (5, 5);
            game.body.push_back(new_pos);
        } else {
            game.body.pop_front();
            game.body.push_back(new_pos);
        }

        game.offset = 0.0;
        game.last_update = SystemTime::now();
    } else {
        game.offset = elapsed.as_secs_f32() / speed.as_secs_f32();
    }

    Ok(())
}
