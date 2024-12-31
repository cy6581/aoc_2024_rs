#![allow(unused)]

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
/// - Iterator operators. In addition to over-engineering the lines() iterator as mentioned, I also
/// experimented with various iterators to compute the checksum. I debated between using .filter(),
/// or its nicer cousin .filter_map() which will automatically unwrap the Option<u32> that I have.
/// The downside to both of them though is that they will iterate through the entire Vec, even
/// though we know that after some point all which remain are None (empty slots). So in this case,
/// .take_while() is more efficient because it stops at the first None, ignoring the rest.
///
/// - Drawbacks of using 'continue' statements in a single loop - part 1. Typically, I'm used to test
/// for a single predicate and 'continue' the loop otherwise, like so:
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
/// which makes it 100% clear that these conditions are indeed mutually exclusive and exhaustive.
/// There is no need for the `continue` statement either. Also, it's not as nested as I originally
/// visualised (merely an `else-if` and `else`) so perhaps this style might be growing on me.
///
/// - Drawbacks of using 'continue' statements - part 2. Part 2 was an even bigger problem in
/// trying to use the style of a single-loop, and the 'continue' statement. For one, it became
/// unavoidable to use a nested-while loop, when we are dealing with multiple pointers (front and
/// back) to represent a block. Let's call the parent left/right pointers 'boundary pointers', they
/// represent the boundary of the processed and unprocessed blocks. However, in part 2, each pointer
/// actually represents a pair of sub-pointers (front and back), that represent the length of a
/// block. The sub-pointers need to be repeatedly incremented or decremented independent of the
/// main pointer, so you have no choice but to write an inner while loop like so.
///
/// ```
/// let (mut file_back, mut file_front) = (right, right);
/// while file_front > 0 && blocks[file_front] == blocks[file_front - 1] {
///     file_front -= 1;
/// }
/// ```
/// Working with sub-pointers becomes even more tricky because you need to ensure that:
/// - Neither of the pointers goes out of bounds (in the main loop, you only check the bounds of
/// parent left and right pointers).
/// - The parent pointers need to be advanced BEYOND the sub-pointers when you are done with a
/// block. Concretely, this means setting it to something like `file_front + 1` while of course
/// checking that it stays within bounds.
/// There must be a better way, and hence my next learning point:
///
/// - It is MUCH MUCH easier to represent a block using a starting ponter and length. I briefly
/// alluded to this fact when saying that the sub-pointers (front and back) are needed to represent
/// the LENGTH of the block. After referencing the solution of a friend, I realised that I had
/// chosen a cumbersome abstraction. In fact, Rust has already solved this problem and given us a
/// best practice in the form of slices, which are stored under the hood as a starting pointer and
/// a length. This makes it so much easier to:
/// - Compare the length, you literally compare two values,
/// - Iterate over blocks, you literally iterate over starting pointers
/// - Prevent out of bounds, since you never increment pointers that could potentially go out of
/// bounds, you only iterate until the last pointer (which you don't even to do manually).
/// Then I also started to see why I potentially ran into problems using 'while' loops to increment
/// indices as per certain conditions. And this part from the Rust book finally made a lot more sense:
///
/// "For example, if you changed the definition of the a array to have four elements but forgot to
/// update the condition to while index < 4, the code would panic. It’s also slow, because the
/// compiler adds runtime code to perform the conditional check of whether the index is within the
/// bounds of the array on every iteration through the loop. ...
/// Using the for loop, you wouldn’t need to remember to change any other code if you changed the
/// number of values in the array, as you would with the method used in Listing 3-4. ...
/// The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
/// Even in situations in which you want to run some code a certain number of times, as in the
/// countdown example that used a while loop in Listing 3-3, most Rustaceans would use a for loop."
/// https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
///
/// At this risk of sounding cliched, I must say this is quite a paradigm shift. Instead relying too
/// much on the two-pointer technique (which can be easily abused into four-pointers), we should be
/// thinking in terms of iterators, which are less error-prone.
/// To practice this, I solved Part Two once more using this new representation, and you can see how
/// much simpler the code is.

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

fn solve_part_one(mut file_blocks: Vec<Option<u64>>) -> u64 {
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
}

fn solve_part_two(mut blocks: Vec<Option<u64>>) -> u64 {
    let mut right = blocks.len() - 1;
    while right > 0 {
        // advance pointer if it's free space, until a file
        if blocks[right].is_none() {
            right -= 1;
            continue;
        }
        let (mut file_back, mut file_front) = (right, right);
        while file_front > 0 && blocks[file_front] == blocks[file_front - 1] {
            file_front -= 1;
        }
        let mut left = 0;
        loop {
            // advance pointer if it's a file, until free space
            if blocks[left].is_some() {
                left += 1;
                continue;
            }
            let (mut empty_front, mut empty_back) = (left, left);
            while empty_back < blocks.len() - 1 && blocks[empty_back] == blocks[empty_back + 1] {
                empty_back += 1;
            }
            // the principle is to process, then set the outside (limit pointers) to the furthest advance
            if empty_front > file_front {
                // not possible to move this file, set right to before the file_front to skip
                right = if file_front > 0 { file_front - 1 } else { 0 };
                break;
            } else if (file_back - file_front) > (empty_back - empty_front) {
                // empty block cannot fit the file, set left to after the empty_back to skip
                left = empty_back + 1;
            } else {
                // do until whole file is swapped
                while !(file_back < file_front) {
                    // we want to swap the pointers that are furthest away to move them closer inwards
                    blocks.swap(empty_front, file_back);
                    empty_front += 1;
                    file_back -= 1;
                }
                // in this condition, left is discarded, so only set right to furthest processed file
                right = if file_front > 0 { file_front - 1 } else { 0 };
                break;
            }
        }
    }

    blocks
        .iter()
        .enumerate()
        .filter_map(|(idx, id_opt)| id_opt.and_then(|id| id.checked_mul(idx as u64)))
        .sum()
}

//
// Functions for alternate solution to part two
//
#[derive(Debug)]
struct FileSpace {
    id: usize,
    start: usize,
    len: usize,
}
#[derive(Debug)]
struct FreeSpace {
    start: usize,
    len: usize,
}

fn parse_input_alt(input: &str) -> (Vec<FileSpace>, Vec<FreeSpace>) {
    let mut file_spaces = Vec::new();
    let mut free_spaces = Vec::new();
    let disk_map = input.chars();

    let mut start_idx = 0;
    let mut block_id = 0;
    for (raw_idx, length) in disk_map.enumerate() {
        let len = length.to_digit(10).unwrap() as usize;
        if raw_idx % 2 == 0 {
            file_spaces.push(FileSpace {
                start: start_idx,
                len,
                id: block_id,
            });
            block_id += 1;
        } else {
            free_spaces.push(FreeSpace {
                start: start_idx,
                len,
            })
        }
        start_idx += len;
    }
    (file_spaces, free_spaces)
}

fn solve_part_two_alt((mut file_spaces, mut free_spaces): (Vec<FileSpace>, Vec<FreeSpace>)) -> u64 {
    for file_space in file_spaces.iter_mut().rev() {
        for free_space in free_spaces.iter_mut() {
            let no_valid_move = free_space.start > file_space.start;
            if no_valid_move {
                break;
            } else if free_space.len >= file_space.len {
                let free_space_start = free_space.start;
                free_space.start += file_space.len;
                free_space.len -= file_space.len;
                file_space.start = free_space_start;
                break;
            }
        }
    }

    file_spaces
        .iter()
        .map(|f| (f.start..f.start + f.len).sum::<usize>() * f.id) // sum the range * id value
        .sum::<usize>() as u64
}

fn main() {
    // see day_01 for the reason to import the text string this way
    // const INPUT: &'static str = include_str!("../input/day_09_test.txt");
    const INPUT: &'static str = include_str!("../input/day_09.txt");

    let file_blocks = parse_input(INPUT);

    // println!("file_blocks: {:?}", file_blocks);

    let part_one_answer = solve_part_one(file_blocks.clone());

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 6471961544878);

    let part_two_answer = solve_part_two(file_blocks.clone());
    assert_eq!(part_two_answer, 6511178035564);

    // Alternate solution to part two
    let file_blocks = parse_input_alt(INPUT);
    let part_two_answer = solve_part_two_alt(file_blocks);

    println!("Part two answer: {part_two_answer}");
    assert_eq!(part_two_answer, 6511178035564);
}
