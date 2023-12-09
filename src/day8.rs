use std::collections::HashMap;
use std::io::Read;
use num::integer::lcm;

fn follow_instruction(
    instruction: char,
    current_location: &String,
    map: &HashMap<String, (String, String)>,
) -> String {
    match instruction {
        'L' => (*map.get(current_location).unwrap().0.clone()).to_string(),
        'R' => (*map.get(current_location).unwrap().1.clone()).to_string(),
        _ => panic!("impossible")
    }
}

pub fn run() {
    let mut file = std::fs::File::open("day8/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut node_map: HashMap<String, (String, String)> = HashMap::new();

    let instructions = contents.split('\n').next().unwrap();

    let mut nodes: Vec<_> = contents.split('\n').skip(2).collect();
    nodes.pop();

    for node in nodes {
        let key = node.split(" = ").next().unwrap();
        let mut value = node
            .split(" = ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.replace(['(', ')'], ""));
        node_map
            .entry(key.to_string())
            .or_insert_with(|| (value.next().unwrap(), value.next().unwrap()));
    }

    /*
    let mut cnt = 0;
    let mut current_location = "AAA".to_string();

    while current_location != "ZZZ" {
        for instruction in instructions.chars() {
            current_location = follow_instruction(instruction, &current_location, &node_map);
            cnt += 1;
        }
    }
    
    println!("{}", cnt);
    */

    let mut current_locations: Vec<_> = node_map.iter().map(|x| x.0).filter(|x| x.ends_with('A')).cloned().collect();
    let mut steps: Vec<usize> = Vec::new();

    for loc in current_locations {
        let mut location = loc;
        let mut cnt_1 = 0;
        while !location.ends_with('Z') {
            for instruction in instructions.chars() {
                location = follow_instruction(instruction, &location, &node_map);
                cnt_1 += 1;
            }
        }
        steps.push(cnt_1);
    }

    let mut res = 1;
    for step in steps {
        res = lcm(res, step);
    }

    println!("{}", res)
}
