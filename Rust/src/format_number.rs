pub fn padding_number_to_3_digits(number: i32) -> String {
    if number < 0 {
        return format!("-{}", padding_number_to_3_digits(number.abs()));
    }
    let numeric_str = if number == 0 {
        "000".to_string()
    } else if number < 10 {
        format!("00{}", number)
    } else if number < 100 {
        format!("0{}", number)
    } else {
        number.to_string()
    };
    numeric_str
}

pub fn format_number_kilo_by_kilo(number: i64) -> String {
    if number == 0 {
        return "0".to_string();
    } else if number < 0 {
        return format!("-{}", format_number_kilo_by_kilo(-number));
    }
    
    let mut number = number;
    let mut formatted_str = String::new();
    let mut count = 0;
    
    while number > 0 {
        let chunk = number % 1000;
        if number >= 1000 {
            formatted_str = format!(",{:03}", chunk) + &formatted_str;
        } else {
            formatted_str = chunk.to_string() + &formatted_str;
        }
        number /= 1000;
        count += 1;
    }
    
    formatted_str
}