extern crate chrono;
use std::env;

use chrono::NaiveDateTime;

fn modify_date(str: &str) -> Result<String, String> {
    let fmt = "%Y%m%d-%H%M%S";
    println!("Parsing Date: {}", str);

    let parsed_date= NaiveDateTime::parse_from_str(str, fmt);
    match parsed_date {
        Ok(date) => Ok(date.timestamp_millis().to_string()),
        Err(err) => Err(err.to_string())
    }
}

fn runner(str: &str) -> Result<String, String> {
    match str {
        "-h" | "--help" 
            => Ok("Accepts YYYYMMDD-HHMMSS as an input (eg. 20231012-121200)".to_string()),                 
        _   => modify_date(str)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let _ = match args.len() > 1 {
        true => {
            let value = runner(&args[1]);
            println!("{}", value.unwrap_or_else(|err| { err }))
                
        },
        false => eprintln!("You need to pass in a variable. Use -h for help")        
    };

    ()
}
