fn parse_input(input: &'static str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| line.split_whitespace())
        .map(|parts| {
            parts
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        }).collect()
}


// Note: This function was refactored into this strange state as I thought it could be repurposed
// to use with part two, but having a tolerance of 1 invalid window within windows. Unfortunately,
// this failed to consider the edge case where having 1 invalid window would still be unsafe, as
// in [1, 2, 9, 10, 11, 12] -> even though the [2, 9] window is invalid, removing 9 still would not
// help as it would merely leave another [2, 10] invalid window. Doesn't seem to be a way to tweak
// the sliding window algorithm. 
// TODO clean up to use .any() and .all() instead to init the booleans directly.
fn is_safe_with_tolerance(row: &[u32], tolerance: usize) -> bool {
    let num_windows = row.len() - 1; 
    let num_decreasing = row.windows(2).filter(|w| w[0] > w[1]).count();
    let num_increasing = row.windows(2).filter(|w| w[1] > w[0]).count();
    let num_in_range = row.windows(2)
        .map(|w| w[0].abs_diff(w[1]))
        .filter(|diff| (1..4).contains(diff)) // 1-3 inclusive
        .count();

    let is_decreasing = num_windows - num_decreasing <= tolerance;
    let is_increasing = num_windows - num_increasing <= tolerance;
    let is_in_range = num_windows - num_in_range <= tolerance;
    // use XOR as more semantically correct, even though both cannot be true
    is_decreasing ^ is_increasing && is_in_range
}

fn is_safe_with_dampener(row: &[u32]) -> bool {
    if is_safe_with_tolerance(row, 0) {
        return true;
    }

    for i in 0..row.len() {
        let mut modified = row.to_vec();
        modified.remove(i);
        if is_safe_with_tolerance(&modified, 0) {
            return true;
        }
    }
    false
}


fn solve_part_one(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().filter(|row| is_safe_with_tolerance(row, 0)).count() as u32
}

fn solve_part_two(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().filter(|row| is_safe_with_dampener(row)).count() as u32
}

fn main() {
    // see day_01 for the reason to import the text string this way
    const INPUT: &'static str = include_str!("../input/day_02.txt");
    
    let input = parse_input(INPUT);

    let part_one_answer = solve_part_one(&input);

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 486);

    let part_two_answer = solve_part_two(&input);

    println!("Part two answer: {part_two_answer}");
    assert_eq!(part_two_answer, 540);
}



