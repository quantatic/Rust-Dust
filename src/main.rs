mod world;

use world::Element;
use world::World;

use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::convert::TryFrom;
use std::error::Error;
use std::time::Duration;

const WORLD_WIDTH: i32 = 50;
const WORLD_HEIGHT: i32 = 50;
const PIXEL_SIZE: i32 = 15;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let mut world = World::new(WORLD_WIDTH, WORLD_HEIGHT);
    let mut cursor_size = 1;

    for _ in 0..100 {
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
                Event::KeyUp {
                    keycode: Some(Keycode::Equals),
                    ..
                } => cursor_size += 1,
                Event::KeyUp {
                    keycode: Some(Keycode::Minus),
                    ..
                } => cursor_size = (cursor_size - 1).max(1),
                _ => {}
            }
        }

        /*
         *world.get_element_mut(100, 120) = Some(Element::Dust);
         *world.get_element_mut(50, 45) = Some(Element::Dust);
         */

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mouse_state = event_pump.mouse_state();

        // Box origin is: floor((x - ((cursor_size - 1) / 2)) / PIXEL_SIZE) * PIXEL_SIZE.
        //
        // |                    cursor_size - 1    |
        // | x - (PIXEL_SIZE * (---------------))  |
        // |                           2           | * PIXEL_SIZE
        // | ------------------------------------- |
        // |                PIXEL_SIZE             |
        // ---                                   ---
        //
        // We are working with integer values, so we need to be a bit careful as to when we divide
        // (the equation above assumes floating point division). We rearrange the equation as
        // follows:
        //
        // floor(((2 * x) - (PIXEL_SIZE * (cursor_size - 1))) / (2 * PIXEL_SIZE)) * PIXEL_SIZE.
        //
        // | 2x - (PIXEL_SIZE * (cursor_size - 1)) |
        // | --------------------------------      | * PIXEL_SIZE
        // |        2 * PIXEL_SIZE                 |
        // ---                                   ---
        //
        // We take advantage of the fact that integer division is equivalent to the floor operation
        // to do this without needing to convert to floating point numbers to do this calculation.
        //
        // For the y pixel index, we need to subtract from (world.height() - cursor_size), because
        // mouse coordinates are relative to the top of the screen.
        //
        // These values are the index of the world pixel, not the drawn pixel.
        let mouse_box_x_origin =
            ((mouse_state.x() * 2) - (PIXEL_SIZE * (cursor_size - 1))) / (2 * PIXEL_SIZE);
        let mouse_box_y_origin = world.height()
            - cursor_size
            - (((mouse_state.y() * 2) - (PIXEL_SIZE * (cursor_size - 1))) / (2 * PIXEL_SIZE));

        println!("{}, {}", mouse_box_x_origin, mouse_box_y_origin);

        // If both are pressed, do nothing.
        if mouse_state.left() != mouse_state.right() {
            for delta_y in 0..cursor_size {
                for delta_x in 0..cursor_size {
                    let affected_x = mouse_box_x_origin + delta_x;
                    let affected_y = mouse_box_y_origin + delta_y;

                    if mouse_state.left() {
                        *world.get_element_mut(affected_x, affected_y) = Some(Element::Dust);
                    }

                    if mouse_state.right() {
                        *world.get_element_mut(affected_x, affected_y) = None;
                    }
                }
            }
        }

        for x in 0..world.width() {
            for y in 0..world.height() {
                if let Some(_element) = world.get_element(x, y) {
                    canvas.set_draw_color(Color::RGB(0xD8, 0xCC, 0xB5)); // dust color
                    canvas.fill_rect(Rect::new(
                        x * PIXEL_SIZE,
                        (WORLD_HEIGHT - y - 1) * PIXEL_SIZE,
                        u32::try_from(PIXEL_SIZE).unwrap(),
                        u32::try_from(PIXEL_SIZE).unwrap(),
                    ))?;
                }
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));

        for delta_y in 0..cursor_size {
            for delta_x in 0..cursor_size {
                canvas.draw_rect(Rect::new(
                    (mouse_box_x_origin + delta_x) * PIXEL_SIZE,
                    (world.height() - 1 - mouse_box_y_origin - delta_y) * PIXEL_SIZE,
                    u32::try_from(PIXEL_SIZE).unwrap(),
                    u32::try_from(PIXEL_SIZE).unwrap(),
                ))?;
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 15));

        world.tick();
    }

    Ok(())
}
