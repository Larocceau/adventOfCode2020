pub fn tree_at(map: &Vec<Vec<char>>, x: &usize, y: &usize) -> bool {
    let row = &map[*y];
    row[x % row.len()] == '#'
}

pub fn points(dx: &usize, map_height: &usize) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let mut y = 0;

    while y < *map_height {
        points.push((y * dx, y));
        y += 1;
    }

    println!("points: {points:?}");
    points
}

pub fn solve_part_1(map: &Vec<Vec<char>>, dx: &usize) {
    let trees = points(dx, &map.len())
        .iter()
        .filter(|(x, y)| tree_at(map, x, y))
        .count();

    println!("You will pass {trees} trees")
}
