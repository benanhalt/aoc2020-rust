use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("day4.txt").expect("failed to load input");
    let passports: Vec<HashMap<_, _>> = input
        .split("\n\n") // Passport are separated by blank lines
        .map(|passport| {
            passport
                .split_whitespace() // Fields are separated by ws.
                .map(|field| {
                    // Fields are of the form key:value.
                    let mut kv = field.split(":");
                    (kv.next().expect("key"), kv.next().expect("value"))
                })
                .collect()
        })
        .collect();

    // Print count of passports that have the required keys.
    println!(
        "part 1: {}",
        passports.iter().filter(has_required_keys).count()
    );

    // Print count of passports that are fully valid.
    println!("part 2: {}", passports.iter().filter(is_valid).count());
}

/// Does a passport have the required keys?
fn has_required_keys(passport: &&HashMap<&str, &str>) -> bool {
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_keys.iter().all(|key| passport.contains_key(key))
}

/// Do all the fields in the passport pass all the validation rules
/// and does the passport have all the required keys?
fn is_valid(passport: &&HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&key, &value)| match key {
        "byr" => match value.parse() {
            // field must have a year in the given range as the value
            Ok(year) => (1920 <= year) && (year <= 2002),
            Err(_) => false,
        },
        "iyr" => match value.parse() {
            // field must have a year in the given range as the value
            Ok(year) => (2010 <= year) && (year <= 2020),
            Err(_) => false,
        },
        "eyr" => match value.parse() {
            // field must have a year in the given range as the value
            Ok(year) => (2020 <= year) && (year <= 2030),
            Err(_) => false,
        },
        "hgt" => {
            // Field must consist of some digits followed by either
            // "cm" or "in" as the units. The digits represent an
            // integer value that must be in the given range according
            // to the units.
            let units: String = value.chars().skip_while(|c| c.is_ascii_digit()).collect();
            let v: i32 = value
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            match units.as_str() {
                "cm" => 150 <= v && v <= 193,
                "in" => 59 <= v && v <= 76,
                _ => false,
            }
        }
        "hcl" => {
            value.starts_with("#")
                && value
                    .strip_prefix("#")
                    .unwrap()
                    .chars()
                    .all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        _ => false,
    }) && has_required_keys(passport)
}
