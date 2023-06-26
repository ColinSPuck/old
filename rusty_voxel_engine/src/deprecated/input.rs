// input.rs

use crate::camera::Camera;
use glium::glutin::event::VirtualKeyCode;
use std::collections::HashSet;
use glam::{Vec3, Quat};
use crate::event_manager::{EventManager, CameraEvent, Event};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::clone::Clone;

pub struct InputHandler {
    keys: HashSet<VirtualKeyCode>,
    camera: Arc<Mutex<Camera>>,
    rotation: Quat,
    mouse_sensitivity: f32,
}
impl Clone for InputHandler {
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
            camera: self.camera.clone(),
            rotation: self.rotation,
            mouse_sensitivity: self.mouse_sensitivity,
        }
    }
}
impl fmt::Debug for InputHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InputHandler")
            .field("keys", &self.keys)
            .field("rotation", &self.rotation)
            .field("mouse_sensitivity", &self.mouse_sensitivity)
            .finish()
    }
}
impl InputHandler {
    pub fn new(camera: Camera) -> Self {
        Self {
            keys: HashSet::new(),
            camera: Arc::new(Mutex::new(camera)),
            rotation: Quat::IDENTITY,
            mouse_sensitivity: 0.1,
        }
    }
    pub fn get_camera(&self) -> &Arc<Mutex<Camera>> {
        &self.camera
    }
    
    pub fn key_down(&mut self, key: VirtualKeyCode) {
        self.keys.insert(key);
    }

    pub fn key_up(&mut self, key: VirtualKeyCode) {
        self.keys.remove(&key);
    }

    pub fn handle_mouse_motion(&mut self, delta: &(f64, f64), event_manager: &mut EventManager<InputHandler>) {
        let (x_offset, y_offset) = *delta;
        let x_offset = x_offset as f32 * self.mouse_sensitivity;
        let y_offset = y_offset as f32 * self.mouse_sensitivity;

        let pitch_quaternion = Quat::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), -y_offset.to_radians());
        let yaw_quaternion = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), -x_offset.to_radians());
        self.rotation = yaw_quaternion * self.rotation * pitch_quaternion;
        self.rotation = self.rotation.normalize();

        event_manager.process_event(Arc::new(CameraEvent::UpdateRotation(self.rotation)));
    }
    pub fn handle_mouse_scroll(&mut self, delta: &(f64, f64), event_manager: &mut EventManager<InputHandler>) {
        let (_x_offset, y_offset) = *delta;
        let zoom_delta = y_offset as f32 * self.mouse_sensitivity;
        // Handle zoom
        event_manager.process_event(Arc::new(CameraEvent::UpdateZoom(zoom_delta)));
    }
    
    pub fn handle_keyboard_input(&mut self, dt: f32) -> Vec3 {
        let camera_speed = 2.5 * dt;
        let mut position_delta = Vec3::ZERO;

        if self.keys.contains(&VirtualKeyCode::W) {
            position_delta += camera_speed * Vec3::new(0.0, 0.0, -1.0);
        }
        if self.keys.contains(&VirtualKeyCode::S) {
            position_delta += camera_speed * Vec3::new(0.0, 0.0, 1.0);
        }
        if self.keys.contains(&VirtualKeyCode::A) {
            position_delta += camera_speed * Vec3::new(-1.0, 0.0, 0.0);
        }
        if self.keys.contains(&VirtualKeyCode::D) {
            position_delta += camera_speed * Vec3::new(1.0, 0.0, 0.0);
        }
        if self.keys.contains(&VirtualKeyCode::Space) {
            position_delta += camera_speed * Vec3::new(0.0, 1.0, 0.0);
        }
        if self.keys.contains(&VirtualKeyCode::LControl) {
            position_delta += camera_speed * Vec3::new(0.0, -1.0, 0.0);
        }

        position_delta
    }

    pub fn update_camera(&self, position_delta: &Vec3, rotation_delta: &Quat) -> Vec<Arc<dyn Event + Send + Sync>> {
        let mut events = Vec::new();
        if *position_delta != Vec3::ZERO {
            events.push(Arc::new(CameraEvent::UpdatePosition(position_delta.clone())) as Arc<dyn Event + Send + Sync>);
        }
        if *rotation_delta != Quat::IDENTITY {
            events.push(Arc::new(CameraEvent::UpdateRotation(rotation_delta.clone())) as Arc<dyn Event + Send + Sync>);
        }
        events
    }
}