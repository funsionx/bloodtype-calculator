use crate::lang_handler::lang_handler;

pub fn probabilities(lang: &u8, m_bt: u32, f_bt: u32, m_rh: &str, f_rh: &str) {
    lang_handler(
        &lang,
        "Blood type probabilities:",
        "Вероятности группы крови:",
    );

    match (m_bt, f_bt) {
        (1, 1) => println!("I - 100%"),
        (2, 2) => println!("I - 6%\nII - 94%"),
        (3, 3) => println!("I - 6%\nIII - 94%"),
        (4, 4) => println!("II - 25%\nIII - 25%\nIV - 50%"),
        (1, 2) | (2, 1) => println!("I - 25%\nII - 75%"),
        (1, 3) | (3, 1) => println!("I - 25%\nIII - 75%"),
        (1, 4) | (4, 1) => println!("II - 50%\nIII - 50%"),
        (2, 3) | (3, 2) => println!("I - 6%\nII - 19%\nIII - 19%\nIV - 56%"),
        (2, 4) | (4, 2) => println!("II - 50%\nIII - 37%\nIV - 13%"),
        (4, 3) | (3, 4) => println!("II - 37%\nIII - 50%\nIV - 13%"),
        (_, _) => println!("You printed wrong nums"),
    }

    lang_handler(
        &lang,
        "Rhesus factor probabilities:",
        "Вероятности резус-фактора:",
    );

    match (m_rh, f_rh) {
        ("-", "-") => println!("Rh- - 100%"),
        ("+", "-") => println!("Rh+ - 75%\nRh- - 25%"),
        ("-", "+") => println!("Rh+ - 75% !!Might be rhesus-conflict\nRh- - 25%"),
        ("+", "+") => println!("Rh+ - 75%\nRh- - 25%"),
        (_, _) => println!("Errr"),
    }
}
