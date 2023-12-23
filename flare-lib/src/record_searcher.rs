use std::option::Option;
use std::vec::Vec;

pub struct StringMatcherNode<Record> {
    pub character: char,
    pub children: Vec<StringMatcherNode<Record>>,
    pub record: Option<Record>
}

pub struct StringSearcher<Record> {
    root_node: StringMatcherNode<Record>
}

impl<Record> StringSearcher<Record> {
    pub fn new() -> Self {
        Self {
            root_node: StringMatcherNode {
                character: 'f',
                children: Vec::new(),
                record: None
            }
        }
    }

    pub fn find_mutably(&mut self, path: &[char]) -> Option<&mut StringMatcherNode<Record>> {
        let mut selector: &mut StringMatcherNode<Record> = &mut self.root_node;

        for path_segment_index in 0..path.len() {
            let path_segment = path[path_segment_index];
            let potential_selector = selector
                .children
                .iter_mut()
                .find(|child| child.character == path_segment);

            if let Some(mut new_selector) = potential_selector {
                selector = new_selector;
            } else {
                return None;
            }
        }

        Some(selector)
    }

    pub fn set_record(&mut self, label: &str, record: Record) {

    }
}