use std::fmt::Debug;

pub fn tree_at(map: &Vec<Vec<char>>, x: &usize, y: &usize) -> bool {
    let row = &map[*y];
    row[x % row.len()] == '#'
}

pub fn points(dx: &usize, dy: &usize, map_height: &usize) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let mut y = 0;

    while y < *map_height {
        points.push((y * dx / dy, y));
        y += dy;
    }
    points
}

fn count_trees_on_path(dx: &usize, dy: &usize, map: &Vec<Vec<char>>) -> usize {
    points(dx, dy, &map.len())
        .iter()
        .filter(|(x, y)| tree_at(map, x, y))
        .count()
}

pub fn solve(map: &Vec<Vec<char>>, paths: &Vec<(usize, usize)>) {
    let final_value = paths
        .iter()
        .map(|(dx, dy)| count_trees_on_path(dx, dy, map))
        .fold(1, |a, b| a * b);

    println!("The product of all trees you pass is {final_value}")
}
