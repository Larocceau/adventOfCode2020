use std::collections::HashMap;

pub fn contains_shiny_gold(bag: &str, map: &HashMap<&str, Vec<(&str, u32)>>) -> bool {
    let inner = map
        .get(bag)
        .expect(&format!("Can not to find {bag} in {map:?}"));

    match inner.as_slice() {
        [] => false,
        content => content
            .iter()
            .any(|(v, _)| *v == "shiny gold" || contains_shiny_gold(v, map)),
    }
}

pub fn count_inner_bags(bag: &str, map: &HashMap<&str, Vec<(&str, u32)>>) -> u32 {
    let mut total = 0;

    for (bag, count) in map.get(bag).expect("That really should exist") {
        println!("Now counting {bag}");
        total += count * (count_inner_bags(bag, map) + 1);
    }
    total
}

pub fn solve_part_1(data: &HashMap<&str, Vec<(&str, u32)>>) {
    let sg = data.keys().filter(|bag| contains_shiny_gold(bag, &data));

    println!("there are {} bags containing shiny gold", sg.count() + 1)
}

pub fn solve_part_2(data: &HashMap<&str, Vec<(&str, u32)>>) {
    let bags_in_shiny_gold = count_inner_bags("shiny gold", data);

    println!("a shiny gold bag contains {bags_in_shiny_gold} bags");
}

pub fn parse<'a>(input: &'a str) -> (&'a str, Vec<(&'a str, u32)>) {
    let (color, rest) = input.split_once(" bags contain ").expect("Missing contain");

    let contents = if rest == "no other bags." {
        Vec::new()
    } else {
        rest.split(", ")
            .map(|v| {
                let (first, rest) = v.split_at(1);
                let (color, _) = rest
                    .rsplit_once(' ')
                    .expect(&format!("did not find final bunch in {first}"));

                let number: u32 = first.parse().expect(&format!(
                    "First character is not a digit {}, rest: {}",
                    first, rest
                ));

                (color.trim(), number)
            })
            .collect()
    };
    (color, contents)
}
