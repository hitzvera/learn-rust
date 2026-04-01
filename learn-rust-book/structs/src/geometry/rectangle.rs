use super::error::GeometryError;
use super::traits::HasArea;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RectangleData {
    width: u32,
    height: u32,
}

impl RectangleData {
    pub fn new(width: u32, height: u32) -> Result<Self, GeometryError> {
        if width == 0 {
            return Err(GeometryError::ZeroDimension { field: "width" });
        }

        if height == 0 {
            return Err(GeometryError::ZeroDimension { field: "height" });
        }

        Ok(Self { width, height })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl HasArea for RectangleData {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn calculate_rectangle_area(rectangle_data: &RectangleData) -> u32 {
    rectangle_data.area()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_rectangle() {
        let rect = RectangleData::new(30, 50).expect("rectangle should be valid");
        assert_eq!(rect.area(), 1500);
    }

    #[test]
    fn rejects_zero_width() {
        let result = RectangleData::new(0, 50);
        assert!(matches!(
            result,
            Err(GeometryError::ZeroDimension { field: "width" })
        ));
    }

    #[test]
    fn can_hold_rectangle() {
        let bigger = RectangleData::new(30, 50).expect("rectangle should be valid");
        let smaller = RectangleData::new(10, 40).expect("rectangle should be valid");
        assert!(bigger.can_hold(&smaller));
    }
}
