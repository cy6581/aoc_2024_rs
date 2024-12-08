/// Commentary
///
/// Instinctively, I reached for DFS. Luckily, it didn't seem to be over-engineered for a variable
/// number of operators because surely enough, in part 2, they asked to add another operator.
///
/// Things I tried and learnt
/// - Using slices is a memory-efficient way of tracking the remaining elements in the array.
/// Really learning to appreciate the Slice API in Rust.
///
/// - Using a `match` statement is an elegant way to handle the various operations. It also helps
/// for a combination check on whether there is any previous result and how to proceed with the
/// operation.
///
/// - The terminal condition for DFS should be when there are no more nums to process, it seems
/// simple but midway I struggled with something more tricky like the length of `results` Vec.
/// I think using a results Vec simplified certain things but threw me off in this way.
/// TODO, I wonder how others modelled this thing.

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    // TODO, tried using .take() to oeprate on the line sections without collecting into Vec<&str>
    // but this proved too unweidly, must be a better way
    // maybe I can split on both ":" and " "
    input
        .lines()
        .map(|line| {
            let sections: Vec<&str> = line.split(":").collect();
            let target = sections[0].parse().unwrap();
            let nums = sections[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (target, nums)
        })
        .collect()
}

enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn concatenate(x: i64, y: i64) -> i64 {
    format!("{}{}", x, y).parse().unwrap() // assume the output won't overflow i64::MAX
}

fn dfs_find_target(
    nums: &[i64],
    operators: &[Operator],
    results: &mut Vec<i64>,
    target: &i64,
) -> bool {
    if nums.len() == 0 {
        results.last().unwrap() == target
    } else {
        for operator in operators {
            let num = *nums
                .first()
                .expect("should have at least one num to operate on");

            let last_res = results.last();
            let next_res = match (operator, last_res) {
                (_, None) => num,
                (Operator::Add, Some(l)) => num + l,
                (Operator::Multiply, Some(l)) => num * l,
                (Operator::Concatenate, Some(l)) => concatenate(*l, num), // don't forget to put in reverse order
            };

            results.push(next_res);
            if dfs_find_target(&nums[1..], operators, results, target) {
                return true; // stop DFS right away
            }
            results.pop(); // backtrack
        }
        false
    }
}

fn solve_part_one(tasks: &Vec<(i64, Vec<i64>)>) -> i64 {
    tasks
        .iter()
        .filter(|(target, nums)| {
            let mut results: Vec<i64> = vec![];
            let operators = [Operator::Add, Operator::Multiply];
            dfs_find_target(&nums[..], &operators, &mut results, target)
        })
        .map(|(target, _)| target)
        .sum()
}

fn solve_part_two(tasks: &Vec<(i64, Vec<i64>)>) -> i64 {
    tasks
        .iter()
        .filter(|(target, nums)| {
            let mut results: Vec<i64> = vec![];
            let operators = [Operator::Add, Operator::Multiply, Operator::Concatenate];
            dfs_find_target(&nums[..], &operators, &mut results, target)
        })
        .map(|(target, _)| target)
        .sum()
}

fn main() {
    // see day_01 for the reason to import the text string this way
    // const INPUT: &'static str = include_str!("../input/day_07_test.txt");
    const INPUT: &'static str = include_str!("../input/day_07.txt");

    let values = parse_input(INPUT);

    let part_one_answer = solve_part_one(&values);

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 3119088655389);

    let part_two_answer = solve_part_two(&values);

    println!("Part two answer: {part_two_answer}");
    assert_eq!(part_two_answer, 264184041398847);
}
