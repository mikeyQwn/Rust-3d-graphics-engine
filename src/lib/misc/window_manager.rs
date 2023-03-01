extern crate sdl2;

struct ScreenSaverData {
    data: Vec<sdl2::pixels::Color>,
}

pub struct WindowManager {
    // window: sdl2::video::Window,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    screen_saver_data: ScreenSaverData,
}

impl WindowManager {
    pub fn new() -> Self {
        let (window, sdl_context) = Self::get_new_window_and_context();
        let mut canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::WHITE);
        canvas.clear();
        canvas.present();
        let (width, height) = canvas.output_size().expect("Could not get a size");
        let number_of_screens_for_full_cycle = 3;
        let screen_saver_data = ScreenSaverData {
            data: Vec::with_capacity(
                width as usize * height as usize * number_of_screens_for_full_cycle as usize,
            ),
        };

        return Self {
            canvas,
            event_pump,
            screen_saver_data,
        };
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        self.canvas.output_size().expect("Could not get a size")
    }

    pub fn fill_window(&mut self, offset: f64) {
        let (width, _) = self.get_window_size();
        let true_offset = ((offset * 10.0) % (width * 4) as f64) as u32;

        for x in 0..width {
            let color = self.screen_saver_data.data[(x + true_offset) as usize];
            self.canvas.set_draw_color(color);
            self.canvas
                .draw_line(
                    sdl2::rect::Point::new(x as i32, 0),
                    sdl2::rect::Point::new(x as i32, width as i32),
                )
                .unwrap();
        }
    }

    pub fn fill_screen_saver_data(&mut self) {
        let (width, _) = self.get_window_size();
        let f64_width = width as f64;
        for x in 0..(width * 5) {
            let x_normalized = x as f64 / f64_width % 1.0;
            let color = match ((x / width) as f64).floor() as i32 {
                0 => sdl2::pixels::Color::RGB(
                    (x_normalized * 255.0) as u8,
                    0,
                    (255.0 - x_normalized * 255.0) as u8,
                ),
                1 => sdl2::pixels::Color::RGB(255, (x_normalized * 255.0) as u8, 0),
                2 => sdl2::pixels::Color::RGB(
                    (255.0 - x_normalized * 255.0) as u8,
                    255,
                    (x_normalized * 255.0) as u8,
                ),
                3 => sdl2::pixels::Color::RGB(0, (255.0 - x_normalized * 255.0) as u8, 255),
                4 => sdl2::pixels::Color::RGB(
                    (x_normalized * 255.0) as u8,
                    0,
                    (255.0 - x_normalized * 255.0) as u8,
                ),
                5 => sdl2::pixels::Color::RGB(
                    (x_normalized * 255.0) as u8,
                    0,
                    (255.0 - x_normalized * 255.0) as u8,
                ),
                _ => panic!("X index out of bounds in fill_screen_saver_data"),
            };
            self.screen_saver_data.data.push(color);
        }
    }

    pub fn update_window(&mut self, is_running: &mut bool) -> () {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    *is_running = false;
                }
                _ => {}
            }
        }
        self.canvas.present();
    }

    fn get_new_window_and_context() -> (sdl2::video::Window, sdl2::Sdl) {
        let aspect_ratio = 16.0 / 9.0;
        let width_px = 900;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "Rust 3d engine",
                width_px,
                (width_px as f64 / aspect_ratio) as u32,
            )
            .build()
            .unwrap();

        (window, sdl_context)
    }
}
