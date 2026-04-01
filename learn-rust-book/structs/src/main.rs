use structs::geometry::{GeometryError, HasArea, RectangleData, calculate_rectangle_area};

fn print_area(shape: &impl HasArea) {
    println!("Shape area: {}", shape.area());
}

fn main() -> Result<(), GeometryError> {
    let rectangle = RectangleData::new(30, 50)?;

    println!("{:#?}", rectangle);
    println!(
        "Rectangle dimensions: width={}, height={}",
        rectangle.width(),
        rectangle.height()
    );

    let area = calculate_rectangle_area(&rectangle);
    println!("The area of the rectangle is {area} square pixels.");
    print_area(&rectangle);

    // we can also use macros dbg like this
    let scale = 2;
    let rect1 = RectangleData::new(dbg!(30 * scale), 50)?;

    dbg!(&rect1); // dbg! takes ownership
    println!(
        "Can first rectangle hold second? {}",
        rectangle.can_hold(&rect1)
    );

    Ok(())
}
