use std::str::Chars;

pub struct Node<'a> {
    text: &'a str,
    line: usize,
    column: usize,
    index: usize
}

pub struct CharacterCollector<'a> {
    source_characters: Chars<'a>,
    current_position: usize,
    collected_string: String
}

impl<'a> CharacterCollector<'a> {
    pub fn new(source_string: &'a str, starting_position: usize) -> Self {
        CharacterCollector {
            source_characters: source_string.chars(),
            current_position: starting_position,
            collected_string: String::new()
        }
    }

    /// Write the currently tracked character to the collected string and increment the position counter.
    /// The next call of this function will be the next character corresponding to the position counter.
    pub fn collect_and_increment_position(&mut self) -> Option<char> {
        let potential_character = self.source_characters.next();
        if potential_character.is_none() {
            return None;
        }

        let character = potential_character.unwrap();
        self.collected_string.push(character);
        return Some(character);
    }


}

pub struct StringSplitter {

}

impl StringSplitter {

}