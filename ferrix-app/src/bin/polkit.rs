use ferrix_app::{dmi::DMIResult, kernel::KResult};
use std::env;

fn print_error_mode() {
    let data = DMIResult::Error {
        error: "The ferrix-polkit operating mode is not specified!".to_string(),
    };
    println!("{}", data.to_json().unwrap());
}

fn main() {
    let mut args = env::args().skip(1);
    let mode = args.next();

    match mode {
        Some(mode) => {
            if &mode == "dmi" {
                let data = DMIResult::new();
                println!("{}", data.to_json().unwrap());
            } else if &mode == "kmods" {
                let data = KResult::new();
                println!("{}", data.to_json().unwrap());
            } else {
                print_error_mode();
            }
        }
        None => print_error_mode(),
    }
}
