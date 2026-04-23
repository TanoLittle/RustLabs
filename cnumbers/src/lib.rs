pub mod solution {

    use std::fmt::Formatter;
    use std::fmt::Display;
    use std::ops::Add;
    use std::ops::AddAssign;

    #[derive(Clone, Copy, Default)]
    pub struct ComplexNumber{
        real: f64,
        imag: f64
    }

    impl ComplexNumber{
        pub fn new(real: f64, imag: f64) -> Self{
            ComplexNumber{real, imag}
        }

        pub fn real(&self) -> f64{
            self.real 
        }

        pub fn imag(&self) -> f64{
            self.imag 
        }

        pub fn from_real(real: f64) -> Self{
            ComplexNumber::new(real, 0.0)
        }
        
        pub fn to_tuple(&self) -> (f64, f64){
            (self.real, self.imag)
        }




    }

    
    impl Display for ComplexNumber{
        
        fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "{} + {}i", self.real, self.imag)
        }
    }

    impl Add<ComplexNumber> for ComplexNumber{
        type Output = ComplexNumber;
        fn add(self, rhs: ComplexNumber) -> ComplexNumber{
            ComplexNumber { real: self.real + rhs.real, imag: self.imag + rhs.imag }
        }
    }

    impl Add<f64> for ComplexNumber{
        type Output = ComplexNumber;
        fn add(self, rhs: f64) -> ComplexNumber{
            ComplexNumber { real: self.real + rhs, imag: self.imag }
        }
    }

    impl AddAssign for ComplexNumber {
        fn add_assign(&mut self, rhs: ComplexNumber) {
            self.real += rhs.real;
            self.imag += rhs.imag;
        }
    }

    // trait for implementing let c = a + &b
    impl Add<&ComplexNumber> for ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: &ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl Add<&ComplexNumber> for &ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: &ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    // impl From<f64> for ComplexNumber{
    //     fn from(value: f64) -> ComplexNumber {
    //         ComplexNumber::from_real(value)
    //     }
    // }

}



use solution::ComplexNumber;

impl From<ComplexNumber> for f64 {
    fn from(c: ComplexNumber) -> f64 {
        if c.imag() != 0.0 {
            panic!("ja")
        }
        c.real()
    }
}
