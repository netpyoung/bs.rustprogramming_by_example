extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};

fn main() {
    let sdl_context = sdl2::init()
        .expect("[sdl] init fail");

    let video_subsystem = sdl_context.video()
        .expect("[sdl] video init fail");

    let title = "tetris";
    let w = 800;
    let h = 600;
    let window = video_subsystem.window(title, w, h)
        .position_centered()
        .opengl()
        .build()
        .expect("[sdl] window init fail");

    let mut canvas = window.into_canvas()
        .build()
        .expect("[sdl] canvas init fail");

    let mut event_pump = sdl_context.event_pump()
        .expect("[sdl] event pump init fail");



    let texture_creator: TextureCreator<_> =
        canvas.texture_creator();
    const TEXTURE_SIZE: u32 = 32;
    let mut square_texture = texture_creator
        .create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
        .expect("[fail] create texture");
    canvas.with_texture_canvas(&mut square_texture, |texture| {
        texture.set_draw_color(Color::RGB(0, 255, 0));
        texture.clear();
    }).expect("[fail] color a texture");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => { }
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        let rect = Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE);
        canvas.copy(&square_texture, None, rect)
            .expect("[fail] copy texture");
        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    println!("Hello, world!");
}
