use std::io;

use r3bl_rs_utils::arena::*;

#[derive(Debug, Clone)]
enum FileType {
    File(String, i32),
    Directory(String),
}

#[derive(Debug, Clone)]
struct DirectorySize {
    name: String,
    size: i32,
}

fn decode_file_hierarchy(input: &str) -> Arena<FileType> {
    let mut arena = Arena::<FileType>::new();
    let root_id = arena.add_new_node(FileType::Directory("/".to_string()), None);
    let commands = input.split("$ ");
    let mut current_path = vec![root_id];

    commands.for_each(|command| {
        if command.starts_with("cd") {
            let path = command.split_whitespace().skip(1).next().unwrap();
            match path {
                "/" => current_path = vec![root_id],
                ".." => {
                    current_path.pop();
                }
                _ => {
                    let children_of_current = arena.get_children_of(*current_path.last().unwrap());
                    match children_of_current {
                        None => {
                            panic!("ls required first to populate")
                        }
                        Some(vec) => {
                            vec.iter().for_each(|child_id| {
                                let ptr = arena.get_node_arc(*child_id).unwrap();
                                let ptr = ptr.read().unwrap();
                                match &ptr.payload {
                                    FileType::File(_, _) => {}
                                    FileType::Directory(name) => {
                                        if name == path {
                                            current_path.push(*child_id)
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            }
        } else if command.starts_with("ls") {
            command.lines().skip(1).for_each(|line| {
                let mut iter = line.split_whitespace();
                let size_or_dir = iter.next().unwrap();
                let name = iter.next().unwrap();

                match size_or_dir {
                    "dir" => {
                        arena.add_new_node(
                            FileType::Directory(name.to_string()),
                            Some(*current_path.last().unwrap()),
                        );
                    }
                    _ => {
                        arena.add_new_node(
                            FileType::File(name.to_string(), size_or_dir.parse().unwrap()),
                            Some(*current_path.last().unwrap()),
                        );
                    }
                }
            });
        }
    });

    arena
}

fn calculate_size(arena: Arena<FileType>) -> Arena<DirectorySize> {
    let mut result = Arena::new();
    calculate_directory_size_recurse(&arena, &mut result, 0 as usize, None);
    result
}

fn calculate_directory_size_recurse(
    old_arena: &Arena<FileType>,
    new_arena: &mut Arena<DirectorySize>,
    old_arena_current_directory_id: usize,
    new_arena_parent_id: Option<usize>,
) {
    let cost_nodes = old_arena.tree_walk_dfs(old_arena_current_directory_id);
    let mut parent_id = None;
    match cost_nodes {
        Some(nodes) => {
            let size = nodes.iter().fold(0, |accum, item| {
                let node_ptr = old_arena.get_node_arc(*item).unwrap();
                let node_ptr = node_ptr.read().unwrap();
                match node_ptr.payload {
                    FileType::Directory(_) => accum,
                    FileType::File(_, size) => accum + size,
                }
            });

            let current_node = old_arena
                .get_node_arc(old_arena_current_directory_id)
                .unwrap();
            let current_node = current_node.read().unwrap();
            if let FileType::Directory(name) = &current_node.payload {
                parent_id = Some(new_arena.add_new_node(
                    DirectorySize {
                        name: name.to_string(),
                        size,
                    },
                    new_arena_parent_id,
                ));
            }
        }
        None => {}
    }
    let children = old_arena.get_children_of(old_arena_current_directory_id);
    match children {
        Some(nodes) => nodes.iter().for_each(|node| {
            calculate_directory_size_recurse(old_arena, new_arena, *node, parent_id);
        }),
        None => {}
    }
}

fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();

    let file_system = decode_file_hierarchy(&input);
    let size_tree = calculate_size(file_system);

    let size_list = size_tree
        .filter_all_nodes_by(&|id, directory| directory.size <= 100_000)
        .unwrap();
    let size = size_list.iter().fold(0, |accum, item| {
        let ptr = size_tree.get_node_arc(*item).unwrap();
        let ptr = ptr.read().unwrap();
        accum + ptr.payload.size
    });

    println!("The size of the files less than 100,000 Bytes is: {size}");
    let root_size = size_tree.get_node_arc(0 as usize).unwrap();
    let root_size = root_size.read().unwrap();
    let root_size = root_size.payload.size;
    println!("Root size: {root_size}");

    let directory_to_delete = size_tree
        .filter_all_nodes_by(&|_, directory| {
            30_000_000 <= (70_000_000 - 43_598_596 + directory.size)
        })
        .unwrap();
    let (directory_to_delete, directory_size) =
        directory_to_delete
            .iter()
            .fold((String::new(), 0), |accum, item| {
                let ptr = size_tree.get_node_arc(*item).unwrap();
                let ptr = ptr.read().unwrap();
                if accum.0 == "" || (accum.0 != "" && accum.1 > ptr.payload.size) {
                    (ptr.payload.name.clone(), ptr.payload.size)
                } else {
                    accum
                }
            });

    println!("Delete Directory: {directory_to_delete} which is size: {directory_size}");
}
