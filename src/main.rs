use enigo::*;
use rdev::{listen, Event};

static mut TYPED: Vec<String> = Vec::new();

const NUMS : [&str; 11] = ["0","1","2","3","4","5","6","7","8","9","."];
const SYMBOLS : [&str; 6] = ["+","-","*","/","^","√"];

fn callback(event: Event){
    let result = match event.name {
        Some(string) => string,
        None => String::new()
    };
    if result.is_empty(){
        unsafe {
            if result == "\u{8}" {
                TYPED.pop();
                return;
            }
            TYPED.push(result);
            let mut typedrev = TYPED.clone();
            typedrev.reverse();
            let mut num1 = String::new();
            let mut num2 = String::new();
            let mut symbol = String::new();
            if typedrev[0] != "@" {
                return;
            }
            let typedrev = typedrev[1..].to_vec();
            for item in &typedrev {
                if NUMS.contains(&item.as_str()) {
                    if symbol.is_empty() {
                        if num2.is_empty() && item == "." {
                            num2 += "0";
                        }
                        num2 += item;
                    }
                    else {
                        if num1.is_empty() && item == "." {
                            num1 += "0";
                        }
                        num1 += item;
                    }
                }
                else if SYMBOLS.contains(&item.as_str()) {
                        symbol = item.clone();
                }
                else {
                    break;
                }
            }
            num1 = num1.chars().rev().collect();
            num2 = num2.chars().rev().collect();
            if !num1.is_empty() && !num2.is_empty() && !symbol.is_empty() {
                let answer = match symbol.as_str(){
                    "+" => num1.parse::<f64>().unwrap() + num2.parse::<f64>().unwrap(),
                    "-" => num1.parse::<f64>().unwrap() - num2.parse::<f64>().unwrap(),
                    "*" => num1.parse::<f64>().unwrap() * num2.parse::<f64>().unwrap(),
                    "/" => num1.parse::<f64>().unwrap() / num2.parse::<f64>().unwrap(),
                    "^" => num1.parse::<f64>().unwrap().powf(num2.parse::<f64>().unwrap()),
                    "√" => num2.parse::<f64>().unwrap().powf(1.0/num1.parse::<f64>().unwrap()),
                    _ => 0.0
                };
                TYPED.clear();
                let totallen = num1.len() + num2.len() + symbol.len() + 1;
                let mut enigo = Enigo::new();
                for _ in 0..totallen {
                    enigo.key_sequence_parse("\u{8}");
                }
                enigo.key_sequence_parse(answer.to_string().as_str());
            }
        };
    }
    
}

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}