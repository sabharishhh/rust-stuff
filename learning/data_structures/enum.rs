enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64)
}

fn calculate_area(shape: Shape) -> f64 {
    let result = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle(width, length) => width * length,
        Shape::Square(side) => side * side
    };
    return result;
}

fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(6.0);
    let rectangle = Shape::Rectangle(7.0, 10.5);

    println!("Area of circle: {}sq.m", calculate_area(circle));
    println!("Area of square: {}sq.m", calculate_area(square));
    println!("Area of rectangle: {}sq.m", calculate_area(rectangle));
}