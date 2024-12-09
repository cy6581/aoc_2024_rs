/// # Commentary
///
/// Part One was fairly straightforward, but I got into trouble trying to refactor my fn for use in
/// part Two. Part One was basically a sliding window (size 2) over all elements, to see if any
/// window violated the rules. It could be easily solved using constructs like .any() or .all(),
/// which I initially wrote. However, I got carried away in Part Two which called for a tolerance
/// of one element being in violation of the rules. I tried to reason that this meant having 1
/// window being in violation, tested some inputs, but failed to realise an important edge case:
/// there could be an input with 1 invalid window, that would still *remain unsafe* even after the
/// removal of 1 element. As an example:
/// [1, 2, 9, 10, 11, 12] -> even though the [2, 9] window is invalid, removing 9 still would not
/// help as it would merely leave another [2, 10] invalid window.
///
/// I was forced to abandon the "tolerance" approach and revert to the most straightforward idea
/// of testing every sub-array with one element removed. You can see the mess I left, and still
/// leave for posterity's sake, in the `is_safe_with_tolerance` fn.
///
/// # Things I tried and learnt
/// - Using the XOR operator to combine 2 mutually exclusive flags, which may help to ferret out
/// strange bugs if the flags are not correctly set, as opposed to using the OR operator.
///
/// - Using the .windows() method on the Slice type. Which will work for anything "slice-able",
/// like arrays and Vecs. It makes it much much easier to generate all possible windows, without
/// worrying about going out of bounds etc.  

fn parse_input(input: &'static str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|parts| parts.map(|s| s.parse::<u32>().unwrap()).collect())
        .collect()
}

// Note: This function was refactored into this strange state as I thought it could be repurposed
// to use with part two, but having a tolerance of 1 invalid window within windows. Unfortunately,
// this failed to consider the edge case where having 1 invalid window would still be unsafe, as
// in
fn is_safe_with_tolerance(row: &[u32], tolerance: usize) -> bool {
    let num_windows = row.len() - 1;
    let num_decreasing = row.windows(2).filter(|w| w[0] > w[1]).count();
    let num_increasing = row.windows(2).filter(|w| w[1] > w[0]).count();
    let num_in_range = row
        .windows(2)
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
    input
        .iter()
        .filter(|row| is_safe_with_tolerance(row, 0))
        .count() as u32
}

fn solve_part_two(input: &Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .filter(|row| is_safe_with_dampener(row))
        .count() as u32
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
