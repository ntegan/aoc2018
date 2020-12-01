use dialoguer::{theme::ColorfulTheme, Select};
#[allow(dead_code)]
pub fn example() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(selections)
        .interact()
        .unwrap();
    //.items(&selections[..])

    println!("Enjoy your {}!", selections[selection]);
}

pub fn read_until_eof() -> Result<String, Box<dyn std::error::Error>> {
    use std::fmt::Write;
    use std::io;
    let mut input = String::new();
    let mut buf = String::new();

    loop {
        let bytes_read = io::stdin().read_line(&mut input)?;
        if bytes_read == 0 {
            break;
        }
        write!(input, "{}", buf);
    }

    Ok(input)
}
