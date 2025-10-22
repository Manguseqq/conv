use conv::convert;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Use conv -<type> <value> <to_type>");
        eprintln!("Example: conv -hex A -to_dec ; Output: 10");
        return;
    }
    
    match convert(&args[1], &args[2], &args[3]) {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("{}", err),
    }
}
