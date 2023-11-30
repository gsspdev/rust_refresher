use std::fmt;

fn main() {

    struct Point {
        x: f64,
        y: f64,
    }

    // Implementation block, all 'Point' associated functions/methods go here
    impl Point {
        // This is an 'associated function' because
        // associated w/ a type, 'Point'.
        fn origin() -> Point { Point { x: 0.0, y: 0.0 } }

        fn new(x: f64, y: f64) -> Point { 
            Point { x: x, y: y }}

        fn diagonal(xy: f64) -> Point {
            Self::new(xy, xy)
        }
    }
    
    struct ColorCube {
        x: f64,
        y: f64,
        z: f64,
        color: String
    }

    impl ColorCube {
        fn new (x: f64, y: f64, z: f64, color: &str) -> ColorCube {
            ColorCube { x: x, y: y, z: z, color: color } 
            }
        fn funky_blue_gen() -> ColorCube {
            ColorCube { x: 3.2, y: 4.6, z: 2.4, color: "funky blue"}
        }
    }

    let funky_blue_color_cube == funky_blue_gen();
    println!("the color cube is *# fUnKy #** with coordinates {}, {}, {} & color {}");

    impl fmt::Display for ColorCube {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}. {}. {}. {})", self.x, self.y, self.z, self.color)
        }
    }

    struct Rect {
        x: f64,
        y: f64,
    }
    
    impl Rect {
    fn new (x: f64, y: f64) -> Rect {
        Rect { x: x, y: y }
    }
    fn gen_square(l: f64) -> Rect {
            Rect { x: l, y: l }
        }
    }

    let square_one = Rect::gen_square(7.2);
    println!("square is {}", square_one);


    impl fmt::Display for Rect {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}. {})", self.x, self.y)
        }
    }



    // Implement Display for Point
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let origin = Point::origin();
    let point_one = Point::new(2.46, 2.14);
    let point_two = Point::diagonal(4.6);

    println!("point_one is {} - point_two is {} - origin is {}", point_one, point_two, origin);
}
