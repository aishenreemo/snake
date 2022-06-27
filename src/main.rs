mod amend;
mod listener;
mod display;

pub mod core;

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
                eprintln!("Encountered error while processing input:\n{error_msg:?}");
            }
        }

        // update
        if let Err(error_msg) = amend::update(commands) {
            eprintln!("Encountered error while updating data:\n{error_msg:?}");
        }

        // render
        if let Err(error_msg) = display::render(&mut canvas) {
            eprintln!("Encountered error while rendering canvas:\n{error_msg:?}");
        }

        // 30 fps
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 30));
    }
}
