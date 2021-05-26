mod world;

pub use world::Element;
pub use world::World;

use rand::prelude::*;
use sdl2::{keyboard::Keycode, mouse};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{event::Event};

use std::convert::TryFrom;
use std::error::Error;
use std::time::Duration;

const WORLD_WIDTH: i32 = 60;
const WORLD_HEIGHT: i32 = 60;
const PIXEL_SIZE: i32 = 10;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let mut world = World::new(WORLD_WIDTH, WORLD_HEIGHT);

    for _i in 0..800 {
        let x = rng.gen_range(0..WORLD_WIDTH);
        let y = rng.gen_range(0..WORLD_HEIGHT);
        *world.get_element_mut(x, y) = Some(Element::Dust);
    }

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Dust Demo",
            u32::try_from(WORLD_WIDTH * PIXEL_SIZE).unwrap(),
            u32::try_from(WORLD_WIDTH * PIXEL_SIZE).unwrap(),
        )
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let mouse_state = event_pump.mouse_state();
        canvas.draw_rect(Rect::new(
            mouse_state.x() - (PIXEL_SIZE / 2),
            mouse_state.y() - (PIXEL_SIZE / 2),
            u32::try_from(PIXEL_SIZE).unwrap(),
            u32::try_from(PIXEL_SIZE).unwrap(),
        ))?;

        // If both are pressed, do nothing.
        if mouse_state.left() != mouse_state.right() {
            let affected_x = mouse_state.x() / PIXEL_SIZE;
            let affected_y = WORLD_HEIGHT - (mouse_state.y() / PIXEL_SIZE) - 1;

            if mouse_state.left() {
                *world.get_element_mut(affected_x, affected_y) = Some(Element::Dust);
            }

            if mouse_state.right() {
                *world.get_element_mut(affected_x, affected_y) = None;
            }
        }

        for x in 0..world.width() {
            for y in 0..world.height() {
                if let Some(_element) = world.get_element(x, y) {
                    canvas.set_draw_color(Color::RGB(0xD8, 0xCC, 0xB5));
                    canvas.fill_rect(Rect::new(
                        x * PIXEL_SIZE,
                        (WORLD_HEIGHT - y - 1) * PIXEL_SIZE,
                        u32::try_from(PIXEL_SIZE).unwrap(),
                        u32::try_from(PIXEL_SIZE).unwrap(),
                    ))?;
                }
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        world.tick();
    }

    Ok(())
}
