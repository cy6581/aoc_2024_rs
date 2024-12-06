/// Assumptions
/// 1. There are no circular dependencies encoded in the rules of prereqs-to-targets (i.e. the 
/// graph is acyclic). Otherwise the list of rules is invalid and the whole problem falls apart. So 
/// we can skip this check.
/// 2. All source preqrequisites are directly connected to their dependent destinations, because there
/// is an explicit rule for every single preq-to-target relationship.
/// TODO: it seems like the term for this might be a Transitive Closure, to check
use std::collections::HashMap;
use std::cmp::Ordering;

fn parse_input(input: &str) -> (HashMap<u32,Vec<u32>>, Vec<Vec<u32>>) {
    // TODO there must be another cleaner way without collecting to the intermediate Vec
    // or using sections...
    let lines = input.lines().collect::<Vec<_>>();
    let mut sections = lines.split(|line| line.is_empty());

    let deps = sections.next().unwrap();
    let mut dep_adj_list: HashMap<u32,Vec<u32>> = HashMap::with_capacity(deps.len());
    for dep in deps {
        if let [source, dest] = &dep.split("|").collect::<Vec<_>>()[..2] {
            let (source, dest) = (source.parse::<u32>().unwrap(), dest.parse::<u32>().unwrap());
            let entry = dep_adj_list.entry(source).or_insert(Vec::new());
            entry.push(dest);
        }
    }

    let tasks = sections.next().unwrap();
    let tasks: Vec<Vec<u32>> = tasks
        .into_iter()
        .map(|line| line.split(","))
        .map(|parts| {
            parts
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    
    (dep_adj_list, tasks)
}

fn is_valid_task_path(task_path: &Vec<u32>, dep_adj_list: &HashMap<u32,Vec<u32>>) -> bool {
    // Because of assumption 2: each source will map directly to the subequent destination, we can 
    // save on checking the remaining nodes except the immediate the next node. Use sliding window.
    for win in task_path.windows(2) {
        let source = win[0];
        let dest = win[1];
        if let Some(known_destinations) = dep_adj_list.get(&source) {
            if !known_destinations.iter().any(|&t| t == dest ) {
                return false;
            }
        } else {
            // source vertex has no destinations, the path is broken
            return false;
        }
    }
    true
}

fn to_reordered_task_path(task_path: &Vec<u32>, adj_list: &HashMap<u32,Vec<u32>>) -> Vec<u32> {
    // Once again, because of assumption 2, saves us the trouble of performing full topological 
    // reordering of the path, e.g. using DFS. Instead, we know that between 2 nodes 'v' and 'w', 
    // 'v' must either appear in the (outgoing) adjacency list of 'w', or 'w' must appear in the 
    // adjacency list of 'v'. Hence, we can pass the result of this comparison to a sort function.
    let mut reordered = task_path.to_vec();
    reordered.sort_unstable_by(|v, w| {
        let v_dests = adj_list.get(&v);
        let w_dests = adj_list.get(&w);
        if v_dests.is_some() && v_dests.unwrap().iter().any(|dest| dest == w) {
            Ordering::Less
        } else if w_dests.is_some() && w_dests.unwrap().iter().any(|d| d == v) {
            Ordering::Greater
        } else {
            unreachable!();
        }
    });
    reordered
    /*
    1. sort task_path, using the predicate fn for elements 'v' and 'w'
    2. if 'v' in adj list of 'w', return Order::LessThan // v is prereq
    3. elseif 'w' in adj list of 'v', return Order::GreaterThan
    4. else unreachable!()
    */
}


fn solve_part_one(dep_adj_list: &HashMap<u32, Vec<u32>>, tasks: &Vec<Vec<u32>>) -> u32 {
    tasks.iter().fold(0, |sum, task_path| {
        if is_valid_task_path(task_path, dep_adj_list) {
            let mid_idx = task_path.len() / 2;
            sum + task_path[mid_idx]
        } else {
            sum
        }
    })
}

fn solve_part_two(dep_adj_list: &HashMap<u32, Vec<u32>>, tasks: &Vec<Vec<u32>>) -> u32 {
    tasks.iter().fold(0, |sum, task_path| {
        if !is_valid_task_path(task_path, dep_adj_list) {
            let reordered = to_reordered_task_path(task_path, dep_adj_list);
            let mid_idx = reordered.len() / 2;
            sum + reordered[mid_idx]
        } else {
            sum
        }
    })
}

fn main() {
    // see day_01 for the reason to import the text string this way
    // const INPUT: &'static str = include_str!("../input/day_05_test.txt");
    const INPUT: &'static str = include_str!("../input/day_05.txt");

    let (dep_adj_list, tasks) = parse_input(INPUT);

    let part_one_answer = solve_part_one(&dep_adj_list, &tasks);

    println!("Part one answer: {part_one_answer}");
    assert_eq!(part_one_answer, 6260);

    let part_two_answer = solve_part_two(&dep_adj_list, &tasks);

    println!("Part two answer: {part_two_answer}");
    assert_eq!(part_two_answer, 5346);
}
