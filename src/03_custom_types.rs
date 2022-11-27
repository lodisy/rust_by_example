struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle {
            top_left,
            bottom_right,
        } = self;

        let Point { x: x_1, y: y_1 } = top_left;
        let Point { x: x_2, y: y_2 } = bottom_right;

        (x_2 - x_1) * (y_2 - y_1)
    }
}

impl Rectangle {
    fn square(point: Point, width: f32) -> Rectangle {
        let bottom_right_point = Point {
            x: (point.x + width),
            y: (point.y + width),
        };

        Rectangle {
            top_left: point,
            bottom_right: bottom_right_point,
        }
    }
}

fn main() {
    let rec = Rectangle {
        top_left: Point { x: (1.0), y: (2.0) },
        bottom_right: Point { x: (3.0), y: (4.0) },
    };

    let square = Rectangle::square(Point { x: 0.0, y: 0.0 }, 4.0);
    // 3.1.1
    println!("The area of rect is {}.", rec.rect_area());
    // 3.1.2
    println!(
        "The square with width 4.0 should have area of {}.",
        square.rect_area()
    );
}
