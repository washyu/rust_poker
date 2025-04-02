#[cfg(target_os = "windows")]
#[link(name = "SDL2")]
#[link(name = "SDL2_image")]
#[link(name = "SDL2_ttf")]
extern "C" {}

mod cards;
mod game;
mod render;
mod ui;

use game::GameState;
use render::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

fn main() -> Result<(), String>{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust Poker", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Initialize SDL_Image
    sdl2::image::init(sdl2::image::InitFlag::PNG)?;

    let mut renderer = Renderer::new(&texture_creator)?;
    let mut game_state = GameState::new();

    let mut event_pump = sdl_context.event_pump()?;
    let mut last_update = Instant::now();

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                // Process game-specific events
                _ => game_state.handle_event(&event, &mut renderer),
            }
        }

        // Update game state
        let now = Instant::now();
        let dt = now.duration_since(last_update);
        last_update = now;
        game_state.update(dt);

        // Render the game
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 100, 0)); // Set the background color
        canvas.clear();
        renderer.render(&mut canvas, &game_state)?;
        canvas.present();

        // Limit frame rate
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())

}
