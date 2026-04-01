pub mod error;
pub mod rectangle;
pub mod traits;

pub use error::GeometryError;
pub use rectangle::{RectangleData, calculate_rectangle_area};
pub use traits::HasArea;
