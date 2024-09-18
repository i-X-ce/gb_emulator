mod cartridge;
mod cpu;
mod gpu;
mod instruction;
mod mapper;
mod memory_bus;

use cartridge::Cartridge;
use cpu::CPU;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;
use sdl2::EventPump;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let scale: f32 = 3.0;
    let window = video_subsystem
        .window(
            "Gameboy Emulator",
            (20.0 * 8.0 * scale) as u32,
            (18.0 * 8.0 * scale) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(scale, scale).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 160, 144)
        .unwrap();

    let mut screen_state = [0 as u8; 160 * 3 * 144];

    //...

    let cartridge = Cartridge::new("..//rom//cpu_instrs.gb");
    let mut cpu = CPU::new(cartridge);

    loop {
        cpu.step();
        if cpu.bus.gpu.ly == 144 {
            let mut screen_state = cpu.bus.gpu.frame;
            handle_user_input(&mut event_pump);
            texture.update(None, &screen_state, 160 * 2).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();

            ::std::thread::sleep(std::time::Duration::new(0, 70_000));
        }
    }

    // loop {
    //     handle_user_input(&mut event_pump);
    //     read_screen_state(&mut screen_state);
    //     texture.update(None, &screen_state, 160 * 2).unwrap();
    //     canvas.copy(&texture, None, None).unwrap();
    //     canvas.present();

    //    ::std::thread::sleep(std::time::Duration::new(0, 70_000));
    // }

    // init sdl2
}

fn handle_user_input(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            _ => { /* do nothing */ }
        }
    }
}
