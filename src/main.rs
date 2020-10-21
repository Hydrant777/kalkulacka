use std::io::{self, Write};
use regex::Regex;

fn main() {
    
    let mut input: String = String::new();
    print!("Zadaj volaco do kalkulatora: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Error reading from STDIN");

    calculus(input);

    fn calculus(input: String) {
      let mut n1: f64 = 0.0;
      let mut op: String = "+".to_string();
      let mut n2: f64 = 0.0;
      let mut result: f64 = 0.0;
      let re: Regex = Regex::new(r"(^\d+)(.)(\d+$)").unwrap();
      for cap in re.captures_iter(&input.trim()) {
        n1 = cap[1].parse().unwrap();
        op = cap[2].to_string();
        n2 = cap[3].parse().unwrap();
//        println!("{} {} {}", n1, op, n2);
      }

      match op.as_str() {
        "+" => result = n1+n2,
        "-" => result = n1-n2,
        "*" => result = n1*n2,
        "/" => result = n1/n2,
        "%" => result = n1%n2,
        _ => println!("Wrong operation (or not implemented)."),
      };

      println!("{} {} {} = {}", n1, op, n2, result);
    }

}
