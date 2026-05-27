pub mod solution {

    use std::fmt::Formatter;
    use std::fmt::Display;
    use std::hash::Hasher;
    use std::ops::Add;
    use std::ops::AddAssign;
    use std::hash::Hash;

    #[derive(Clone, Copy, Default, Debug)]
    pub struct ComplexNumber{
        real: f64,
        imag: f64
    }

    #[derive(Debug, PartialEq)]
    pub enum ComplexNumberError{
        ImaginaryNotZero
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

        pub fn module(&self) -> f64{
            (self.real.powi(2) + self.imag.powi(2)).sqrt()
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

    // impl From<ComplexNumber> for f64 {
    // fn from(c: ComplexNumber) -> f64 {
    //     if c.imag() != 0.0 {
    //         panic!("ja")
    //     }
    //     c.real()
    // }

    impl TryFrom<ComplexNumber> for f64{
        type Error = ComplexNumberError;

        fn try_from(c: ComplexNumber) -> Result<Self, Self::Error>{
            if c.imag() != 0.0 {
                Err(ComplexNumberError::ImaginaryNotZero)
            }
            else {
                Ok(c.real())
            }
        }
    }

    impl From<f64> for ComplexNumber {
        fn from (c: f64) -> ComplexNumber{
            ComplexNumber::from_real(c)
        }
    }

    impl PartialEq for ComplexNumber{

        fn eq(&self, other: &Self) -> bool {
            self.real() == other.real() && self.imag() == other.imag()
        }
    }

    impl PartialOrd for ComplexNumber{
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for ComplexNumber{
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.module().total_cmp(&other.module())
        }
    }

    impl Eq for ComplexNumber{}

    impl AsRef<f64> for ComplexNumber {
        fn as_ref(&self) -> &f64 {
            &self.real
        }
    }

    impl AsMut<f64> for ComplexNumber {
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    impl Hash for ComplexNumber {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_u64(self.real().to_bits());
            state.write_u64(self.imag().to_bits());
        }
    }

}





// use solution::ComplexNumber;
