use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let input: Vec<(String, String)> = get_input::<Input>("day-12").0;
    let connections = connections(input);
    let mut complete_paths = Vec::new();
    let initial_path = vec![String::from("start")];
    extend_pathways(&mut complete_paths, initial_path, &connections, true);
    let num_paths = complete_paths.len();
    dbg!(num_paths);
}

fn part1() {
    let input: Vec<(String, String)> = get_input::<Input>("day-12").0;
    let connections = connections(input);
    let mut complete_paths = Vec::new();
    let initial_path = vec![String::from("start")];
    extend_pathways(&mut complete_paths, initial_path, &connections, false);
    let num_paths = complete_paths.len();
    dbg!(num_paths);
}

fn extend_pathways(
    complete_paths: &mut Vec<Vec<String>>,
    current_path: Vec<String>,
    connections: &HashMap<String, Vec<String>>,
    allow_double_visit: bool,
) {
    for path in connections.get(current_path.last().unwrap()).unwrap() {
        let mut current_path = current_path.clone();
        if path.eq("end") {
            current_path.push(path.to_string());
            complete_paths.push(current_path);
        } else if path.eq("start") {
            // Nothing
        } else if path.chars().all(|a| a.is_uppercase()) || !current_path.contains(path) {
            current_path.push(path.to_string());
            extend_pathways(
                complete_paths,
                current_path,
                connections,
                allow_double_visit,
            );
        } else if allow_double_visit {
            current_path.push(path.to_string());
            extend_pathways(complete_paths, current_path, connections, false);
        }
    }
}

fn connections(paths: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut connections = HashMap::new();
    for path in paths {
        let entry = connections.entry(path.0.clone()).or_insert_with(Vec::new);
        entry.push(path.1.clone());
        let entry = connections.entry(path.1).or_insert_with(Vec::new);
        entry.push(path.0);
    }
    connections
}

#[derive(Clone, Deserialize)]
struct Input(Vec<(String, String)>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
