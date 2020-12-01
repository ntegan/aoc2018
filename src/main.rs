mod input;
mod myerror;

//// Day 4
// on the walls, someone has been writing down the ID of the one guard on duty
// every night at midnight for the past few months.
// elves decided one guard is enough for the night shift.
// they've also decided to write down when they fall asleep or wake up while
// at their post (puzzle input).

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //input::example();

    let input = input::read_until_eof()?;

    let claim_string_list: Vec<&str> = input.lines().collect();



    Ok(())
}
