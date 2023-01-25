#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Location {
    #[cfg(feature = "std")]
        #[inline]
        #[must_use]
        /// The absolute distance to the other point
        /// Requires std
        pub fn distance(&self, other: &Self) -> f32 {
            let dx: f32 = (self.x - other.x).abs();
            let dy: f32 = (self.y - other.y).abs();
            f32::sqrt((dx * dx) + (dy * dy))
        }
        
        #[cfg(feature = "std")]
        #[inline]
        #[must_use]
        /// The angle to the other point, in radians
        /// Requires std
        pub fn angle_to(&self, other: &Self) -> f32 {
            let x_diff = other.x - self.x;
            let y_diff = other.y - self.y;

            (y_diff).atan2(x_diff)
        }
}

pub trait HasLocation {
    fn location(&self, scale: f32) -> Location;
}
