
extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Rect;
use std::time::Duration;

struct Position {
    x: f32,
    y: f32
}

struct Size {
    width: u32,
    height: u32
}

struct Entity {
    position: Position,
    size: Size,
    speed: f32,
    rect: Rect
}

impl Entity {
    fn new(position: Position, size: Size, speed: f32) -> Entity {
        Entity {
            rect: Rect::new(position.x.round() as i32, position.y.round() as i32, size.width, size.height),
            position,
            size,
            speed
        }
    }

    fn get_rect(&mut self) -> Rect {
        self.rect.x = self.position.x.round() as i32;
        self.rect.y = self.position.y.round() as i32;
        self.rect.w = self.size.width as i32;
        self.rect.h = self.size.height as i32;
        self.rect
    }
}

fn rects_collide(a: &Rect, b: &Rect) -> bool {
    let a_left = a.x;
    let a_right = a.x + a.w as i32;
    let a_top = a.y;
    let a_bottom = a.y + a.h as i32;

    let b_left = b.x;
    let b_right = b.x + b.w as i32;
    let b_top = b.y;
    let b_bottom = b.y + b.h as i32;

    a_left < b_right && a_right > b_left && a_top < b_bottom && a_bottom > b_top
}

fn main() {
    let mut player: Entity = Entity::new(
        Position { x: 364.0, y: 500.0 },
        Size { width: 64, height: 64 },
        3.5
    );

    let mut enemies: Vec<Entity> = vec![
        Entity::new(
            Position { x: 364.0, y: 0.0 },
            Size { width: 64, height: 64 },
            2.3
        )
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
        // Clear Canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Event Loop
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    if let Some(pressed_key) = keycode {
                        match pressed_key {
                            Keycode::Escape | Keycode::Q => break 'running,
                            _ => {}
                        }
                    }
                },
                Event::Quit {..} => break 'running,
                _ => {}
            }
        }

        // Player
        let mut move_dir = 0.0;

        for scancode in event_pump.keyboard_state().pressed_scancodes().into_iter() {
            match scancode {
                Scancode::A | Scancode::Left => move_dir = -player.speed,
                Scancode::D | Scancode::Right => move_dir = player.speed,
                _ => {}
            }
        }

        player.position.x = player.position.x + move_dir;

        canvas.copy(&player_texture, None, Some(player.get_rect())).unwrap();

        // Enemies
        for enemy in &mut enemies {
            enemy.position.y = enemy.position.y + enemy.speed;

            if rects_collide(&enemy.get_rect(), &player.get_rect()) {
                println!("Game over.");
                break 'running;
            }

            canvas.copy(&enemy_texture, None, Some(enemy.get_rect())).unwrap();
        }

        // Present the canvas & sleep
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Exiting game..");
}