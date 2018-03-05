extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread::sleep;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};

use sdl2::video::{Window, WindowContext};
use std::time::{Duration, SystemTime};

use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};

use std::fs::File;
use std::io::{self, Read, Write};

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn slice_to_string(slice: &[u32]) -> String {
    slice
        .iter()
        .map(|highscore| highscore.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn save_highscores_and_lines(highscores: &[u32], number_of_lines: &[u32]) -> bool {
    let s_highscores = slice_to_string(highscores);
    let s_number_of_lines = slice_to_string(number_of_lines);
    write_into_file(
        &format!("{}\n{}\n", s_highscores, s_number_of_lines),
        "scores.txt",
    ).is_ok()
}

fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(|nb| nb.parse::<u32>().ok())
        .collect()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file("scores.txt") {
        let mut lines = content
            .splitn(2, "\n")
            .map(|line| line_to_slice(line))
            .collect::<Vec<_>>();
        if lines.len() == 2 {
            let number_lines = lines.pop().unwrap();
            let highscores = lines.pop().unwrap();
            Some((highscores, number_lines))
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                    TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
                }
                texture.clear();
            })
            .expect("[fail] color to texture");
        Some(square_texture)
    } else {
        None
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("[sdl] init fail");

    let video_subsystem = sdl_context.video().expect("[sdl] video init fail");

    let title = "tetris";
    let w = 800;
    let h = 600;
    let window = video_subsystem
        .window(title, w, h)
        .position_centered()
        .opengl()
        .build()
        .expect("[sdl] window init fail");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("[sdl] canvas init fail");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("[sdl] event pump init fail");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    const TEXTURE_SIZE: u32 = 32;

    let green_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Green,
        TEXTURE_SIZE,
    ).expect("[fail] color create a texture");

    let blue_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Blue,
        TEXTURE_SIZE,
    ).expect("[fail] color create a texture");

    sdl2::image::init(INIT_PNG | INIT_JPG).expect("[fail] init image context");

    let image_texture = texture_creator
        .load_texture("assets/rust.png")
        .expect("[fail] load image");

    let timer = SystemTime::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => true,
        };

        // let square_texture = if display_green {
        //     &green_square
        // } else {
        //     &blue_square
        // };

        let square_texture = &image_texture;

        canvas
            .copy(
                square_texture,
                None,
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
            )
            .expect("[fail] copy texture");
        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    println!("Hello, world!");
}
