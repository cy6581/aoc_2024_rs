/// # Commentary
///
/// Other than brute force, I was thinking of how to apply Dynamic Programming to this problem,
/// given that there are repeated sub-problems and there exists a recursive relationship
/// between the problems. However, I struggled think of an optimal 2-D DP grid, short of trying
/// every single possible starting number from 0 to a very high number which would cover all the
/// cases. In this case, top-down memoization seemed easier, even though I dislike it as it the
/// memoization happens in a haphazard fashion rather than in a sequential fashion. Also, using a
/// HashMap to memoize has a performance penalty as compared to using a Vec (which only works if
/// you proceed in a sequential fashion). Anyhow, this was what I came up with.
///
/// # Things I tried and learnt
use std::collections::HashMap;

fn solve_part_one(nums: &[u64]) -> u64 {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    nums.iter()
        .map(|&num| count_pebbles_recursive(num, 25, &mut cache))
        .sum()
}

fn solve_part_two(nums: &[u64]) -> u64 {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    nums.iter()
        .map(|&num| count_pebbles_recursive(num, 75, &mut cache))
        .sum()
}

fn count_pebbles_recursive(num: u64, level: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    let num_str = num.to_string();

    if level == 1 {
        if num == 0 {
            1
        } else if num_str.len() % 2 == 0 {
            2
        } else {
            1
        }
    } else {
        // TODO mention this, technically using tuples as key is not efficient
        if let Some(&result) = cache.get(&(num, level)) {
            result
        } else {
            let result = if num == 0 {
                count_pebbles_recursive(1, level - 1, cache)
            } else if num_str.len() % 2 == 0 {
                let mid = num_str.len() / 2;
                // TODO check this syntax
                let left_num = (&num_str[..mid]).parse::<u64>().unwrap();
                let right_num = (&num_str[mid..]).parse::<u64>().unwrap();
                count_pebbles_recursive(left_num, level - 1, cache)
                    + count_pebbles_recursive(right_num, level - 1, cache)
            } else {
                count_pebbles_recursive(num * 2024, level - 1, cache)
            };
            cache.insert((num, level), result);
            result
        }
    }
}

fn main() {
    // const INPUT: [u64; 2] = [125, 17]; // test
    const INPUT: [u64; 8] = [28591, 78, 0, 3159881, 4254, 524155, 598, 1];

    let part_one_answer = solve_part_one(&INPUT);

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 220722);

    let part_two_answer = solve_part_two(&INPUT);

    println!("Part two answer: {part_two_answer}");
    assert_eq!(part_two_answer, 261952051690787);
}
