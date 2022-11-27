use std::fmt;

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

fn main() {
    let pi = 3.141592;

    let c = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("My name is {0}, {1} {0}", "Bond", "James",);

    println!("Pi is roughly {:.3}", pi);

    println!("This struct `{}` won't print...", Structure(3));

    println!("{:?}", c);

    println!("{}", c);
}
