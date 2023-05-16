use std::process::exit;

pub fn lang_handler(lang: &u8, first: &str, second: &str) {
    match lang {
        1 => {
            println!("{}", first)
        }
        0 => {
            println!("{}", second)
        }
        _ => {
            println!("The only available languages are russian and english");
            exit(1)
        }
    }
}
