use std::process::exit;
use std::{io, println};

mod calculate;
mod lang_handler;
mod probabilities;

use crate::calculate::calculate;
use crate::lang_handler::lang_handler;
use crate::probabilities::probabilities;
fn main() {
    println!("Welcome! Enter your language (0 is for ru or 1 is for en)");
    let mut lang = String::new();
    io::stdin().read_line(&mut lang).unwrap_or_else(|err| {
        println!("Something bad happened: {err} ");
        exit(1)
    });
    let lang = lang.trim().parse().unwrap();
    lang_handler(
        &lang,
        "Enter mother's blood type and rhesus factor (example: 4 +):",
        "Введите группу крови и резус-фактор матери (например: 4 +)",
    );

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|err| {
        println!("Something bad happened: {err} ");
        exit(1)
    });

    let values = calculate(&input);

    let mothers_bloodtype = values[0].parse().unwrap_or_else(|err| {
        println!("Error: {err}\nYou should print a number according to your blood type");
        exit(1)
    });
    let mothers_rh = values[1];

    lang_handler(
        &lang,
        "Enter father's blood type and rhesus factor (example: 4 +):",
        "Введите группу крови и резус-фактор отца (например: 4 +)",
    );

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap_or_else(|err| {
        println!("Something bad happened: {err} ");
        exit(1)
    });

    let values = calculate(&input2);

    let fathers_bloodtype = values[0].parse().unwrap_or_else(|err| {
        println!("Error: {err}\nYou should print a number according to your blood type");
        exit(1)
    });
    let fathers_rh = values[1];

    if fathers_rh.parse::<i32>().is_ok() | mothers_rh.parse::<i32>().is_ok() {
        println!("Please write + or - for rhesus factor");
        exit(1)
    }

    println!("<--------------------------------------->");

    probabilities(
        &lang,
        mothers_bloodtype,
        fathers_bloodtype,
        mothers_rh,
        fathers_rh,
    )
}
