//! User-friendly tree display for sort results.
//!
//! Builds a tree from [`MoveRecord`] destinations and renders it with
//! box-drawing characters, similar to the `tree` command.

use std::collections::BTreeMap;

use crate::core::MoveRecord;

/// Print a tree of move actions grouped by destination directory.
pub fn print_move_tree(actions: &[MoveRecord]) {
    if actions.is_empty() {
        return;
    }

    let mut root = DirTree::default();
    for action in actions {
        let components: Vec<String> = action
            .dest_relative
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();
        root.insert(&components);
    }

    let nodes = convert_and_sort(root);
    let count = nodes.len();
    for (i, node) in nodes.iter().enumerate() {
        node.render("", i == count - 1);
    }
}

enum TreeNode {
    Dir {
        name: String,
        children: Vec<TreeNode>,
    },
    File {
        name: String,
    },
}

#[derive(Debug, Default)]
struct DirTree {
    children: BTreeMap<String, DirTree>,
    files: Vec<String>,
}

impl DirTree {
    fn insert(&mut self, components: &[String]) {
        if components.is_empty() {
            return;
        }
        if components.len() == 1 {
            self.files.push(components[0].clone());
        } else {
            let dir = components[0].clone();
            let rest = &components[1..];
            self.children.entry(dir).or_default().insert(rest);
        }
    }
}

impl TreeNode {
    fn render(&self, prefix: &str, is_last: bool) {
        match self {
            TreeNode::Dir { name, children } => {
                let connector = if is_last { "└── " } else { "├── " };
                println!("{prefix}{connector}{name}/");
                let child_prefix = if is_last {
                    format!("{prefix}    ")
                } else {
                    format!("{prefix}│   ")
                };
                let count = children.len();
                for (i, child) in children.iter().enumerate() {
                    child.render(&child_prefix, i == count - 1);
                }
            }
            TreeNode::File { name } => {
                let connector = if is_last { "└── " } else { "├── " };
                println!("{prefix}{connector}{name}");
            }
        }
    }
}

fn convert_and_sort(tree: DirTree) -> Vec<TreeNode> {
    let mut nodes: Vec<TreeNode> = Vec::new();

    for (_name, child) in tree.children {
        let grandchildren = convert_and_sort(child);
        nodes.push(TreeNode::Dir {
            name: _name,
            children: grandchildren,
        });
    }

    for file in tree.files {
        nodes.push(TreeNode::File { name: file });
    }

    nodes.sort_by(|a, b| match (a, b) {
        (TreeNode::Dir { name: a_name, .. }, TreeNode::Dir { name: b_name, .. }) => {
            a_name.cmp(b_name)
        }
        (TreeNode::Dir { .. }, TreeNode::File { .. }) => std::cmp::Ordering::Less,
        (TreeNode::File { .. }, TreeNode::Dir { .. }) => std::cmp::Ordering::Greater,
        (TreeNode::File { name: a_name }, TreeNode::File { name: b_name }) => a_name.cmp(b_name),
    });

    nodes
}
