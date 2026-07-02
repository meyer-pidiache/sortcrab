//! User-friendly tree display for sort results.
//!
//! Builds a tree from [`MoveRecord`] destinations and renders it with
//! box-drawing characters, similar to the `tree` command.

use std::collections::BTreeMap;
use std::path::Path;

use crate::core::MoveRecord;

/// Print a tree of move actions grouped by destination directory.
pub fn print_move_tree(actions: &[MoveRecord], target: &Path) {
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

    println!("{}", target.display());

    let nodes = convert_and_sort(root);
    let count = nodes.len();
    for (i, node) in nodes.iter().enumerate() {
        node.render("", i == count - 1);
    }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ── DirTree::insert ─────────────────────────────────────────────

    #[test]
    fn insert_single_file() {
        let mut tree = DirTree::default();
        tree.insert(&["file.txt".to_string()]);
        assert_eq!(tree.files, vec!["file.txt"]);
        assert!(tree.children.is_empty());
    }

    #[test]
    fn insert_nested_file() {
        let mut tree = DirTree::default();
        tree.insert(&["a".to_string(), "b".to_string(), "file.txt".to_string()]);
        assert!(tree.files.is_empty());
        assert_eq!(tree.children.len(), 1);
        let a = &tree.children["a"];
        assert_eq!(a.children.len(), 1);
        let b = &a.children["b"];
        assert_eq!(b.files, vec!["file.txt"]);
    }

    #[test]
    fn insert_empty_components_is_noop() {
        let mut tree = DirTree::default();
        tree.insert(&[]);
        assert!(tree.files.is_empty());
        assert!(tree.children.is_empty());
    }

    #[test]
    fn insert_multiple_files_same_dir() {
        let mut tree = DirTree::default();
        tree.insert(&["dir".to_string(), "a.txt".to_string()]);
        tree.insert(&["dir".to_string(), "b.txt".to_string()]);
        let dir = &tree.children["dir"];
        assert_eq!(dir.files.len(), 2);
        assert!(dir.files.contains(&"a.txt".to_string()));
        assert!(dir.files.contains(&"b.txt".to_string()));
    }

    #[test]
    fn insert_multi_level_nesting() {
        let mut tree = DirTree::default();
        tree.insert(&[
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "deep.txt".to_string(),
        ]);
        let a = &tree.children["a"];
        let b = &a.children["b"];
        let c = &b.children["c"];
        assert_eq!(c.files, vec!["deep.txt"]);
    }

    // ── convert_and_sort ────────────────────────────────────────────

    #[test]
    fn convert_empty_tree_yields_empty_vec() {
        let tree = DirTree::default();
        assert!(convert_and_sort(tree).is_empty());
    }

    #[test]
    fn convert_sorts_dirs_before_files() {
        let mut tree = DirTree::default();
        tree.insert(&["a_file.txt".to_string()]);
        tree.insert(&["beta_dir".to_string(), "n.txt".to_string()]);
        tree.insert(&["alpha_dir".to_string(), "n.txt".to_string()]);

        let nodes = convert_and_sort(tree);
        assert_eq!(nodes.len(), 3);

        // Dirs come first, alphabetically sorted
        match &nodes[0] {
            TreeNode::Dir { name, .. } => assert_eq!(name, "alpha_dir"),
            other => panic!("expected Dir at [0], got {other:?}"),
        }
        match &nodes[1] {
            TreeNode::Dir { name, .. } => assert_eq!(name, "beta_dir"),
            other => panic!("expected Dir at [1], got {other:?}"),
        }
        // Files after all dirs
        match &nodes[2] {
            TreeNode::File { name } => assert_eq!(name, "a_file.txt"),
            other => panic!("expected File at [2], got {other:?}"),
        }
    }

    #[test]
    fn convert_sorts_files_alphabetically() {
        let mut tree = DirTree::default();
        tree.insert(&["z.txt".to_string()]);
        tree.insert(&["a.txt".to_string()]);
        tree.insert(&["m.txt".to_string()]);

        let nodes = convert_and_sort(tree);
        assert_eq!(nodes.len(), 3);

        match &nodes[0] {
            TreeNode::File { name } => assert_eq!(name, "a.txt"),
            other => panic!("expected File at [0], got {other:?}"),
        }
        match &nodes[1] {
            TreeNode::File { name } => assert_eq!(name, "m.txt"),
            other => panic!("expected File at [1], got {other:?}"),
        }
        match &nodes[2] {
            TreeNode::File { name } => assert_eq!(name, "z.txt"),
            other => panic!("expected File at [2], got {other:?}"),
        }
    }

    #[test]
    fn convert_sorts_dirs_alphabetically() {
        let mut tree = DirTree::default();
        tree.insert(&["zeta".to_string(), "f.txt".to_string()]);
        tree.insert(&["alpha".to_string(), "f.txt".to_string()]);

        let nodes = convert_and_sort(tree);
        assert_eq!(nodes.len(), 2);

        match &nodes[0] {
            TreeNode::Dir { name, .. } => assert_eq!(name, "alpha"),
            other => panic!("expected Dir at [0], got {other:?}"),
        }
        match &nodes[1] {
            TreeNode::Dir { name, .. } => assert_eq!(name, "zeta"),
            other => panic!("expected Dir at [1], got {other:?}"),
        }
    }

    #[test]
    fn convert_stable_order_dirs_mixed_with_files() {
        let mut tree = DirTree::default();
        tree.insert(&["aaa.txt".to_string()]);
        tree.insert(&["dir".to_string(), "x.txt".to_string()]);
        tree.insert(&["zzz.txt".to_string()]);
        tree.insert(&["bbb.txt".to_string()]);

        let nodes = convert_and_sort(tree);
        // Order: dir (alpha), then files sorted
        assert_eq!(nodes.len(), 4);
        match &nodes[0] {
            TreeNode::Dir { name, .. } => assert_eq!(name, "dir"),
            other => panic!("expected Dir first, got {other:?}"),
        }
        match &nodes[1] {
            TreeNode::File { name } => assert_eq!(name, "aaa.txt"),
            other => panic!("expected File, got {other:?}"),
        }
        match &nodes[2] {
            TreeNode::File { name } => assert_eq!(name, "bbb.txt"),
            other => panic!("expected File, got {other:?}"),
        }
        match &nodes[3] {
            TreeNode::File { name } => assert_eq!(name, "zzz.txt"),
            other => panic!("expected File, got {other:?}"),
        }
    }

    // ── print_move_tree (integration) ───────────────────────────────

    #[test]
    fn print_move_tree_empty_is_noop() {
        // Should not panic and produce no output
        let actions: Vec<MoveRecord> = vec![];
        let dir = tempfile::tempdir().unwrap();
        print_move_tree(&actions, dir.path());
        // No assertion needed — just checking it doesn't panic
    }

    #[test]
    fn print_move_tree_single_file_does_not_panic() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("target");
        let actions = vec![MoveRecord {
            dest_relative: PathBuf::from("Documents/PDF/report.pdf"),
            dry_run: true,
        }];
        print_move_tree(&actions, &target);
    }

    #[test]
    fn print_move_tree_multiple_files_does_not_panic() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("target");
        let actions = vec![
            MoveRecord {
                dest_relative: PathBuf::from("Documents/PDF/report.pdf"),
                dry_run: true,
            },
            MoveRecord {
                dest_relative: PathBuf::from("Documents/PDF/notes.pdf"),
                dry_run: true,
            },
            MoveRecord {
                dest_relative: PathBuf::from("Media/Audio/song.mp3"),
                dry_run: true,
            },
        ];
        print_move_tree(&actions, &target);
    }

    #[test]
    fn print_move_tree_deep_nesting_does_not_panic() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("target");
        let actions = vec![
            MoveRecord {
                dest_relative: PathBuf::from("a/b/c/d/e/f/g/deep.txt"),
                dry_run: true,
            },
            MoveRecord {
                dest_relative: PathBuf::from("a/b/c/sibling.txt"),
                dry_run: true,
            },
        ];
        print_move_tree(&actions, &target);
    }
}
