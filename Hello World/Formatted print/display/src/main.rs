use std::fmt;

#[derive(Debug)]
pub struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let number = Complex { real: 3.3, imag: 7.2 };
    assert_eq!(format!("{}", number), format!("{} + {}i", number.real, number.imag));
    assert_eq!(format!("{:?}", number), format!("Complex {{ real: {}, imag: {} }}", number.real, number.imag));
}