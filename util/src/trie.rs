use std::collections::HashMap;

pub enum Trie {
    Value(u32),
    SubTree(HashMap<char, Trie>),
}

impl Trie {
    pub fn new(elements: Vec<(&str, u32)>) -> Trie {
        if elements.len() == 1 && elements[0].0.len() == 0 {
            return Trie::Value(elements[0].1);
        }

        let mut char_map: HashMap<char, Vec<(&str, u32)>> = HashMap::new();
        for element in elements {
            let mut chars = element.0.chars();

            let first_char = chars.next().unwrap();
            if !char_map.contains_key(&first_char) {
                char_map.insert(first_char, vec![]);
            }
            char_map
                .get_mut(&first_char)
                .unwrap()
                .push((chars.as_str(), element.1));
        }

        let mut final_map: HashMap<char, Trie> = HashMap::new();
        for char_mapping in char_map {
            final_map.insert(char_mapping.0, Trie::new(char_mapping.1));
        }

        return Trie::SubTree(final_map);
    }

    pub fn match_at(&self, text: &Vec<char>, start_index: usize) -> Option<u32> {
        return match self {
            Trie::Value(val) => Some(*val),
            Trie::SubTree(map) => {
                if text.len() == start_index {
                    return None;
                }

                let c = text[start_index];

                if !map.contains_key(&c) {
                    return None;
                }

                return map.get(&c).unwrap().match_at(text, start_index + 1);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_trie() {
        let res = Trie::new(vec![("yes", 5), ("no", 6)]);
        if let Trie::SubTree(map) = res {
            assert!(map.contains_key(&'y'));
            assert!(map.contains_key(&'n'));
        } else {
            panic!("Trie is not a SubTree");
        }
    }

    #[test]
    fn match_found() {
        let sut = Trie::new(vec![("yes", 5), ("no", 6)]);
        let res = sut.match_at(&"yesitis".chars().collect(), 0);
        if let Some(value) = res {
            assert_eq!(value, 5);
        } else {
            panic!("result not found");
        }
    }

    #[test]
    fn match_not_found() {
        let sut = Trie::new(vec![("yes", 5), ("no", 6)]);
        let res = sut.match_at(&"yesitis".chars().collect(), 1);
        if let Some(_) = res {
            panic!("result should not be found");
        }
    }
}
