extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread::sleep;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};

use sdl2::video::{Window, WindowContext};
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32)
    -> Option<Texture<'a>> {
    if let Ok(mut square_texture) =
        texture_creator.create_texture_target(None, size, size) {
            canvas.with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Green =>
                        texture.set_draw_color(Color::RGB(0, 255, 0)),
                    TextureColor::Blue =>
                        texture.set_draw_color(Color::RGB(0, 0, 255)),
                }
                texture.clear();
            }).expect("[fail] color to texture");
            Some(square_texture)
        } else {
            None
        }
}



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

    let green_square =
        create_texture_rect(&mut canvas,
                            &texture_creator,
                            TextureColor::Green,
                            TEXTURE_SIZE)
        .expect("[fail] color create a texture");


    let blue_square =
        create_texture_rect(&mut canvas,
                            &texture_creator,
                            TextureColor::Blue,
                            TEXTURE_SIZE)
        .expect("[fail] color create a texture");

    let timer = SystemTime::now();

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

        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => {
                true
            }
        };

        let square_texture = if display_green {
            &green_square
        } else {
            &blue_square
        };


        canvas.copy(square_texture,
                    None,
                    Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("[fail] copy texture");
        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    println!("Hello, world!");
}
