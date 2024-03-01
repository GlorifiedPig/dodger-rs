
extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

struct Enemy {
    position: Rect
}

fn main() {
    let enemies: Vec<Enemy> = vec![Enemy{position: Rect::new(0, 0, 32, 32)}];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Dodger", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    let texture_creator = canvas.texture_creator();
    let enemy_texture = texture_creator.load_texture("assets/baddie.png").unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    if let Some(pressed_key) = keycode {
                        match pressed_key {
                            Keycode::Escape => break 'running,
                            _ => {}
                        }
                    }
                },
                Event::Quit {..} => break 'running,
                _ => {}
            }
        }

        for enemy in &enemies {
            canvas.copy(&enemy_texture, None, Some(enemy.position)).unwrap();
        }

        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Exiting game..");
}