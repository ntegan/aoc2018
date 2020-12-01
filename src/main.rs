mod input;

// device must be calibrated before first use.
// frequency drict detected. cannot maintain destination lock.
//
// below message device shows a sequence of changes in frequency (puzzle input)
//
// value +6 means frequency increases by 6.
// -3 decrease by 3.
//
// example, +1, -2, +3, +1 with startying at 0 gives 3
use std::fmt;
#[derive(Debug)]
struct MyError;
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error")
    }
}
impl std::error::Error for MyError {}
fn find_first_repeated_frequency(sequence: String) -> Result<i64, Box<dyn std::error::Error>> {
    let mut frequency = 0i64;
    let mut frequencies = Vec::<i64>::new();
    frequencies.push(frequency);

    const FIRST_CHAR: usize = 0;
    const DRIFT_VALUE_START: usize = 1;

    loop {
        for line in sequence.lines() {
            let first_char = line.chars().nth(FIRST_CHAR).ok_or("Empty Line")?;
            let slice = &line[DRIFT_VALUE_START..];
            let drift_value: i64 = slice.parse::<i64>()?;

            match first_char {
                '+' => {
                    frequency = frequency + drift_value;
                }
                '-' => {
                    frequency = frequency - drift_value;
                }
                _ => {
                    return Err(Box::new(MyError));
                }
            }

            if frequencies.contains(&frequency) {
                return Ok(frequency);
            }
            frequencies.push(frequency);
        }
    }
    eprintln!("No dice");
    return Err(Box::new(MyError));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //input::example();

    let input = input::read_until_eof()?;

    let result = find_first_repeated_frequency(input)?;

    println!("answer: {}", result);

    Ok(())
}
