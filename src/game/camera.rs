use std;

use glium::glutin;

#[derive(Clone, Copy)]
pub struct Camera {
    aspect_ratio: f32,
    position: (f32, f32, f32),
    direction: (f32, f32, f32),

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,

    rotating_left: bool,
    rotating_right: bool,
    rotating_up: bool,
    rotating_down: bool
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1024.0 / 768.0,
            position: (0.0, 0.0, 0.0),
            direction: (0.0, 0.0, 0.0),

            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,

            rotating_left: false,
            rotating_right: false,
            rotating_up: false,
            rotating_down: false
        }
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = std::f32::consts::PI / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0  / (fov / 2.0).tan();

        // note: remember that this is column-major
        [
            [f / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0]
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();

            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s_norm.1 * f.2 - s_norm.2 * f.1,
                 s_norm.2 * f.0 - s_norm.0 * f.2,
                 s_norm.0 * f.1 - s_norm.1 * f.0);

        let p = (-self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
                 -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
                 -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2);

        // note: this is column major
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1, p.2, 1.0]
        ]
    }

    pub fn update(&mut self, dimensions: (u32, u32)) {
        self.aspect_ratio = dimensions.1 as f32 / dimensions.0 as f32;

        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s.1 * f.2 - s.2 * f.1,
                 s.2 * f.0 - s.0 * f.2,
                 s.0 * f.1 - s.1 * f.0);

        if self.moving_up {
            self.position.0 += u.0 * 0.01;
            self.position.1 += u.1 * 0.01;
            self.position.2 += u.2 * 0.01;
        }

        if self.moving_left {
            self.position.0 -= s.0 * 0.01;
            self.position.1 -= s.1 * 0.01;
            self.position.2 -= s.2 * 0.01;
        }

        if self.moving_down {
            self.position.0 -= u.0 * 0.01;
            self.position.1 -= u.1 * 0.01;
            self.position.2 -= u.2 * 0.01;
        }

        if self.moving_right {
            self.position.0 += s.0 * 0.01;
            self.position.1 += s.1 * 0.01;
            self.position.2 += s.2 * 0.01;
        }

        if self.moving_forward {
            self.position.0 += f.0 * 0.01;
            self.position.1 += f.1 * 0.01;
            self.position.2 += f.2 * 0.01;
        }

        if self.moving_backward {
            self.position.0 -= f.0 * 0.01;
            self.position.1 -= f.1 * 0.01;
            self.position.2 -= f.2 * 0.01;
        }

        if self.rotating_up {
            self.direction.0 += u.0 * 0.02;
            self.direction.1 += u.1 * 0.02;
            self.direction.2 += u.2 * 0.02;
        }

        if self.rotating_down {
            self.direction.0 -= u.0 * 0.02;
            self.direction.1 -= u.1 * 0.02;
            self.direction.2 -= u.2 * 0.02;
        }

        if self.rotating_right {
            self.direction.0 += s.0 * 0.02;
            self.direction.1 += s.1 * 0.02;
            self.direction.2 += s.2 * 0.02;
        }

        if self.rotating_left {
            self.direction.0 -= s.0 * 0.02;
            self.direction.1 -= s.1 * 0.02;
            self.direction.2 -= s.2 * 0.02;
        }
    }

    pub fn process_input(&mut self, event: &glutin::WindowEvent) {
        let input = match *event {
            glutin::WindowEvent::KeyboardInput {input, ..} => input,
            _ => return
        };

        let pressed = input.state == glutin::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };
        match key {
            glutin::VirtualKeyCode::Space => self.moving_up = pressed,
            glutin::VirtualKeyCode::LShift => self.moving_down = pressed,
            glutin::VirtualKeyCode::A => self.moving_left = pressed,
            glutin::VirtualKeyCode::D => self.moving_right = pressed,
            glutin::VirtualKeyCode::W => self.moving_forward = pressed,
            glutin::VirtualKeyCode::S => self.moving_backward = pressed,

            glutin::VirtualKeyCode::Up => self.rotating_up = pressed,
            glutin::VirtualKeyCode::Down => self.rotating_down = pressed,
            glutin::VirtualKeyCode::Right => self.rotating_right = pressed,
            glutin::VirtualKeyCode::Left => self.rotating_left = pressed,
            _ => ()
        };
    }
}