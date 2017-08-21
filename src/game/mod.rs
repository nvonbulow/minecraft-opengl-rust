use glium::{self, glutin, Surface};

use std::thread;
use std::time::{Duration, Instant};

mod camera;
mod renderer;
mod world;

#[derive(Copy, Clone)]
pub struct GameState {
    should_quit: bool,
    camera: camera::Camera,
}

impl GameState {
    fn load() -> GameState {
        GameState {
            should_quit: false,
            camera: camera::Camera::new()
        }
    }
}

enum Action {
    Stop,
    Continue
}

fn start_loop<F>(mut callback: F) where F: FnMut() -> Action {
    let mut accumulator = Duration::new(0,0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            //update the state here ??
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}

pub fn start() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut state = GameState::load();

    state.camera.set_position((0.0, 0.0, -1.0));
    state.camera.set_direction((0.0, 0.0, 1.0));
    let mut renderer = renderer::GameRenderer::new(&display);

    start_loop(|| -> Action {
        let mut frame = display.draw();
        state.camera.update(frame.get_dimensions());
        renderer.render_frame(&mut frame, &mut state);
        frame.finish().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => state.should_quit = true,
                    ev => state.camera.process_input(&ev),
                },
                _ => (),
            }
        });

        match state.should_quit {
            true => Action::Stop,
            false => Action::Continue
        }
    });
}