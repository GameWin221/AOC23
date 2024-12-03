use std::collections::HashMap;

const FILE_DATA: &str = include_str!("input.txt");

#[derive(PartialEq, Debug, Clone)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster
}

#[derive(Debug, Clone)]
struct Module {
    pub mod_type: ModuleType,
    pub destinations: Vec<String>,
    pub states: HashMap<String, bool>,
}

#[derive(Debug, Clone)]
struct NextPulse {
    pub src: String,
    pub dst: String,
    pub state: bool
}

fn traverse(name: &String, src: &String, state: bool, modules: &mut HashMap<&str, Module>) -> Vec<NextPulse> {
    //println!("{} -{}-> {}", src, state, name);

    if !modules.contains_key(name.as_str()) {
        return vec![];
    }

    let mut module = (*modules.get(name.as_str()).unwrap()).clone();
    let mut next = Vec::new();

    if module.mod_type == ModuleType::FlipFlop {
        if !state {
            let next_state = !module.states.get("in").unwrap().clone();

            for dest in &module.destinations {
                next.push(NextPulse { src: name.clone(), dst: dest.clone(), state: next_state })
            }

            *module.states.get_mut("in").unwrap() = next_state;
        }
    } else if module.mod_type == ModuleType::Conjunction {
        *module.states.get_mut(src).unwrap() = state;

        let next_state = !module.states.values().all(|&st| st);

        for dest in &module.destinations {
            next.push(NextPulse { src: name.clone(), dst: dest.clone(), state: next_state })
        }
    } else /* Broadcaster */ {
        for dest in &module.destinations {
            next.push(NextPulse { src: name.clone(), dst: dest.clone(), state })
        }
    }

    modules.get_mut(name.as_str()).unwrap().states = module.states;

    next
}

pub fn answer() -> usize {
    let mut modules: HashMap<&str, Module> = HashMap::new();

    FILE_DATA.lines().for_each(|line| {
        let line: Vec<&str> = line.split(" -> ").collect();
        let (mut name, destinations) = (line[0], line[1]);

        let mut states = HashMap::new();

        let mod_type = if name.starts_with('%') {
            name = &name[1..];
            states.insert(String::from("in"), false);
            ModuleType::FlipFlop
        } else if name.starts_with('&') {
            name = &name[1..];
            
            ModuleType::Conjunction
        } else {
            ModuleType::Broadcaster
        };

        let destinations: Vec<String> = destinations.split(", ").map(|str| str.to_string()).collect();

        modules.insert(name, Module {
            mod_type,
            destinations,
            states
        });
    });

    FILE_DATA.lines().for_each(|line| {
        let line: Vec<&str> = line.split(" -> ").collect();
        let (mut name, destinations) = (line[0], line[1]);

        if name.starts_with(['%', '&']) {
            name = &name[1..];
        }

        let destinations: Vec<String> = destinations.split(", ").map(|str| str.to_string()).collect();

        for destination in &destinations {
            if let Some(dest) = modules.get_mut(destination.as_str()) {
                if dest.mod_type == ModuleType::Conjunction {
                    dest.states.insert(name.to_string(), false);
                }
            }
        }
    });

    let mut high = 0;
    let mut low = 0;

    for _ in 0..1000 {
        let mut previous = traverse(&String::from("broadcaster"), &String::from("button"), false, &mut modules);

        low += 1 + previous.len(); // (button -low-> broadcaster) + (broadcaster -low-> ...)

        loop {
            let mut next = Vec::new();

            for p in &previous {
                for n in traverse(&p.dst, &p.src, p.state, &mut modules) {
                    if n.state {
                        high += 1;
                    } else {
                        low += 1;
                    }

                    next.push(n);
                }
            }
    
            if next.is_empty() {
                break;
            }
    
            std::mem::swap(&mut next, &mut previous);
        }
    }

    low * high
}
