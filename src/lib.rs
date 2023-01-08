use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack{
    compartment1: String,
    compartment2: String
}

impl Rucksack{
    fn new(contents: &str) -> Option<Rucksack>{
        match contents.len() % 2 {
            0 => Some(Rucksack{
                compartment1: contents[0..contents.len()/2].to_string(),
                compartment2: contents[contents.len()/2..].to_string()
            }),
            1 => None,
            _ => panic!("Mod 2 (% 2) cannot return anything but 0 or 1"),
        }
    }
}

/// Calculates the priority
/// # Examples
/// ```
/// use std::fs;
/// use advent_of_code_2022_03::calculate_priority;
///
/// let contents = fs::read_to_string("example-input.txt").unwrap();
/// assert_eq!(calculate_priority(&contents), 157);
/// ```
pub fn calculate_priority(input: &String) -> u32 {
    let rucksacks = input.split('\n').filter(|x| !x.is_empty()).map(|x| Rucksack::new(x).unwrap());
    dbg!(rucksacks.collect::<Vec<_>>());
    10
}
