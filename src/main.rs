use std::io::{self, Write};
use regex::Regex;

fn main() {
    
    let mut input: String = String::new();
    print!("Zadaj volaco do kalkulatora: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Error reading from STDIN");

    calculus("5*3+3"); // 18
    calculus("10+(8-3*2)"); // 12
    calculus("10+(8-3*2*(6-3))"); // 0
    calculus("(2+4)*10+(8-3*2*(6-3))"); // 50
    calculus("(2+4)*10+(8-3*2*(6-3)*(3*6))"); // -256
    calculus("-(3*(2+(4*8*(-2+4*(2+1)))))"); // -966
    calculus("-(3*(2+(4*8*(-2+4*(2+1))))"); // Missing the last parenthesis -> error? :P

    

    // calculus(&input);

}

fn calculate(equation: &str) -> f64 {

  let mut result: f64;
  match equation.parse::<f64>() {
    Ok(n) => result = n,
    Err(_e) => result = 0.0,
  }
  let mut result_string: String = "".into();
  let mut partial = equation.trim().to_string();

  print!(" Calculate: {}", &partial);

  let c_multiply: Regex = Regex::new(r"([\+-]?\d+)\*([\+-]?\d+)").unwrap();
  let c_divide: Regex = Regex::new(r"([\+-]?\d+)/([\+-]?\d+)").unwrap();
  let c_add: Regex = Regex::new(r"([\+-]?\d+)\+([\+-]?\d+)").unwrap();
  let c_substract: Regex = Regex::new(r"([\+-]?\d+)\-([\+-]?\d+)").unwrap();
  let c_doubleplus: Regex = Regex::new(r"(\+{2})").unwrap();
  let c_doubleminus: Regex = Regex::new(r"(-{2})").unwrap();

  loop {
    if c_doubleplus.is_match(&partial) {
      partial = c_doubleplus.replace(&partial, "+").to_string();
      print!(" ++ => {}", &partial);
    } else if c_doubleminus.is_match(&partial) {
      partial = c_doubleminus.replace(&partial, "+").to_string();
      print!(" -- => {}", &partial);
    } else if c_multiply.is_match(&partial) {
      for cap in c_multiply.captures(&partial) {
        result = cap[1].parse::<f64>().unwrap()*cap[2].parse::<f64>().unwrap();
        result_string = "+".into();
        result_string.push_str(result.to_string().as_str());
      }
      partial = c_multiply.replace(&partial, result_string.as_str()).to_string();
      print!(" {} => {}", result, &partial);
    } else if c_divide.is_match(&partial) {
      for cap in c_divide.captures(&partial) {
        result = cap[1].parse::<f64>().unwrap()/cap[2].parse::<f64>().unwrap();
        result_string = "+".into();
        result_string.push_str(result.to_string().as_str());
      }
      partial = c_divide.replace(&partial, result_string.as_str()).to_string();
      print!(" {} => {}", result, &partial);
    } else if c_add.is_match(&partial) {
      for cap in c_add.captures(&partial) {
        result = cap[1].parse::<f64>().unwrap()+cap[2].parse::<f64>().unwrap();
      }
      partial = c_add.replace(&partial, result.to_string().as_str()).to_string();
      print!(" {} => {}", result, &partial);
    } else if c_substract.is_match(&partial) {
      for cap in c_substract.captures(&partial) {
        result = cap[1].parse::<f64>().unwrap()-cap[2].parse::<f64>().unwrap();
      }
      partial = c_substract.replace(&partial, result.to_string().as_str()).to_string();
      print!(" {} => {}", &partial, result);
    } else {
      break;
    }
  }
    io::stdout().flush().unwrap();
    result
}

fn calculus(input: &str) {

  let mut op: String = "+".to_string();
  let mut result: f64;
  let mut output = input.trim().to_string();

  print!("Braces: {}", output);
  let braces: Regex = Regex::new(r"(\([^()]+\))").unwrap();
  loop {
    if braces.is_match(&output) {
      for cap in braces.captures(&output) {
        op = cap[1].to_string();
        // print!(" => {}", op);
      }

      result = calculate(&op);
      output = braces.replace(&output, result.to_string().as_str()).to_string();
      print!(" Result: {}, Output: {}", result, output);
    } else {
      break;
    }
  }

  result = calculate(&output);

  println!(" Input: {} => {}",&output, result);

  // ahaaa, takze nie output bol na vine, ale output_mod stale..  ok, ale ja chcem updatnut tie data, kde mi ukazuje &output
  // cize som dumal, ze takto.. izy :)
  // kua, ale o tom sme uz rozpravali, aj ked.. chuj si pamatam :)
  // hybaj na blabla.. bol by v tom cert, aby sme to nedali
  
  // match op.as_str() {
  //   "+" => result = n1+n2,
  //   "-" => result = n1-n2,
  //   "*" => result = n1*n2,
  //   "/" => result = n1/n2,
  //   "%" => result = n1%n2,
  //   _ => println!("Wrong operation (or not implemented)."),
  // };

}