use structs::geometry::{GeometryError, HasArea, RectangleData, calculate_rectangle_area};

#[test]
fn rectangle_area_is_calculated_from_public_api() {
    let rectangle = RectangleData::new(10, 20).expect("rectangle should be valid");

    let area = calculate_rectangle_area(&rectangle);
    assert_eq!(area, 200);
}

#[test]
fn rectangle_constructor_returns_error_for_zero_dimension() {
    let result = RectangleData::new(0, 20);

    assert!(matches!(
        result,
        Err(GeometryError::ZeroDimension { field: "width" })
    ));
}

#[test]
fn rectangle_implements_has_area_trait() {
    let rectangle = RectangleData::new(7, 6).expect("rectangle should be valid");

    fn compute_area(shape: &impl HasArea) -> u32 {
        shape.area()
    }

    assert_eq!(compute_area(&rectangle), 42);
}
