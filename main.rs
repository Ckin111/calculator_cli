mod cli;
mod calculator;

pub fn main() {
    cli::run();
    let num1: f32 = 5.0;
    let num2: f32 = 6.0;
    let c = calculator::run(num1,num2,'a');

}