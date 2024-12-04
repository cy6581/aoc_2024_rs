use std::collections::HashMap;

fn build_cols(input: &'static str) -> (Vec<u32>, Vec<u32>) {
    input.lines()
        .map(|line| line.split_whitespace())
        .map(|mut cols| (cols.next().unwrap(), cols.next().unwrap()))
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        // Notice the magic here, instead of Vec<(u32, u32)>, Vec of tuples, we .collect() to 
        // (Vec<u32>, Vec<u32>), tuple of Vecs
        .collect()
}


fn solve_part_one(left: &[u32], right: &[u32]) -> u32 {
    left.iter()
    // we can pass a slice directly to .zip() since it implements IntoIterator
        .zip(right) 
        .map(|(&l, &r)| l.abs_diff(r))
        .sum()
}

fn solve_part_two(left: &[u32], right: &[u32]) -> u32 {
    let mut hashmap: HashMap<u32, u32> = HashMap::new();
    for &num in right {
        let count = hashmap.entry(num).or_insert(0);
        *count += 1;
    }
    left.iter()
        .fold(0, |sum, &num| {
            // .get() takes a ref and returns Option<&V>, we need to call .cloned() to get Option<V>
            let count: u32 = hashmap.get(&num).cloned().unwrap_or(0);
            sum + count * num
        })
}

fn main() {
    // TODO: better explain this thingamagic, namely, how does it work to return a string slice
    // and revision on how string slices work. 
    // include_str!() takes a relative path from current file, similar to relative module paths, 
    // since 'input' dir is a sibling to main.rs, we start with 'input'. note that this differs 
    // from how the path structure for std::fs::File methods work, which resolve from CWD iirc. 
    // (TODO check)
    // Even though this loads the entire string into the binary and into memory (TODO check), I 
    // believe this is neglible for 14KB of text...
    const INPUT: &'static str = include_str!("input/day1.txt");
    
    let (mut left, mut right) = build_cols(INPUT);

    // Sorting is necessary for part one and helps for part two, so we can do it here. Use 
    // .sort_unstable() which is usually faster and guaranteed in-place (i.e. will never allocate 
    // auxiliary memory) compared to .sort(). In this case, we don't care that it there is no 
    // stable order to equal elements.
    left.sort_unstable();
    right.sort_unstable();

    let part_one_answer = solve_part_one(&left, &right);

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 2970687);

    let part_two_answer = solve_part_two(&left, &right);

    println!("Part two answer: {part_two_answer}");
    assert_eq!(part_two_answer, 23963899);
}



