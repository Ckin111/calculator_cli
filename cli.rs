
// mod calculator;

struct Calculator{
    num1: f32,
    num2: f32
}

impl Calculator {
    fn new (n1: f32, n2: f32) -> Calculator {
        Calculator {
            num1: n1,
            num2: n2
        }
    }

    fn add(&self) -> f32 {
        let x: f32 = self.num1 + self.num2;
        x
    }
    
    fn subtract(&self) -> f32 {
        let x: f32 = self.num1 - self.num2;
        x
    }

    fn multiply(&self) -> f32 {
        let x: f32 = self.num1 * self.num2;
        x
    }

    fn divide(&self) -> f32 {
        let x: f32 = self.num1 / self.num2;
        x
    }
    
    fn modul(&self) -> f32 {
        let x: f32 = self.num1 % self.num2;
        x
    }
}

pub fn run() {
    let c = Calculator::new(5.0,5.0);
    println!("{}",c.add());
    println!("{}",c.subtract());
    println!("{}",c.multiply());
    println!("{}",c.divide());
    println!("{}",c.modul());
}