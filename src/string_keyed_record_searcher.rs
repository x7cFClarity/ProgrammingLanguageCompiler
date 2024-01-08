use std::option::Option;
use std::vec::Vec;

pub struct Node<Record> {
    pub character: char,
    pub children: Vec<Node<Record>>,
    pub record: Option<Record>
}

pub struct StringKeyedRecordSearcher<Record> {
    root_node: Node<Record>
}

impl<Record> StringKeyedRecordSearcher<Record> {
    pub fn find_mutably(&mut self, path: &[char], create_missing_segments: bool) -> Option<&mut Node<Record>> {
        let mut selector: &mut Node<Record> = &mut self.root_node;

        for path_segment_index in 0..path.len() {
            let path_segment = path[path_segment_index];
            let mut failed = false;

            {
                let potential_selector = selector
                    .children
                    .iter_mut()
                    .find(|child| child.character == path_segment);

                if let Some(mut new_selector) = potential_selector {
                    selector = new_selector;
                } else if !create_missing_segments {
                    return None;
                } else {
                    failed = true;
                }
            }

            if failed {
                selector.children.push(Node {
                    character: path_segment,
                    record: None,
                    children: Vec::new()
                });
            }

            selector = &mut selector.children[0];
        }

        Some(selector)
    }

    pub fn set_record(&mut self, label: &str, record: Record) {
        // let path = label.chars().collect();
        let path = &['a', 'b', 'c'];
        let reference_to_target = self.find_mutably(path, true);

        reference_to_target.unwrap().record = Some(record);
    }
}

impl<Record> Default for StringKeyedRecordSearcher<Record> {
    fn default()-> Self {
        Self {
            root_node: Node {
                character: 'f',
                children: Vec::new(),
                record: None
            }
        }
    }
}