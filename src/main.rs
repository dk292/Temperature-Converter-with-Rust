use std::io;
fn main() {
    println!("Convert the temperature");
    loop {
        println!("Convert from Fahrenheit or Celsius (F/C)");
        let mut temperature_input = String::new();
        io::stdin()
            .read_line(&mut temperature_input)
            .expect("Failde to read the line");
        let temperature_input = temperature_input.trim();
            
        println!("Enter the value");
        let mut input_value = String::new();
        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read the line");
        let input_value: f64 = match input_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        convert_temperature(temperature_input, input_value
        );
        break;
    }
}

fn convert_temperature(temp: &str, input: f64){
    let  output_temp:f64;
    let temp_str = temp.to_uppercase();
    if temp_str == "C" {
        output_temp = (input * (9f64/5f64)) + 32f64;
        println!("The Fahrenheit temperature is {0:.1$}", output_temp, 3);
    }else if temp_str == "F" {
        output_temp = 5f64/9f64 * (input - 32f64);
        println!("The Celsius temperature is {0:.1$}", output_temp, 3);
    }
}