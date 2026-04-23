pub mod solution {

    use std::fmt::Formatter;
    use std::fmt::Display;
    use std::ops::Add;


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
}





