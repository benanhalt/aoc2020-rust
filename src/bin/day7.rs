use std::collections::HashSet;
use std::fs;
use std::iter;

#[derive(Debug)]
struct Rule {
    container: String,
    contains: Vec<Contents>,
}

#[derive(Debug)]
struct Contents {
    count: usize,
    item: String,
}

fn main() {
    let rules: Vec<Rule> = fs::read_to_string("day7.txt")
        .expect("failed to load input")
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let container = format!("{} {}", words.next().unwrap(), words.next().unwrap());

            if line.ends_with("no other bags.") {
                Rule {
                    container,
                    contains: Vec::new(),
                }
            } else {
                let contains = line
                    .split("contain")
                    .nth(1)
                    .expect("contained clause")
                    .split(",")
                    .map(|contained| {
                        let mut words = contained.split_whitespace();
                        let (count, adj, color) = (words.next(), words.next(), words.next());
                        Contents {
                            count: count.unwrap().parse().unwrap(),
                            item: format!("{} {}", adj.unwrap(), color.unwrap()),
                        }
                    })
                    .collect();
                Rule {
                    container,
                    contains,
                }
            }
        })
        .collect();

    println!(
        "part 1: {}",
        containers_of(&rules, String::from("shiny gold"))
            .collect::<HashSet<_>>()
            .len()
    );

    println!("part 2: {}", contains(&rules, String::from("shiny gold")));
}

fn containers_of_<'a>(rules: &'a [Rule], bag: String) -> impl Iterator<Item = String> + 'a {
    rules
        .iter()
        .filter(move |rule| rule.contains.iter().any(|contents| *contents.item == bag))
        .map(|rule| rule.container.clone())
}

fn containers_of<'a>(rules: &'a [Rule], bag: String) -> Box<dyn Iterator<Item = String> + 'a> {
    Box::new(containers_of_(rules, bag).flat_map(move |container| {
        iter::once(container.clone()).chain(containers_of(rules, container.clone()))
    }))
}

fn contains_(rules: &[Rule], bag: String) -> &[Contents] {
    match rules.iter().find(|rule| rule.container == bag) {
        Some(rule) => &rule.contains[..],
        None => &[],
    }
}

fn contains(rules: &[Rule], bag: String) -> usize {
    contains_(rules, bag.clone())
        .iter()
        .map(|Contents { count, item }| count + count * contains(rules, item.clone()))
        .sum()
}
