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
/// Processes a drift sequence (newline separated drifts) where each drift
/// value is positive or negative, e.g. `+1` or `-2`.
fn process_frequency_drift_sequence(sequence: String) -> Result<i64, Box<dyn std::error::Error>> {
    let mut total = 0i64;

    const FIRST_CHAR: usize = 0;
    const DRIFT_VALUE_START: usize = 1;

    for line in sequence.lines() {
        let first_char = line.chars().nth(FIRST_CHAR).ok_or("Empty Line")?;
        let slice = &line[DRIFT_VALUE_START..];
        let drift_value: i64 = slice.parse::<i64>()?;

        match first_char {
            '+' => {
                total = total + drift_value;
            }
            '-' => {
                total = total - drift_value;
            }
            _ => {
                return Err(Box::new(MyError));
            }
        }
    }
    //return Err(Box::new(MyError));
    Ok(total)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //input::example();

    let input = input::read_until_eof()?;

    let result = process_frequency_drift_sequence(input)?;

    println!("answer: {}", result);

    Ok(())
}
