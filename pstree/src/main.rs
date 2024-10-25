use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead};

struct DArc {
    parent: usize,
    child: usize,
}

struct DGraph {
    arcs: Vec<DArc>,
    pid_to_name: HashMap<usize, String>,
}

impl DGraph {
    fn new() -> DGraph {
        DGraph {
            arcs: Vec::new(),
            pid_to_name: HashMap::new(),
        }
    }

    fn add_arc(&mut self, parent: usize, child: usize, child_name: String) {
        self.arcs.push(DArc { parent, child });
        self.pid_to_name.insert(child, child_name);
    }

    fn print_tree(
        &self,
        f: &mut std::fmt::Formatter,
        parent: usize,
        indent: usize,
    ) -> std::fmt::Result {
        // Print the current node with indentation
        if parent != 0 {
            write!(
                f,
                "{:indent$}{}\n",
                "",
                format!("{}({})", self.pid_to_name.get(&parent).unwrap(), parent),
                indent = indent * 2
            )?;
        } else {
            write!(f, "{:indent$}{}\n", "", parent, indent = indent * 2)?;
        }

        // Iterate over the child nodes
        for arc in &self.arcs {
            if arc.parent == parent {
                // Recursively print each child node with increased indentation
                self.print_tree(f, arc.child, indent + 1)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for DGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.print_tree(f, 0, 0)?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let path = "/proc";
    let re = Regex::new(r"^\d+$").unwrap();
    let mut graph = DGraph::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().unwrap();
        if re.is_match(file_name_str) {
            let entry_path = entry.path();
            let entry_path_str = entry_path.to_str().unwrap();
            let file = File::open(format!("{}/{}", entry_path_str, "stat"))?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let columns: Vec<&str> = line.split(" ").collect();
                let parent = columns.get(3).unwrap().parse::<usize>().unwrap();
                let child = columns.get(0).unwrap().parse::<usize>().unwrap();
                let child_name = columns.get(1).unwrap().replace("(", "").replace(")", "");
                graph.add_arc(parent, child, child_name);
            }
        }
    }
    println!("{}", graph);
    Ok(())
}
