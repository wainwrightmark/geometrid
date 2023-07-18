#[cfg(any(test, feature = "glam"))]
pub trait HasCenter {
    /// Get the `Vec2` at the centre of this
    fn get_center(&self, scale: f32) -> glam::f32::Vec2;
}
