

pub mod solution {

    use std::fmt::Formatter;
    use std::fmt::Display;
    use std::fmt;


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



    }

    
    impl fmt::Display for ComplexNumber{
        
        fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "{} + {}i", self.real, self.imag)
        }
    }


}





