pub fn convert(input_type: &str, value: &str, output_type: &str) -> Result<String, String> {
    let num = match input_type {
        "-hex" => u64::from_str_radix(value, 16),
        "-dec" => value.parse::<u64>(),
        "-bin" => u64::from_str_radix(value, 2),
        "-oct" => u64::from_str_radix(value, 8),
        _ => return Err(format!("Unknown input type: {}", input_type)),
    };

    let num = num.map_err(|_| format!("Invalid value: {}", value))?;

    let result = match output_type {
        "-to_hex" => format!("{:X}", num),
        "-to_dec" => format!("{}", num),
        "-to_bin" => format!("{:b}", num),
        "-to_oct" => format!("{:o}", num),
        "-to_all" => format!(
            "[{}] Converted to:\n  hex: {:X}\n  dec: {}\n  bin: {:b}\n  oct: {:o}",
            value, num, num, num, num
        ),
        _ => return Err(format!("Unknown output type: {}", output_type)),
    };

    Ok(result)
}
