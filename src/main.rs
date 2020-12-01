mod input;
mod myerror;

#[derive(Debug)]
struct StringPair(String, String);

fn get_possible_pairs(box_id_list: Vec<&str>) -> Vec<StringPair> {
    let mut result = Vec::<StringPair>::new();
    for id in box_id_list.iter() {
        for id2 in box_id_list.iter() {
            if *id != *id2 {
                result.push(StringPair(String::from(*id), String::from(*id2)));
            }
        }
    }
    result
}

/// test if a pair of strings differ by exactly n characters
fn string_pair_differs_by_n_characters(pair: &StringPair, n: u64) -> bool {
    let mut number_of_different_characters = 0;

    for char_pair in pair.0.chars().zip(pair.1.chars()) {
        if char_pair.0 != char_pair.1 {
            number_of_different_characters = number_of_different_characters + 1;
        }
    }

    number_of_different_characters == n
}

fn find_pairs_differing_by_n_characters(
    box_id_list: Vec<&str>,
    n: u64,
) -> Result<StringPair, Box<dyn std::error::Error>> {
    assert!(n > 0);

    let possible_pairs = get_possible_pairs(box_id_list);

    for pair in possible_pairs {
        if string_pair_differs_by_n_characters(&pair, n) {
            return Ok(pair);
        }
    }

    return Err(Box::new(myerror::MyError));
}

fn remove_differing_chars(pair: StringPair) -> String {
    let mut result = String::new();
    for i in pair.0.chars().zip(pair.1.chars()) {
        if i.0 == i.1 {
            result.push(i.0);
        }
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //input::example();

    let input = input::read_until_eof()?;

    let box_ids: Vec<&str> = input.lines().collect();

    let result = find_pairs_differing_by_n_characters(box_ids, 1)?;

    let result = remove_differing_chars(result);

    println!("{}", result);

    Ok(())
}
