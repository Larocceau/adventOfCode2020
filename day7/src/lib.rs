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

pub fn solve_part_1(data: &str) {
    let map: HashMap<&str, Vec<(&str, u32)>> = data.split("\n").map(parse).collect();

    let sg = map.keys().filter(|bag| contains_shiny_gold(bag, &map));

    println!("there are {} bags containing shiny gold", sg.count())
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
