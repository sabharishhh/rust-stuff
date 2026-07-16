struct Rect {
    width: u32,
    height: u32,
}

struct Sqaure {
    side: u32,
}

trait Shape {
    fn shape() -> String;
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Rect {
    fn shape() -> String {
        return String::from("Rectangle");
    }

    fn area(&self) -> u32 {
        return self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}

impl Sqaure {
    fn shape() -> String {
        return String::from("Sqaure");
    }

    fn area(&self) -> u32 {
        return self.side * self.side
    }

    fn perimeter(&self) -> u32 {
        return 4 * self.side;
    }
}

fn get_area_and_perimeter(shape: impl Shape) -> (u32, u32) {
    return (shape.area(), shape.perimeter());
}

fn main() {
    let r = Rect {
        width: 20,
        height: 10,
    };

    let s = Sqaure {
        side: 21,
    };
    println!("Dimensions of {}:", Rect::shape());
    println!("width: {}m; height: {}m", r.width, r.height);

    let (r_area, r_perimeter) = get_area_and_perimeter(r);

    println!("Perimeter of rectangle: {}m", r_perimeter);
    println!("Area of rectangle: {}sq.m", r_area);

    println!();

    println!("Dimensions of {}:", Sqaure::shape());
    println!("side: {}m", s.side);
    println!("Perimeter of sqaure: {}m", s.perimeter());
    println!("Area of sqaure: {}sq.m", s.area());
}
