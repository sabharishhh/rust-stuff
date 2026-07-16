struct Rect {
    width: i32,
    height: i32
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 20
    };

    println!("Area = {}sq.m", rect1.area());
    println!("Perimeter = {}m", rect1.perimeter());
}