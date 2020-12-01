mod input;
mod myerror;

//// DAY 3
////
// each elf has made a claim about which area would be ideal for santa's suit.
// all claims have an id and consist of a single rectangle with edges
// parallel to the edges of the fabric.
//
// Each claim's rectangle defined as follows:
//  * num inches b/t left edge of fabric and left edge of rectangle
//  * num inches b/t top edge of fabric and top edge of rectangle
//  * width of rect in inches
//  * height of rect in inches
//
// claim like `#123 @ 3,2: 5x4` means claim id 123 represents 3 from left,...
// i.e. it represents the #'s here
// ...........
// ...........
// ...#####...
// ...#####...
// ...#####...
// ...#####...
// ...........
// ...........
// ...........
//
// Problem is that many of claims overlap.
// => 2 or more claims may cover part of the same areas.
//
// if elves proceed with plans, none will have enouh fabric.
// how many square inches of fabric are within two or more claims?

mod fabric {
    use regex::Regex;
    const FABRIC_STRING_REGEX: &'static str = "^#*([0-9]*) @ ([0-9]*),([0-9]*): ([0-9]*)x([0-9]*)$";

    pub struct Point(u64, u64);

    #[derive(Debug)]
    pub struct FabricMap {
        claims: Vec<Claim>,
    }
    impl FabricMap {
        pub fn get_only_intact_claim(&self) -> &Claim {
            let mut remaining_claims = Vec::new();
            for i in 0..(&self.claims).len() {
                remaining_claims.push(&self.claims[i]);
            }

            let width = self.max_horizontal_extent();
            let height = self.max_vertical_extent();
            // horizontal and vertical
            for ho in 0..width + 1 {
                for ve in 0..height + 1 {
                    let mut claims_containing = Vec::new();
                    for i in 0..(&self.claims).len() {
                        let claim = &self.claims[i];
                        if claim.contains_point(Point(ho, ve)) {
                            claims_containing.push(&self.claims[i]);
                        }
                    }
                    if claims_containing.len() > 1 {
                        remaining_claims = remaining_claims
                            .iter()
                            .filter(|f| !claims_containing.contains(f))
                            .map(|f| *f)
                            .collect();
                    }
                }
            }
            assert!(remaining_claims.len() == 1);
            remaining_claims[0]
        }
        pub fn from_claim_list(claim_list: Vec<Claim>) -> FabricMap {
            FabricMap { claims: claim_list }
        }
        pub fn max_horizontal_extent(&self) -> u64 {
            let mut max = 0;
            for claim in &self.claims {
                let claim_extent = claim.max_horizontal_extent();
                if claim_extent > max {
                    max = claim_extent;
                }
            }
            max
        }
        pub fn max_vertical_extent(&self) -> u64 {
            let mut max = 0;
            for claim in &self.claims {
                let claim_extent = claim.max_vertical_extent();
                if claim_extent > max {
                    max = claim_extent;
                }
            }
            max
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Claim {
        id: u64,
        inches_from_left: u64,
        inches_from_top: u64,
        width_inches: u64,
        height_inches: u64,
    }

    impl Claim {
        pub fn contains_point(&self, point: Point) -> bool {
            let ho = point.0;
            let ve = point.1;
            if ho >= self.inches_from_left && ho < (self.inches_from_left + self.width_inches) {
                if ve >= self.inches_from_top && ve < (self.inches_from_top + self.height_inches) {
                    return true;
                }
            }
            false
        }
        /// Returns how far the right most pixel of this claim extends.
        /// e.g.
        /// #1 @ 1,3: 4x4
        /// #2 @ 3,1: 4x4
        /// #3 @ 5,5: 2x2
        /// ........
        /// ...2222.
        /// ...2222.
        /// .11XX22.
        /// .11XX22.
        /// .111133.
        /// .111133.
        /// ........
        ///
        /// plot 1 max horizontal extent is 4
        /// plot 2 max horizontal extent is 6
        ///
        pub fn max_horizontal_extent(&self) -> u64 {
            self.inches_from_left + self.width_inches - 1
        }
        pub fn max_vertical_extent(&self) -> u64 {
            self.inches_from_top + self.height_inches - 1
        }
        pub fn from_string(string: &String) -> Result<Claim, Box<dyn std::error::Error>> {
            let re = Regex::new(FABRIC_STRING_REGEX)?;

            let caps = re
                .captures(string)
                .ok_or("No regex matches; invalid claim string")?;

            let id = caps
                .get(1)
                .ok_or("Couldn't get id group")?
                .as_str()
                .parse::<u64>()?;
            let inches_from_left = caps
                .get(2)
                .ok_or("Couldn't get left in group")?
                .as_str()
                .parse::<u64>()?;
            let inches_from_top = caps
                .get(3)
                .ok_or("Couldn't get top in group")?
                .as_str()
                .parse::<u64>()?;
            let width_inches = caps
                .get(4)
                .ok_or("Couldn't get width group")?
                .as_str()
                .parse::<u64>()?;
            let height_inches = caps
                .get(5)
                .ok_or("Couldn't get height group")?
                .as_str()
                .parse::<u64>()?;

            assert!(width_inches > 0);
            assert!(height_inches > 0);

            Ok(Claim {
                id,
                inches_from_left,
                inches_from_top,
                width_inches,
                height_inches,
            })
        }
    }
}

fn claim_list_from_claim_string_list(
    claim_string_list: Vec<&str>,
) -> Result<Vec<fabric::Claim>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    for claim_string in claim_string_list {
        let claim = fabric::Claim::from_string(&String::from(claim_string))?;
        result.push(claim);
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //input::example();

    let input = input::read_until_eof()?;

    let claim_string_list: Vec<&str> = input.lines().collect();

    let claim_list = claim_list_from_claim_string_list(claim_string_list)?;

    let map = fabric::FabricMap::from_claim_list(claim_list);

    let result = map.get_only_intact_claim();

    println!("{:?}", result);

    Ok(())
}
