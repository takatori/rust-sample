use std::fmt:: { Display, Error, Formatter };


enum Fzb {
    Value(u64),
    Fizz(u64),
    Buzz(u64),
    FizzBuzz(u64),
}

impl Display for Fzb {
    
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        match self {
            &Fzb::FizzBuzz(_) => write!(f, "FizzBuzz"),
            &Fzb::Buzz(_) => write!(f, "Buzz"),
            &Fzb::Fizz(_) => write!(f, "Fizz"),
            &Fzb::Value(n) => write!(f, "{}", n)
        }
    }
}

fn main() {
    for n in (1..100).map(to_fzbz) {
        println!("{}", n);
    }
}


fn to_fzbz(n: u64) -> Fzb {
    match n {
        n if n % 15 == 0 => Fzb::FizzBuzz(n),
        n if n % 5 == 0 => Fzb::Buzz(n),
        n if n % 3 == 0 => Fzb::Fizz(n),
        n => Fzb::Value(n),
    }
}
