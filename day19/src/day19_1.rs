use std::collections::HashMap;

const FILE_DATA: &str = include_str!("input.txt");

#[derive(Debug)]
struct Rule {
    pub var: char,
    pub op: char,
    pub val: u32,
    pub dst: String
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
                let (var, val) = condition.split_once(['>', '<']).unwrap();
                let op = if condition.contains('>') {
                    '>'
                } else {
                    '<'
                };
                
                rules.push(Rule {
                    var: var.chars().nth(0).unwrap(),
                    op,
                    val: val.parse().unwrap(),
                    dst: dst.to_string()
                });
            } else {
                rules.push(Rule {
                    var: ' ',
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

    for object in &objects {
        let mut current_dst = String::from("in");

        while current_dst != "A" && current_dst != "R" {
            for rule in workflows.get(&current_dst.as_str().clone()).unwrap() {
                if rule.op == ' ' {
                    current_dst = rule.dst.clone();
                    break;
                } else {
                    let idx = match rule.var {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => 4
                    };

                    if rule.op == '>' {
                        if object[idx] > rule.val {
                            current_dst = rule.dst.clone();
                            break;
                        }
                    } else /* rule.op == '<' */ {
                        if object[idx] < rule.val {
                            current_dst = rule.dst.clone();
                            break;
                        }
                    }
                }
            }
            //current_dst_char = object
        }

        if current_dst == "A" {
            sum += 1;//object.iter().sum::<u32>() as usize;
        }
    }

    sum
}