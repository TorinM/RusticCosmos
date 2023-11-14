/// A 2D Vector
pub struct Vector2D {
    /// The x coordinate of the vector
    pub x: f64,
    /// The y coordinate of the vector
    pub y: f64,
}
impl Vector2D {
    pub fn new(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }
    pub fn zero() -> Vector2D {
        Vector2D { x: 0.0, y: 0.0 }
    }
    pub fn add(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn subtract(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}