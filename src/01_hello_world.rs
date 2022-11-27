use std::fmt::{self, UpperHex};

#[allow(dead_code)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Structure {{{}}}", self.0)
    }
}
#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0} + {1}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{0}: {1}", count, v)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let red = self.red;

        let blue = self.blue;

        let green = self.green;

        UpperHex::fmt(&red, f)?;
        UpperHex::fmt(&green, f)?;
        UpperHex::fmt(&blue, f)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGB ({red}, {green}, {blue})",
            red = self.red,
            green = self.green,
            blue = self.blue,
        )
    }
}

fn main() {
    let pi = 3.141592;

    let c = Complex {
        real: 3.3,
        imag: 7.2,
    };
    // 1.2
    println!("My name is {0}, {1} {0}", "Bond", "James",);
    // 1.2
    println!("Pi is roughly {:.3}", pi);
    // 1.2
    println!("This struct `{}` won't print...", Structure(3));
    // 1.2.2
    println!("{:?}", c);
    // 1.2.2
    println!("{}", c);
    // 1.2.2.1
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    // 1.2.3
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{} 0x{:0>2X}", *color, *color);
    }
}
