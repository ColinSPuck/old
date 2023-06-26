//util.rs
pub use glam::UVec3;

#[macro_export]
macro_rules! const_uvec3 {
    ($values:expr) => {
        glam::UVec3::new($values[0], $values[1], $values[2])
    };
}