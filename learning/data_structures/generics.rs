struct Point<T> {
    x: T,
    y: T
}

// here x and y can be of type T and T can be anything like int, float, str but both x and y can be only T.
// we can have more types such as T, U, V,... and assign different types to different variables

fn main() {
    let integer_point: Point<i32> = Point {x: 2, y: 5};
    let float_point: Point<f64> = Point {x: 3.4, y: 5.1};

    println!("Integer point: ({} {})", integer_point.x, integer_point.y);
    println!("Float point: ({} {})", float_point.x, float_point.y);
}