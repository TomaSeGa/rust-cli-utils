use std::env;

fn is_binary(s: &str) -> bool 
{
    s.chars().all(|c| c == '0' || c == '1')
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2
    {
        eprintln!("Usage: {} <binary number>", args[0]);
        std::process::exit(1);
    }

    let binary = &args[1];
    
    if !is_binary(binary)
    {
        eprintln!("Error : not a binary\nUsage: {} <binary number>", args[0]);
        return;
    }

    match u32::from_str_radix(binary, 2)
    {
        Ok(n) => {
            println!("binary : {}", binary);
            println!("decimal : {}", n);
            println!("hexadecimal : {:X}", n);
        },
        Err(_) => eprintln!("Error conversion"),
    }
}
