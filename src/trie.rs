use std::{path::PathBuf, collections::{HashMap, HashSet}};


#[allow(dead_code)]
pub struct Triee {
    root: TrieeNode,
}

#[allow(dead_code)]
impl <'a> Triee {

    pub fn new() -> Self {
        return Self { root: TrieeNode::new() };
    }

    pub fn paths_for_prefix(&self, word: &[char]) -> Option<&HashSet<PathBuf>> {
        return Self::paths_for_prefix_static(&self.root, word);
    }

    pub fn insert_word(&mut self, word: &[char], found_at: PathBuf) {
        Self::insert_static(&mut self.root, word, found_at);
    }

    pub fn print_ends(&self) {
        Self::ptrint_end_nodes(&self.root, Vec::new());
    }

    fn ptrint_end_nodes(starting_node: &TrieeNode, chars: Vec<char>) {
        if starting_node.is_end {
            println!("{}", chars.iter().collect::<String>());
            for p in starting_node.paths.iter() {
                println!("   {}", p.display());
            }
        }

        for (letter, node) in starting_node.children.iter() {
            let mut v = chars.clone();
            v.push(*letter);
            Self::ptrint_end_nodes(node, v);
        }
    }

    fn paths_for_prefix_static(starting_node: &'a TrieeNode, word: &[char]) -> Option<&'a HashSet<PathBuf>> {
        if word.is_empty() {
            return Some(&starting_node.paths);
        }

        if !starting_node.children.contains_key(&word[0]) {
            return None;
        }

        return Self::paths_for_prefix_static(
            starting_node.children.get(&word[0]).unwrap(),
            &word[1..]
        );

    }

    fn insert_static(starting_node: &mut TrieeNode, word: &[char], found_at: PathBuf) {

        if word.is_empty() { return; }

        let (current_char, is_last) = (word[0], word.len() == 1);

        match (current_char, is_last) {
            (letter, true) => {
                starting_node.children.entry(letter)
                    .and_modify(|v| {
                        v.is_end = true;
                        v.paths.insert(found_at.clone());
                    })
                    .or_insert(TrieeNode {
                        is_end: true,
                        children: HashMap::new(),
                        paths: hash_set_of(found_at.clone())
                    });
            }
            (letter, false) => {
                if starting_node.children.contains_key(&letter) {
                    let dst_child = starting_node.children.get_mut(&letter).unwrap();
                    dst_child.paths.insert(found_at.clone());
                    Self::insert_static(dst_child, &word[1..], found_at)
                } else {
                    starting_node.children.insert(letter, TrieeNode {
                        is_end: false,
                        children: HashMap::new(),
                        paths: hash_set_of(found_at.clone())
                    });
                    Self::insert_static(starting_node.children.get_mut(&letter).unwrap(), &word[1..], found_at);
                }
            }
        }
    }
}

fn hash_set_of<T: Eq + std::hash::Hash>(val: T) -> HashSet<T> {
    let mut t = HashSet::new();
    t.insert(val);
    t
}

struct TrieeNode {
    is_end: bool,
    children: HashMap<char, TrieeNode>,
    paths: HashSet<PathBuf>,
}

impl TrieeNode {
    fn new() -> Self {
        return TrieeNode {
            is_end: false,
            children: HashMap::new(),
            paths: HashSet::new(),
        };
    }
}
