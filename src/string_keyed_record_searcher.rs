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
    pub fn find_mutably(&mut self, label: &str, create_missing_segments: bool) -> Option<&mut Node<Record>> {
        let mut path = label.chars();
        let mut selector: &mut Node<Record> = &mut self.root_node;

        for path_segment_index in 0..label.len() {
            let path_segment_option = path.nth(path_segment_index);
            if None == path_segment_option {
                return None;
            }

            let path_segment = path_segment_option.unwrap();

            let potential_selector = selector
                .children
                .iter_mut()
                .find(|child| child.character == path_segment);

            if let Some(new_selector) = potential_selector {
                selector = new_selector;
            } else if !create_missing_segments {
                return None;
            } else {
                selector.children.push(Node {
                    character: path_segment,
                    record: None,
                    children: Vec::new()
                });

                selector = &mut selector.children[0];
            }
        }

        Some(selector)
    }

    pub fn set_record(&mut self, label: &str, record: Record) {
        let reference_to_target = self.find_mutably(label, true);

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