mod input;

// search or two similar box ids.
// puzzle input is list of likely cndidates (after scannign each box).
// count the number of boxes that have an ID containing exactly two of any
// letter, and then separately counting those with eaxactly three of any letter.
// multiply those two counts togetyher to get a rudimentary checksum
// and compare it to what device predicts.

mod myerror {
    use std::fmt;
    #[derive(Debug)]
    pub struct MyError;
    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "There is an error")
        }
    }
    impl std::error::Error for MyError {}
}

//                    return Err(Box::new(myerror::MyError));
fn box_id_contains_letters_repeating_n_times(line: &str, n: u64) -> bool {
    for car in line.chars() {
        if line.matches(car).count() == n as usize {
            return true;
        }
    }
    false
}

fn count_box_ids_that_contain_letters_repeating_n_times(
    id_list: &String,
    n: u64,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut total: u64 = 0;
    for line in id_list.lines() {
        let result = box_id_contains_letters_repeating_n_times(&line, n);
        if result {
            total = total + 1;
        }
    }
    Ok(total)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //input::example();

    let input = input::read_until_eof()?;

    let num_twice = count_box_ids_that_contain_letters_repeating_n_times(&input, 2)?;
    let num_thrice = count_box_ids_that_contain_letters_repeating_n_times(&input, 3)?;

    println!("Num twice: {}", num_twice);
    println!("Num thrice: {}", num_thrice);
    Ok(())
}
