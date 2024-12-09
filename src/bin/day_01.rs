/// # Commentary
///
/// This was pretty straightforward.
///
/// # Things I tried and learnt
///
/// One thing I did deliberate on was how best to initialise the HashMap with defaults.
/// The recommended way to use the Rust Entry API is to chain .and_modify() before .or_insert(),
/// like so:
/// `entry(ch).and_modify(|counter| *counter += 1).or_insert(1);`
/// However, I still prefer the logical flow prevalent other languages (Java, Python) where we
/// 1) try to get the entry from the HashMap, failing which, insert a default entry,
/// 2) update said entry.
/// I decided while solving the challenge that this would be my preferred logical flow and stuck to
/// it. It will be my default pattern until I understand the merits of the other way.
/// TODO examine the pros/cons of why Rust Entry API prescribes the first way.
///
/// The second thing was learning the difference between .sort() and .sort_unstable().
/// In general, .sort_unstable() will be faster and guaranteed in-place (i.e. does not allocate
/// auxiliary memory) than .sort(), with the trade off that it may reorder equal elements. This
/// happens because different algorithms are used under the hood. As you might have guessed,
/// .sort() uses a combination of MergeSort and QuickSort, with the former always needing extra
/// memory. Whereas, .sort_unstable() uses only in-place algorithms, namely QuickSort and HeapSort
/// for ver 1.83.
use std::collections::HashMap;

fn build_cols(input: &'static str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut cols| (cols.next().unwrap(), cols.next().unwrap()))
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        // TODO explain the magic of .collect() here, instead of Vec<(u32, u32)>, Vec of tuples,
        // we .collect() to (Vec<u32>, Vec<u32>), tuple of Vecs
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
    left.iter().fold(0, |sum, &num| {
        // .get() takes a ref and returns Option<&V>, we need to call .cloned() to get Option<V>
        let count: u32 = hashmap.get(&num).cloned().unwrap_or(0);
        sum + count * num
    })
}

fn main() {
    // TODO: better explain this thingamagic, namely, how does it work to return a string slice
    // and revision on how string slices work.
    // include_str!() takes a relative path from current file, similar to relative module paths,
    // since 'input' dir is a sibling to the parent /bin, we start with '../input'. note that this
    // differs from how the path structure for std::fs::File methods work, which resolve from CWD
    // iirc (TODO check).
    // Even though this loads the entire string into the binary and into memory (TODO check), I
    // believe this is neglible for 14KB of text...
    const INPUT: &'static str = include_str!("../input/day_01.txt");

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
