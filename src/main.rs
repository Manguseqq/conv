use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Use conv -<type> <value> <to_type>");
        eprintln!("Example: conv -hex A -to_dec ; Output: 10");
        return;
    }

    let input_type = &args[1];
    let value = &args[2];
    let output_type = &args[3];

    let num = match input_type.as_str() {
        "-hex" => u64::from_str_radix(value, 16),
        "-dec" => value.parse::<u64>(),
        "-bin" => u64::from_str_radix(value, 2),
        "-oct" => u64::from_str_radix(value, 8),
        _ => {
            eprintln!("Unkown value: {}", input_type);
            return;
        }
    };

    let num = match num {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid value: {}", value);
            return;
        }
    };

    match output_type.as_str() {
        "-to_hex" => println!("{:X}", num),
        "-to_dec" => println!("{}", num),
        "-to_bin" => println!("{:b}", num),
        "-to_oct" => println!("{:o}", num),
        "-to_all" => println!(
            "[{}] Converted to:
        hex: {:X}
        dec: {}
        bin: {:b}
        oct: {:o}",
            value,num, num, num, num
        ),
        _ => eprintln!("Unkown value type: {}", output_type),
    }
}
