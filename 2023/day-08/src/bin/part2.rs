use core::panic;
use std::collections::HashMap;

struct Node {
    value: String,
    left_node: String,
    right_node: String,
}

impl Node {
    fn new(value: String, left: String, right: String) -> Node {
        Node {
            value: value,
            left_node: left,
            right_node: right,
        }
    }
}

struct Graph {
    nodes: Vec<Node>,
    node_hash: HashMap<String, usize>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: vec![],
            node_hash: HashMap::new(),
        }
    }
    fn search_node(&self, target: &str) -> usize {
        if let Some(&index) = self.node_hash.get(target) {
            index
        } else {
            panic!("The node {} is not in the graph", target)
        }
    }
    fn add_node(&mut self, value: &str, left: &str, right: &str) {
        let index = self.nodes.len();
        self.nodes.push(Node::new(
            value.to_string(),
            left.to_string(),
            right.to_string(),
        ));
        self.node_hash.insert(value.to_string(), index);
    }
}
fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u64 {
    let mut graph = Graph::new();
    let mut lines = input.lines();
    let direction = lines.next().unwrap();
    lines.next();
    for line in lines {
        graph.add_node(&line[0..3], &line[7..10], &line[12..15]);
    }
    // for node in graph.nodes {
    //     println!("{}", [node.value + " = (" + &node.left_node + ", " + &node.right_node + ")"].concat())
    // }
    let mut current = "AAA";
    //let mut current_idx = graph.search_node("AAA");
    let direction_len = direction.len();
    //let mut counter = 0;
    let mut idx_list = vec![];
    let mut found_list = vec![];
    let mut len = graph.nodes.len();
    for i in 0..len {
        if graph.nodes[i].value.ends_with("A") {
            idx_list.push(i);
            found_list.push(false);
        }
    }
    len = idx_list.len();
    // while !found_list.iter().all(|&x| x) {
    //     for i in 0..len {
    //         found_list[i] = false;
    //     }
    //     for i in 0..len {
    //         let dir_idx = counter as usize % direction_len;
    //         if &direction[dir_idx..dir_idx+1] == "L" {
    //             current = &graph.nodes[idx_list[i]].left_node;
    //         }
    //         else if &direction[dir_idx..dir_idx+1] == "R" {
    //             current = &graph.nodes[idx_list[i]].right_node;
    //         }
    //         idx_list[i] = graph.search_node(current);
    //         if current.ends_with("Z") {
    //             found_list[i] = true
    //         }
    //     }
    //     counter += 1;
    //     if &counter % 1000000 == 0 {
    //         dbg!(&counter);
    //     }
    // }

    // try
    let mut acc = 1;
    for i in 0..len {
        let mut this_count = 0;
        println!("{}", &graph.nodes[idx_list[i]].value);
        while !found_list[i] {
            let dir_idx = this_count as usize % direction_len;
            if &direction[dir_idx..dir_idx + 1] == "L" {
                current = &graph.nodes[idx_list[i]].left_node;
            } else if &direction[dir_idx..dir_idx + 1] == "R" {
                current = &graph.nodes[idx_list[i]].right_node;
            }
            this_count += 1;
            idx_list[i] = graph.search_node(current);
            if current.ends_with("Z") {
                found_list[i] = true;
                acc = lcm(acc, dbg!(this_count));
            }
            // if &this_count % 1000 == 0 {
            //     println!("{}", &this_count);
            // }
        }
        println!("{}", &graph.nodes[idx_list[i]].value);
    }
    acc
    //counter
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part2(test_case);
        assert_eq!(result, 6u64);
    }
    #[test]
    fn test2() {
        let test_case = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)";
        let result = part2(test_case);
        assert_eq!(result, 2u64);
    }
}
