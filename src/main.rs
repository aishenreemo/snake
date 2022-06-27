mod amend {
    use super::Command;
    type AmendError = Box<dyn ::std::error::Error>;

    pub fn update(commands: Vec<Command>) -> Result<(), AmendError> {
        for cmd in commands.into_iter() {
            match cmd {
                Command::Quit => std::process::exit(0),
            }
        }

        Ok(())
    }
}

mod listener {
    use super::Command;

    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    // use sdl2::keyboard::Keycode;
    type ListenerError = Box<dyn ::std::error::Error>;
    type ListenerResult = Result<(), ListenerError>;

    // triggers when you release a key
    fn listen_key_up(commands: &mut Vec<Command>, keycode: Option<Keycode>) -> ListenerResult {
        if let Some(Keycode::Escape) = keycode {
            commands.push(Command::Quit);
        }

        // match keycode {
        //     Some(Keycode::Escape) => commands.push(Command::Quit),
        //     _ => (),
        // }
        Ok(())
    }

    pub fn handle_event(commands: &mut Vec<Command>, event: Event) -> ListenerResult {
        match event {
            Event::Quit { .. } => commands.push(Command::Quit),
            Event::KeyDown { keycode, .. } => listen_key_up(commands, keycode)?,
            _ => (),
        }

        Ok(())
    }
}

mod display {
    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;

    type DisplayError = Box<dyn ::std::error::Error>;

    pub fn render(canvas: &mut WindowCanvas) -> Result<(), DisplayError> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        Ok(())
    }
}

pub enum Command {
    Quit,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("snek geym", 800, 600)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem.");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas.");

    let mut event_pump = sdl_context.event_pump()?;

    loop {
        // process input
        let mut commands = vec![];
        for event in event_pump.poll_iter() {
            if let Err(error_msg) = listener::handle_event(&mut commands, event) {
                eprintln!("{error_msg:?}");
            }
        }

        // update
        if let Err(error_msg) = amend::update(commands) {
            eprintln!("{error_msg:?}");
        }

        // render
        if let Err(error_msg) = display::render(&mut canvas) {
            eprintln!("{error_msg:?}");
        }

        // 30 fps
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 30));
    }
}
