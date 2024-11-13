use glam::*;
use winit::{
    event::{DeviceEvent, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);

        return proj * view;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

pub struct CameraController {
    speed: f32,
    sense: f32,
    alt: bool,
    lmb: bool,
    mmb: bool,
    rmb: bool,
    scroll: f32,
    lmbmotion: (f32, f32),
    mmbmotion: (f32, f32),
    rmbmotion: (f32, f32),
}

impl CameraController {
    pub fn new(speed: f32, sense: f32) -> Self {
        Self {
            speed,
            sense,
            alt: false,
            lmb: false,
            mmb: false,
            rmb: false,
            scroll: 0.,
            lmbmotion: (0., 0.),
            mmbmotion: (0., 0.),
            rmbmotion: (0., 0.),
        }
    }
    pub fn process_window_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = state.is_pressed();
                match button {
                    MouseButton::Left => {
                        self.lmb = is_pressed;
                        true
                    }
                    MouseButton::Middle => {
                        self.mmb = is_pressed;
                        true
                    }
                    MouseButton::Right => {
                        self.rmb = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => match code {
                KeyCode::AltLeft | KeyCode::AltRight => {
                    self.alt = state.is_pressed();
                    true
                }
                _ => false,
            },
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(x, y),
                ..
            } => {
                self.scroll += y;
                true
            }
            _ => false,
        }
    }
    pub fn process_device_event(&mut self, event: &DeviceEvent) -> bool {
        if !self.alt || !(self.lmb || self.rmb || self.mmb) {
            return false;
        }
        match event {
            DeviceEvent::MouseMotion { delta } => {
                if self.lmb {
                    self.lmbmotion.0 += delta.0 as f32;
                    self.lmbmotion.1 += delta.1 as f32;
                }
                if self.mmb {
                    self.mmbmotion.0 += delta.0 as f32;
                    self.mmbmotion.1 += delta.1 as f32;
                }
                if self.rmb {
                    self.rmbmotion.0 += delta.0 as f32;
                    self.rmbmotion.1 += delta.1 as f32;
                    self.scroll += delta.1 as f32 * self.speed;
                }
                if (self.lmb || self.mmb || self.rmb) {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let eye_vec = camera.eye - camera.target;
        if self.scroll != 0.0 {
            camera.eye = camera.target + eye_vec * (-self.scroll).exp();
            self.scroll = 0.0;
        }
        if !self.alt {
            return;
        }
        if self.lmbmotion != (0., 0.) {
            let pitch_vec = eye_vec.cross(Vec3::Z).normalize();
            let pitch = Quat::from_axis_angle(pitch_vec, self.sense * self.lmbmotion.1);
            let yaw = Quat::from_rotation_z(-self.sense * self.lmbmotion.0);
            camera.eye = camera.target + pitch * yaw * eye_vec;
            self.lmbmotion = (0., 0.);
        }
        if self.mmbmotion != (0., 0.) {
            let hor_vec = eye_vec.cross(Vec3::Z).normalize();
            let mut displacement = hor_vec * self.speed * self.mmbmotion.0;

            let ver_vec = hor_vec.cross(eye_vec).normalize();
            displacement += ver_vec * self.speed * self.mmbmotion.1;

            camera.eye += displacement;
            camera.target += displacement;
            self.mmbmotion = (0., 0.);
        }
    }
}
