
extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Rect;
use std::time::Duration;

struct Player {
    position: Rect
}

struct Enemy {
    position: Rect
}

fn main() {
    let mut player: Player = Player {
        position: Rect::new(400, 500, 64, 64)
    };

    let enemies: Vec<Enemy> = vec![
        Enemy{ position: Rect::new(0, 0, 64, 64) }
    ];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Dodger", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    let texture_creator = canvas.texture_creator();
    let enemy_texture = texture_creator.load_texture("assets/baddie.png").unwrap();
    let player_texture = texture_creator.load_texture("assets/player.png").unwrap();

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

        for scancode in event_pump.keyboard_state().pressed_scancodes().into_iter() {
            match scancode {
                Scancode::A => player.position.x = player.position.x - 3,
                Scancode::D => player.position.x = player.position.x + 3,
                _ => {}
            }
        }

        for enemy in &enemies {
            canvas.copy(&enemy_texture, None, Some(enemy.position)).unwrap();
        }

        canvas.copy(&player_texture, None, Some(player.position)).unwrap();

        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Exiting game..");
}