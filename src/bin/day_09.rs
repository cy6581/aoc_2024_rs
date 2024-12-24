/// # Commentary
///
/// My first thought was to build a vector representing the blocks, including the empty space. Then,
/// I would use the two pointer technique and process the blocks from front and back, swapping items
/// as needed. The tradeoff is that it is space inefficient, as every "empty" slot would occupy
/// memory that would eventually be redundant. To optimize this, I thought of:
/// 1) maintaining counters to know which items have been processed instead of adding empty slots.
/// (I.e. instead of the data structure representing empty slots for you, you use an abstraction to
/// track it numerically and only append items with ID to the vector). This would be a lot more
/// tedious though, as need variables to compare each block of empty space in the front, to each
/// block of numbers in the back, trying to fit them, calculating the difference where there is a
/// mismatch, and saving the difference for the subsequent set of blocks to be processed. I decided
/// against this eventually.
/// 2) using a different data structure, which would allow me to save memory on empty slots. It
/// crossed my mind to use a doubly-linked list, with certain nodes representing free space as a
/// variable in its data. It would also be faster to build than a vector, since I coulld append any
/// number of elements to the back without triggering memory reallocation (which would be a problem
/// in this case given that I cannot pre-allocate the vector size without knowing concretely the
/// number of items). However, after reading up on the LinkedList that comes with Rust, I was
/// disappointed to find that it not allow me to append or remove nodes easily from the middle of
/// the list, which was exactly what I wanted. Also, there doesn't seem to be a way to iterate from
/// the back of the list? Which means I cannot implement the two-pointer technique.
/// Finally, it seemed like I was back to the initial "naive" solution.
///
/// # Things I tried and learnt
/// - How to read a single line of text. I was using lines().take(1) in conjunction with .flat_map()
/// to flatten an iterator of lines, with only 1 line, into a single unit and apply .map() to
/// extract the chars from the unit. If this sounds like a mouthful, it truly is, because, to my
/// horror, I realised how silly of me not to directly call .chars() method on the original &str.
/// Urgh!
///
/// - Iterator operators. In addition over-engineering the lines() iterator as mentioned, I also
/// experimented with various iterators to compute the checksum. I debated between using .filter(),
/// or its nicer cousin .filter_map() will automatically unwrap the Option<u32> that I have here.
/// The downside to both of them though is that they will iterate through the entire Vec, even
/// though we know that after some point all which remain are None (empty slots). So in this case,
/// .take_while() is more efficient because it stops at the first None, ignoring the rest.
///
/// - Writing the loop condition. Typically, I'm used to test for a single predicate and 'continue'
/// the loop otherwise, like so:
/// ```
/// // within the loop
/// if file_blocks[front].is_some() {
///     front += 1;
///     continue;
/// }
/// if file_blocks[back].is_none() {
///     back -= 1;
///     continue;
/// }
/// ```
/// Thinking about it, however, while this looks more aesthetically pleasing, it is less rigorous
/// when formulating the logic, because there is an implied 'else' clause that isn't explicitly
/// spelt-out, and may lead to missing conditions. Not in this situation. But perhaps in more
/// complicated ones. Worse still, if you forget the `continue` statement, you are just setting
/// yourself up for nasty bugs. So in this day's challenge, I wrote it using an `else-if` statement
/// which makes it 100% clear that these conditions indeed mutually exclusive and exhaustive. There
/// is no need for the `continue` statement either. Also, it's not as nested as I originally
/// visualised (merely an `else-if` and `else`) so perhaps this style might be growing on me.

fn parse_input(input: &str) -> Vec<Option<u64>> {
    let items: Vec<char> = input
        // this can be so much eaiser with simply calling .chars()
        .lines()
        .take(1)
        .flat_map(|line| line.chars())
        .collect();
    let mut id: u64 = 0;
    let mut result: Vec<Option<u64>> = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let length = item.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            // TODO another better way is to result.extend() and std::iter::repeat_n
            result.append(&mut vec![Some(id); length]);
            id += 1;
        } else {
            result.append(&mut vec![None; length]);
        }
    }
    result
}

fn solve_part_one(file_blocks: &mut Vec<Option<u64>>) -> u64 {
    let mut front = 0;
    let mut back = file_blocks.len() - 1;

    while front < back {
        if file_blocks[front].is_some() {
            front += 1;
        } else if file_blocks[back].is_none() {
            back -= 1;
        } else {
            file_blocks.swap(front, back);
        }
    }

    file_blocks
        .iter()
        // alternative is .filter_map(|&block| block), but less efficient
        .take_while(|block| block.is_some())
        .map(|block| block.unwrap())
        .enumerate()
        // you can could also .map() directly to the product of idx and id and call .sum()
        .fold(0, |sum, (idx, id)| sum + idx as u64 * id)

    /*
    Writing else forces you to think of exclusive conditions
    1. let front = 0;
    2. let back = file_blocks.len() - 1;
    3. select block at front
    4. if block at front is Some(u32), front += 1; continue // shift front ptr,
    5. else (front block is None) if block at back is None, back -= 1; continue // shift back ptr
    6. else (front is None && back is Some(u32)) blocks.swap (front, back), front += 1; back -= 1;
    7. repeat 3 - 6 while front < back
    8. return blocks .fold(), 0, |sum, block| -> sum + idx * block
    */
}

// fn solve_part_two(grid: &Vec<Vec<char>>) -> u32 {
//     todo!();
// }

fn main() {
    // see day_01 for the reason to import the text string this way
    // const INPUT: &'static str = include_str!("../input/day_09_test.txt");
    const INPUT: &'static str = include_str!("../input/day_09.txt");

    let mut file_blocks = parse_input(INPUT);
    // println!("file_blocks: {:?}", file_blocks);

    let part_one_answer = solve_part_one(&mut file_blocks);

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 6471961544878);

    // let part_two_answer = solve_part_two(&grid);

    // println!("Part two answer: {part_two_answer}");
    // assert_eq!(part_two_answer, 1686);
}
