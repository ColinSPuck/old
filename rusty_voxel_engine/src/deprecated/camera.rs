// camera.rs

use glam::{Quat, Vec3, Mat4};

#[derive(Clone)]
pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
    pub fov: f32,
    pub zoom: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        rotation: Quat,
        fov: f32,
        zoom: f32, 
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            position,
            rotation,
            fov,
            zoom,
            aspect_ratio,
            near,
            far,
        }
    }

    pub fn default() -> Self {
        let position = Vec3::ZERO;
        let rotation = Quat::IDENTITY;
        let fov = 120.0;
        let zoom = 1.0;
        let aspect_ratio = 1024.0 / 768.0;
        let near = 0.1;
        let far = 1000.0;

        Self::new(position, rotation, fov, zoom, aspect_ratio, near, far)
    }

    pub fn view_matrix(&self) -> Mat4 {
        let direction = self.rotation * Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::Y;
        let right = direction.cross(up).normalize();
        let true_up = right.cross(direction).normalize();

        Mat4::look_at_rh(self.position, self.position + direction, true_up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.near,
            self.far,
        )
    }

    pub(crate) fn update_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }
    pub(crate) fn update_position(&mut self, position: Vec3) {
        self.position = position;
    }
    pub(crate) fn update_zoom(&mut self, delta: f32) {
        self.zoom += delta;
        self.zoom = self.zoom.max(0.1).min(10.0);
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }
}
