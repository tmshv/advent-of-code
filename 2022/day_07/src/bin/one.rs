use std::io;

#[derive(Debug)]
struct Command {
    name: String,
    args: String,
    output: Vec<String>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug, Clone, Copy)]
struct NodeId {
    index: usize,
}

#[derive(Debug, Clone)]
struct Node<T> {
    id: NodeId,
    name: String,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    payload: T,
}

#[derive(Debug)]
struct Tree<T> {
    current: Option<NodeId>,
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    fn new() -> Tree<T> {
        Tree {
            current: None,
            nodes: vec![],
        }
    }

    fn set_current(&mut self, value: NodeId) {
        self.current = Some(value);
    }

    fn get_current_mut(&mut self) -> Option<&mut Node<T>> {
        match self.current {
            None => None,
            Some(id) => {
                let node = &mut self.nodes[id.index];
                Some(node)
            }
        }
    }

    fn add_node(&mut self, name: String, payload: T) -> NodeId {
        let id = NodeId {
            index: self.nodes.len(),
        };
        let node = Node {
            id,
            name,
            payload,
            children: vec![],
            parent: self.current,
        };
        self.nodes.push(node);
        match self.current {
            None => {}
            Some(c) => {
                self.nodes[c.index].children.push(id);
            }
        }
        id
    }

    fn get_by_name(&mut self, name: String) -> NodeId {
        let node = &self.nodes[self.current.unwrap().index];
        for i in &node.children {
            let c = &self.nodes[i.index];
            if c.name == name {
                return c.id;
            }
        }
        return NodeId { index: 0 };
    }
}

fn is_absolute(path: &String) -> bool {
    path.starts_with("/")
}

fn is_command(value: &String) -> bool {
    let first = value.chars().next().unwrap();
    return first == '$';
}

fn parse_command(value: String) -> (String, String) {
    let parts = value.split(" ").collect::<Vec<&str>>();
    let command = parts[1];
    match command {
        "cd" => (String::from("cd"), String::from(parts[2])),
        "ls" => (String::from("ls"), String::new()),
        _ => (String::from("exit"), String::new()),
    }
}

fn read_input() -> Vec<Command> {
    let mut command = String::new();
    let mut args = String::new();
    let mut output = Vec::<String>::new();
    let mut items = Vec::<Command>::new();
    for x in io::stdin().lines() {
        let line = x.unwrap();

        // collecting output
        if !is_command(&line) {
            output.push(line);
            continue;
        }

        // run command with collected output
        if !command.is_empty() {
            items.push(Command {
                name: String::from(command),
                args: String::from(args),
                output,
            });
        }

        (command, args) = parse_command(line);
        output = Vec::<String>::new();
    }

    // add last command at the end of the loop
    items.push(Command {
        name: String::from(command),
        args: String::from(args),
        output,
    });
    items
}

fn calculate_folder_size(root: &Tree<Vec<File>>, id: NodeId) -> u32 {
    let mut total = 0;
    let node = &root.nodes[id.index];
    for f in &node.children {
        let folder_size = calculate_folder_size(root, *f);
        total += folder_size;
    }
    for f in &node.payload {
        total += f.size;
    }
    total
}

fn solve_task(root: &Tree<Vec<File>>) {
    let mut total = 0;
    for f in &root.nodes[1..] {
        let size = calculate_folder_size(root, f.id);
        if size < 100000 {
            total += size;
        }
    }
    println!("Result: {}", total);
}

fn flat_fs(root: &Tree<Vec<File>>, id: NodeId, padding: usize) -> Vec<(String, u32, usize)> {
    let mut nodes: Vec<(String, u32, usize)> = vec![];
    let node = &root.nodes[id.index];

    let folder_size = calculate_folder_size(root, id);
    nodes.push((node.name.clone(), folder_size, padding));

    for f in &node.children {
        let next_nodes = flat_fs(root, *f, padding + 1);
        for i in next_nodes {
            nodes.push(i);
        }
    }
    for f in &node.payload {
        nodes.push((f.name.clone(), f.size, padding + 1));
    }
    nodes
}

fn print_fs(tree: &Tree<Vec<File>>) {
    let items = flat_fs(tree, NodeId { index: 0 }, 0);
    for (name, size, padding) in items {
        if size > 0 {
            println!("{} - {} (size={})", " ".repeat(padding), name, size);
        } else {
            println!("{} - {}", " ".repeat(padding), name);
        }
    }
}

fn main() {
    let mut tree = Tree::<Vec<File>>::new();
    let root = tree.add_node("/".to_string(), vec![]);
    tree.set_current(root);

    for x in read_input() {
        if x.name == "cd" {
            let path = x.args;
            if is_absolute(&path) {
                tree.set_current(root);
            } else if path == ".." {
                match tree.current {
                    None => {
                        panic!("Root has to parent");
                    }
                    Some(val) => {
                        let node = &tree.nodes[val.index];
                        tree.set_current(node.parent.unwrap());
                    }
                }
            } else {
                let p = tree.get_by_name(path);
                tree.set_current(p);
            }
        }
        if x.name == "ls" {
            for file in x.output {
                let s: Vec<&str> = file.split(' ').collect();
                if s.len() != 2 {
                    panic!("Something wrong with input");
                }
                let dir_or_size = s[0];
                let file_name = s[1].to_string();
                if dir_or_size == "dir" {
                    tree.add_node(file_name, vec![]);
                } else {
                    let size = dir_or_size.parse::<u32>().unwrap();
                    let folder = tree.get_current_mut();
                    match folder {
                        None => {}
                        Some(node) => {
                            node.payload.push(File {
                                name: file_name,
                                size,
                            });
                        }
                    }
                }
            }
        }
    }

    print_fs(&tree);
    solve_task(&tree);
}
