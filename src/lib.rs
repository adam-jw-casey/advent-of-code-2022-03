use std::collections::HashSet;
use itertools::Itertools;

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Clone)]
struct Item{
    val: char
}

impl Item{
    fn new(item: char) -> Option<Self>{
        match item.is_ascii_alphabetic(){
            true => Some(Item{val: item}),
            false => None
        }
    }

    fn priority(&self) -> u32 {
        match self.val.is_ascii_lowercase(){
            true => self.val as u32 - 97 + 1,
            false => self.val as u32 - 65 + 27
        }
    }
}

#[derive(Debug)]
struct Rucksack{
    compartment1: Vec<Item>,
    compartment2: Vec<Item>
}

impl Rucksack{
    fn new(contents: &str) -> Option<Rucksack>{
        match contents.len() % 2 {
            0 => Some(Rucksack{
                compartment1: contents[0..contents.len()/2].chars().map(|x| Item::new(x).unwrap()).collect(),
                compartment2: contents[contents.len()/2..].chars().map(|x| Item::new(x).unwrap()).collect()
            }),
            1 => None,
            _ => panic!("Mod 2 (% 2) cannot return anything but 0 or 1"),
        }
    }

    fn duplicates(&self) -> HashSet<&Item>{
        let set1: HashSet<&Item> = self.compartment1.iter().collect();
        let set2: HashSet<&Item> = self.compartment2.iter().collect();

        set1.intersection(&set2).map(|x| *x).collect()
    }
}

impl  From<&Rucksack> for HashSet<Item>{
    fn from(sack: &Rucksack) -> HashSet<Item> {
        let set1: HashSet<Item> = sack.compartment1.iter().cloned().collect();
        let set2: HashSet<Item> = sack.compartment2.iter().cloned().collect();

        HashSet::from_iter(set1.union(&set2).map(|x| x.clone()))
    }
}

struct Group{
    rucksacks: [Rucksack; 3]
}

impl <'a> Group{
    fn new<T: Iterator<Item = &'a str>>(raw: T) -> Self{
        Self{
            rucksacks: raw
                .filter(|x| !x.is_empty())
                .map(|x| Rucksack::new(x).unwrap())
                .collect::<Vec<_>>().try_into().expect("There should be three rucksacks to unpack.")
        }
    }

    fn find_badge(&self) -> Item{
        let badge_set: HashSet<Item> = 
            HashSet::from(&self.rucksacks[0])
            .intersection(&HashSet::from(&self.rucksacks[1])).map(|x| x.clone()).collect::<HashSet<Item>>()
            .intersection(&HashSet::from(&self.rucksacks[2])).map(|x| x.clone()).collect::<HashSet<Item>>();
        assert_eq!(badge_set.len(), 1);
        badge_set.iter().next().unwrap().clone()
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
    rucksacks
        .map(|rucksack| {
            rucksack
                .duplicates()
                .iter()
                .map(|item| item.priority())
                .sum::<u32>()
        })
        .sum()
}

/// Finds the badges and calculates their priorities
/// # Examples
/// ```
/// use std::fs;
/// use advent_of_code_2022_03::calculate_badge_priority;
///
/// let contents = fs::read_to_string("example-input.txt").unwrap();
/// assert_eq!(calculate_badge_priority(&contents), 70);
/// ```
pub fn calculate_badge_priority(input: &String) -> u32 {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .into_iter().chunks(3).into_iter()
        .map(|x| Group::new(x))
        .map(|group| group.find_badge().priority())
        .sum()
}
