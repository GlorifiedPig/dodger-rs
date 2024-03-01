
use rand::prelude::*;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::time::{Duration, Instant};

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
        let rect = Rect::new(position.x.round() as i32, position.y.round() as i32, size.width, size.height);
        Entity { rect, position, size, speed }
    }

    fn update_rect(&mut self) {
        self.rect.set_x(self.position.x.round() as i32);
        self.rect.set_y(self.position.y.round() as i32);
        self.rect.set_width(self.size.width);
        self.rect.set_height(self.size.height);
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
    // Game Config
    let start_time = Instant::now();

    let window_width = 800;
    let window_height = 600;

    let mut player: Entity = Entity::new(
        Position { x: 364.0, y: 500.0 },
        Size { width: 64, height: 64 },
        3.5
    );

    let mut enemies: Vec<Entity> = Vec::new();
    let mut last_enemy_spawn: f32 = 0.0;

    let mut score: u32 = 0;
    let mut score_rect = Rect::new(15, 15, 0, 0);

    let starting_enemies: u32 = 6;

    // SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem.window("Dodger", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    // Font
    let font = ttf_context.load_font("assets/Invasion2000.ttf", 128).unwrap();

    // Textures
    let texture_creator = canvas.texture_creator();
    let player_texture = texture_creator.load_texture("assets/player.png").unwrap();
    let enemy_texture = texture_creator.load_texture("assets/baddie.png").unwrap();

    // Game Loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let runtime: f32 = (Instant::now() - start_time).as_secs_f32();
    
        // Clear Canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Event Handler
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => {}
            }
        }

        // Player
        let mut move_dir = 0.0;

        for scancode in event_pump.keyboard_state().pressed_scancodes().into_iter() {
            match scancode {
                Scancode::Q | Scancode::Escape => break 'running,
                Scancode::A | Scancode::Left => move_dir = -player.speed,
                Scancode::D | Scancode::Right => move_dir = player.speed,
                _ => ()
            }
        }

        player.position.x = player.position.x + move_dir;

        if player.position.x < 0.0 {
            player.position.x = 0.0;
        } else if player.position.x > (window_width - player.size.width) as f32 {
            player.position.x = (window_width - player.size.width) as f32;
        }

        player.update_rect();

        canvas.copy(&player_texture, None, Some(player.rect)).unwrap();

        // Enemies
        let difficulty = (runtime / 5.0).floor().min(20.0) as u32; // difficulty increases every 5 seconds with a maximum of 20.

        if enemies.len() < (starting_enemies + difficulty) as usize && runtime > last_enemy_spawn + 0.65 {
            let random_enemy_size: u32 = rand::thread_rng().gen_range(64 + difficulty..=96 + difficulty * 2);

            enemies.push(
                Entity::new(
                    Position {
                        x: rand::random::<f32>() * ((window_width - random_enemy_size) as f32),
                        y: -(random_enemy_size as f32)
                    },
                    Size {
                        width: random_enemy_size,
                        height: random_enemy_size
                    },
                    (2.4 + difficulty as f32 / 10.0) + (rand::random::<f32>() * (1.2 + difficulty as f32 / 10.0))
                )
            );

            last_enemy_spawn = runtime;
        }

        for enemy in &mut enemies {
            enemy.position.y = enemy.position.y + enemy.speed;
            enemy.update_rect();

            if rects_collide(&enemy.rect, &player.rect) {
                break 'running;
            }

            if enemy.position.y > window_height as f32 {
                score = score + 1;
            }

            canvas.copy(&enemy_texture, None, Some(enemy.rect)).unwrap();
        }

        enemies.retain(|enemy| enemy.position.y <= window_height as f32);

        // Draw Score
        let score_surface = font.render(format!("Score: {}", score).as_str()).blended(Color::WHITE).unwrap();
        let score_texture = texture_creator.create_texture_from_surface(&score_surface).unwrap();
        let TextureQuery { width: score_width, height: score_height, .. } = score_texture.query();
    
        let score_aspect_ratio = score_width / score_height;
        score_rect.set_width(210);
        score_rect.set_height(210 / score_aspect_ratio);

        canvas.copy(&score_texture, None, score_rect).unwrap();

        // Present the canvas & sleep
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Game Over\nScore: {}", score);
}