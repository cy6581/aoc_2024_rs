/// # Commentary
///
/// Relatively fun using Regex. It was nice to have something straightforward after trying to solve
/// this in a single sitting after solving Day 02. I was tempted for a moment to write my own
/// matcher, but thankfully I didn't or I would have been stuck forever on Day 03.
///
/// # Things I tried and learnt
/// - Building a regex with by ignoring whitespace. This just makes it so much more readable. In
/// some circles (e.g. Perl), I believe this is called Regex Extended Mode. Also, this mode ignores
/// inline comments to explain what you are trying to do.
///
/// - The regex crate does not using backtracking! There is a bounded-backtracking flag, but there
/// limits. TODO: read up on the algorithm.
///
/// - Overuse of .fold() is probably a bad idea. I was trying not to fallback to my default of
/// using a for-loop, but the .fold() here to selectively toggle flags while trying to accumulate
/// the result just looks unelegant.
use regex::RegexBuilder;

fn solve_part_one(input: &str) -> u32 {
    let pattern = r"
        mul                 # match string 'mul'
        \(                  # match '('
        (?<uint1>\d{1,3})   # capture first number as `uint1` 
        ,                   # match ','
        (?<uint2>\d{1,3})   # capture second number as `uint2`
        \)                  # match ')'
    ";

    let re = RegexBuilder::new(pattern)
        .ignore_whitespace(true) // allows pattern to be written with multi-line whitespace and comments
        .build()
        .unwrap();

    re.captures_iter(input).fold(0, |sum, captures| {
        let change =
            captures["uint1"].parse::<u32>().unwrap() * captures["uint2"].parse::<u32>().unwrap();
        sum + change
    })
}

fn solve_part_two(input: &str) -> u32 {
    let pattern = r"
        (?<set>do\(\))          # capture as `set`
        |(?<unset>don't\(\))    # capture as `unset`
        |mul                    # match string 'mul'
        \(                      # match '('
        (?<uint1>\d{1,3})       # capture first number as `uint1` 
        ,                       # match ','
        (?<uint2>\d{1,3})       # capture second number as `uint2`
        \)                      # match ')'
    ";

    let re = RegexBuilder::new(pattern)
        .ignore_whitespace(true) // allows pattern to be written with multi-line whitespace and comments
        .build()
        .unwrap();

    let mut enabled = true;

    // TODO .fold() seems a little cumbersome
    re.captures_iter(input).fold(0, |sum, caps| {
        if caps.name("set").is_some() {
            enabled = true;
            sum
        } else if caps.name("unset").is_some() {
            enabled = false;
            sum
        } else if enabled {
            let change =
                caps["uint1"].parse::<u32>().unwrap() * caps["uint2"].parse::<u32>().unwrap();
            sum + change
        } else {
            sum
        }
    })
}

fn main() {
    // see day_01 for the reason to import the text string this way
    const INPUT: &'static str = include_str!("../input/day_03.txt");

    let part_one_answer = solve_part_one(&INPUT);

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 175015740);

    let part_two_answer = solve_part_two(&INPUT);

    println!("Part two answer: {part_two_answer}");
    assert_eq!(part_two_answer, 112272912);
}
