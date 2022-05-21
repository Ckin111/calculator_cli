
// struct Calculator{
//     num1: i32,
//     num2: i32
// }

// impl Calculator {
//     fn new (n1: &i32, n2: &i32) -> Calculator {
//         Calculator {
//             num1 = *n1,
//             num2 = *n2
//         }
//     }

//     fn add(&self) -> i32 {
//         let x: i32 = self.num1 + self.num2;
//         x
//     }
// }
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

pub fn run(number1: f32, number2: f32, indicator: char) -> f32 {
    let c = Calculator::new(number1,number2);

    match indicator {
        'a' => c.add(),
        's' => c.subtract(),
        't' => c.multiply(),
        'd' => c.divide(),
        'o' => c.modul(),
        _ => println!("invalid indicator input")
    }
}
