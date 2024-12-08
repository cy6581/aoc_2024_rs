/// Commentary
///
/// This became an excuse for me to practice my DFS. With the result being an over-engineered
/// crossword word finder.

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_part_one(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let target_s = String::from("XMAS");
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (-1, 0),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            for direction in directions {
                // subtle point - technically we should implement backtracking properly,
                // but just being lazy and using a brand new string for each exploration
                // since backtracking only needed for the initial grid cell, TODO clean
                let mut path_s = String::with_capacity(target_s.len());
                if dfs_match_str(grid, direction, &mut path_s, &target_s, (row, col)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn dfs_match_str(
    grid: &Vec<Vec<char>>,
    nxt_dir: (i32, i32),
    path_s: &mut String,
    target_s: &str,
    cell: (usize, usize),
) -> bool {
    let (r, c) = cell;
    path_s.push(grid[r][c]); // push current char
    if !target_s.starts_with(&path_s[..]) {
        false // no match, terminate
    } else if path_s == target_s {
        true // match found
    } else {
        // although a little sloppy, casting to i32 will be safe since we expect
        // r and c << usize::MAX, might be better to use .try_from()
        let (r, c) = (r as i32, c as i32);
        let (dr, dc) = nxt_dir;
        if r + dr >= 0 && r + dr < grid.len() as i32 && c + dc >= 0 && c + dc < grid[0].len() as i32
        {
            let next_cell = ((r + dr) as usize, (c + dc) as usize);
            return dfs_match_str(grid, nxt_dir, path_s, target_s, next_cell);
        }
        false
    }
}

fn solve_part_two(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    fn is_pair(c1: char, c2: char) -> bool {
        c1 == 'M' && c2 == 'S' || c1 == 'S' && c2 == 'M'
    }

    // exclude the boundary rows and cols to make things easier
    for row in 1..grid.len() - 1 {
        for col in 1..grid[1].len() - 1 {
            if grid[row][col] == 'A' {
                let nw = grid[row - 1][col - 1];
                let sw = grid[row + 1][col - 1];
                let ne = grid[row - 1][col + 1];
                let se = grid[row + 1][col + 1];
                if is_pair(nw, se) && is_pair(ne, sw) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    // see day_01 for the reason to import the text string this way
    // const INPUT: &'static str = include_str!("../input/day_04_test.txt");
    const INPUT: &'static str = include_str!("../input/day_04.txt");

    let grid = parse_input(INPUT);

    let part_one_answer = solve_part_one(&grid);

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 2551);

    let part_two_answer = solve_part_two(&grid);

    println!("Part two answer: {part_two_answer}");
    assert_eq!(part_two_answer, 1985);
}
