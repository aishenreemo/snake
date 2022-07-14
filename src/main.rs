use std::time::Duration;

mod fonts;
use fonts::FontManager;

mod game;
use game::Direction;
use game::Game;
use game::Position;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().expect("Could not initialize ttf.");

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

    let mut game = game::init();
    let mut fonts = fonts::init(&ttf_context);

    // traditional game loop
    let mut event_pump = sdl_context.event_pump()?;

    loop {
        // process input
        let mut commands = vec![];
        for event in event_pump.poll_iter() {
            // listen to input events
            listen(&mut commands, event)
        }

        // update game data/info
        update(&mut game, commands);

        // render display based on the info
        if let Err(error_msg) = render(&game, &mut canvas, &mut fonts) {
            eprintln!("Encountered error while rendering canvas:\n{error_msg:?}");
        }

        // 30 fps
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}

enum Command {
    Quit,
    GoUp,
    GoLeft,
    GoRight,
    GoDown,
}

fn listen(commands: &mut Vec<Command>, event: Event) {
    match event {
        Event::Quit { .. } => commands.push(Command::Quit),
        Event::KeyUp { keycode, .. } => listen_on_keyrelease(commands, keycode),
        _ => (),
    }
}

fn listen_on_keyrelease(commands: &mut Vec<Command>, keycode: Option<Keycode>) {
    use Command::*;
    match keycode {
        Some(Keycode::Escape) => commands.push(Quit),
        Some(Keycode::Up) => commands.push(GoUp),
        Some(Keycode::Down) => commands.push(GoDown),
        Some(Keycode::Left) => commands.push(GoLeft),
        Some(Keycode::Right) => commands.push(GoRight),
        Some(Keycode::H) => commands.push(GoLeft),
        Some(Keycode::J) => commands.push(GoDown),
        Some(Keycode::K) => commands.push(GoUp),
        Some(Keycode::L) => commands.push(GoRight),
        Some(Keycode::A) => commands.push(GoLeft),
        Some(Keycode::S) => commands.push(GoDown),
        Some(Keycode::W) => commands.push(GoUp),
        Some(Keycode::D) => commands.push(GoRight),
        _ => (),
    }
}

fn update(game: &mut Game, commands: Vec<Command>) {
    use Direction::*;
    for cmd in commands.into_iter() {
        match cmd {
            Command::Quit => std::process::exit(0),
            Command::GoUp => game.dir_mgr.go(Up),
            Command::GoDown => game.dir_mgr.go(Down),
            Command::GoLeft => game.dir_mgr.go(Left),
            Command::GoRight => game.dir_mgr.go(Right),
        }
    }

    if game.is_position_outside(&game.peek()) {
        game.restart();
        return;
    }

    if game.is_snek_updating() {
        let head = game.snek.get_head();
        let new_pos = game.peek();

        game.dir_mgr.update();

        if head == &new_pos {
            return;
        }

        if game.is_position_onbody(&new_pos) {
            game.restart();
            return;
        }

        game.is_growing = game.is_position_onfood(&new_pos);

        if game.is_growing {
            game.score += 1;
            game.food.relocate(&game.snek, &game.cfg);
        }

        game.snek.update(game.is_growing, new_pos);
    } else {
        game.update_offset();
    }
}

fn render(
    game: &Game,
    canvas: &mut WindowCanvas,
    font_mgr: &mut FontManager,
) -> Result<(), Box<dyn ::std::error::Error>> {
    let size = canvas.output_size()?;

    let window_width = size.0 as f32;
    let window_height = size.1 as f32;

    let padding_percentage = 30.0 / 600.0;
    let game_x = (window_width * padding_percentage).floor() as i32;
    let game_y = (window_height * padding_percentage).floor() as i32;

    let game_length = if window_width > window_height {
        (window_height - game_y as f32 * 2.0) as u32
    } else {
        (window_width - game_x as f32 * 2.0) as u32
    };

    let game_width = game_length;
    let game_height = game_length;

    let cell_width = (game_width as f32 / game.cfg.columns as f32).floor() as u32;
    let cell_height = (game_height as f32 / game.cfg.rows as f32).floor() as u32;

    let game_width = cell_width * game.cfg.columns as u32;
    let game_height = cell_height * game.cfg.rows as u32;

    let game_x = ((window_width - game_width as f32) / 2.0).floor() as i32;
    let game_y = ((window_height - game_height as f32) * 0.3).floor() as i32;

    // clear background
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    // render score
    let texture_creator = canvas.texture_creator();

    let score_color = if game.is_growing {
        Color::WHITE
    } else {
        Color::GRAY
    };

    let monster_bites = font_mgr
        .path("assets/fonts/Monster Bites/Monster Bites.ttf".to_owned())
        .size(32)
        .load()?;

    let score_surface = monster_bites
        .render(&format!("score: {}", game.score))
        .blended(score_color)?;

    let score_texture = texture_creator.create_texture_from_surface(&score_surface)?;

    let score_center = Point::new((window_width * 0.5) as i32, (window_height * 0.9) as i32);
    let score_width = (window_width * 0.3) as u32;
    let score_height = (window_height * 0.05) as u32;

    let score_rect = Rect::from_center(score_center, score_width, score_height);

    canvas.copy(&score_texture, None, score_rect)?;

    // draw game rect
    canvas.set_draw_color(Color::GRAY);
    canvas.draw_rect(Rect::new(game_x, game_y, game_width, game_height))?;

    // render food
    let food_x = (game.food.position.0 as u32 * cell_width) as i32 + game_x;
    let food_y = (game.food.position.1 as u32 * cell_height) as i32 + game_y;

    canvas.set_draw_color(Color::WHITE);
    canvas.fill_rect(Rect::new(food_x, food_y, cell_width, cell_height))?;

    // calculate the direction of each part of the snek
    let mut directions = vec![];
    let body_vec: Vec<Position> = game.snek.body.clone().into();
    for chunk in body_vec.windows(2) {
        let current = chunk[0];
        let next = chunk[1];

        let current_x = current.0 as u32 * cell_width;
        let current_y = current.1 as u32 * cell_height;

        let next_x = next.0 as u32 * cell_width;
        let next_y = next.1 as u32 * cell_height;

        directions.push(match [(current_x, current_y), (next_x, next_y)] {
            _ if game.dir_mgr.current() == &Direction::Idle => Direction::Idle,
            [(x0, _), (x1, _)] if x0 < x1 => Direction::Right,
            [(x0, _), (x1, _)] if x0 > x1 => Direction::Left,
            [(_, y0), (_, y1)] if y0 < y1 => Direction::Down,
            [(_, y0), (_, y1)] if y0 > y1 => Direction::Up,
            _ => Direction::Idle,
        });
    }

    // render snek
    canvas.set_draw_color(Color::YELLOW);

    for (i, part) in game.snek.body.iter().enumerate() {
        let direction = directions.get(i).unwrap_or_else(|| game.dir_mgr.current());
        let mut x = (part.0 as u32 * cell_width) as i32 + game_x;
        let mut y = (part.1 as u32 * cell_height) as i32 + game_y;

        let offset_x = (game.snek.offset * cell_width as f32) as i32;
        let offset_y = (game.snek.offset * cell_height as f32) as i32;

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
