use regex::Regex;
fn main() {
    println!("Hola amigo");
    //creo las expresiones  regex
    //Antes de toda expresion viene la letra r  
    let re_add=Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_sub=Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mul=Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    //solicito datos
    println!("Por favor introduzca su expresion");
    let mut expression=String::new();
    std::io::stdin().read_line(&mut expression).unwrap();


    
    //aplicar operacion de multiplicación
    loop{
        let caps = re_mul.captures(expression.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression=caps.get(0).unwrap().as_str();
        let left_value:i32=caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value:i32=caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value * right_value;

        expression=expression.replace(cap_expression,&addition.to_string());
    }





    //aplicar operacion de resta
    loop{
        let caps = re_sub.captures(expression.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression=caps.get(0).unwrap().as_str();
        let left_value:i32=caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value:i32=caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value - right_value;

        expression=expression.replace(cap_expression,&addition.to_string());
    }




    
    //aplicar operacion de suma
    loop{
        let caps = re_add.captures(expression.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression=caps.get(0).unwrap().as_str();
        let left_value:i32=caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value:i32=caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value + right_value;

        expression=expression.replace(cap_expression,&addition.to_string());
    }
    
    println!("El resultado de tu operacion es : {}",expression);
    
}
