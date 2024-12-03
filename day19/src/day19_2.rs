use std::collections::HashMap;

const FILE_DATA: &str = include_str!("input.txt");

#[derive(Debug)]
struct Rule {
    pub var: usize,
    pub op: char,
    pub val: u32,
    pub dst: String
}

fn get_bounds(idx: usize, workflows: &HashMap<&str, Vec<Rule>>) -> Vec<(u32, u32)> {
    let mut bounds = Vec::new();

    let mut bound = (0, 0);
    let mut finding_min = true;

    for i in 0..=4000 {
        let mut current_dst = String::from("in");

        while current_dst != "A" && current_dst != "R" {
            let rules = workflows.get(&current_dst.as_str().clone()).unwrap();
            println!("{:?}", rules);
            if let Some(rule) = rules.iter().find(|&r| r.var == idx) {
                if rule.op == '>' {
                    if i > rule.val {
                        current_dst = rule.dst.clone();
                    }
                } else /* rule.op == '<' */ {
                    if i < rule.val {
                        current_dst = rule.dst.clone();
                    }
                }
            } else {
                current_dst = rules.last().unwrap().dst.clone();
            }
        }

        if current_dst == "A" {
            println!("A{i}");
            if finding_min {
                bound.0 = i;
            } else {
                bound.1 = i;
                bounds.push(bound);
            }

            finding_min = !finding_min;
        } else if current_dst == "R" {
            //println!("R{i}");
        }
    }

    bounds
}

pub fn answer() -> usize {
    let lines: Vec<Vec<&str>> = FILE_DATA
        .split("\r\n\r\n")
        .map(|block| block.lines().collect())
        .collect();
    
    let mut workflows = HashMap::new();
    let mut objects = Vec::new();

    for &workflow in &lines[0] {
        let line: Vec<&str> = workflow[0..workflow.len()-1].split('{').collect();
        let name = line[0];

        let rules_text: Vec<&str> = line[1].split(',').collect();
        let mut rules = Vec::with_capacity(rules_text.len());

        for r in rules_text {
            if r.contains(':') {
                let (condition, dst) = r.split_once(':').unwrap();
                let (var_text, val) = condition.split_once(['>', '<']).unwrap();
                let op = if condition.contains('>') {
                    '>'
                } else {
                    '<'
                };
                
                let var = match var_text {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => 4,
                };

                rules.push(Rule {
                    var,
                    op,
                    val: val.parse().unwrap(),
                    dst: dst.to_string()
                });
            } else {
                rules.push(Rule {
                    var: 4,
                    op: ' ',
                    val: 0,
                    dst: r.to_string()
                })
            }
        }
        
        //println!("{}: {:?}", name, rules);

        workflows.insert(name, rules);
    }

    for &object in &lines[1] {
        let data: Vec<u32> = object[1..object.len()-1]
            .split([',', '='])
            .filter(|&word| !word.starts_with(['x', 'm', 'a', 's']))
            .map(|num| num.parse().unwrap())
            .collect();

        //println!("{:?}", data);

        objects.push(data);
    }

    let mut sum = 0;

    // find top x bound
    // find bottom x bound


    println!("{:?}", get_bounds(0, &workflows));

    sum
}