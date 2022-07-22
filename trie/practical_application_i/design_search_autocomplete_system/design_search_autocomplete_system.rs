/*
 * Author: Nathan Bockisch
 * Date: July 21, 2022
 */
// A Trie node
struct Node {
    is_word: bool,
    hot_degree: i32,
    children: Box<[Option<Node>; 27]>,
}

impl Node {
    fn new() -> Self {
        Self {
            is_word: false,
            hot_degree: 0,
            children: Default::default(),
        }
    }
}

#[derive(Debug)]
struct SearchResult {
    word: String,
    hot_degree: i32,
}

struct AutocompleteSystem {
    root: Node,
    search: String,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AutocompleteSystem {

    // Initiate the autocomplete system and insert all sentences into the tree
    fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        let mut system = Self {
            root: Node::new(),
            search: String::new(),
        };

        let sentence_iter = sentences.iter().zip(times.iter());
        for (_, (sentence, hot_degree)) in sentence_iter.enumerate() {
            system.insert_node(sentence, *hot_degree);
        }

        system
    }

    // Insert a sentence and hot degree into the tree
    fn insert_node(&mut self, sentence: &String, hot_degree: i32) {
        let mut cur = &mut self.root;

        for c in sentence.chars() {
            let cur_index = Self::get_index(c);

            if let None = cur.children[cur_index] {
                cur.children[cur_index] = Some(Node::new());
            }

            cur = cur.children[cur_index].as_mut().unwrap();
        }

        cur.is_word = true;
        cur.hot_degree += hot_degree;
    }

    // Insert a character into the tree and get completion results
    fn input(&mut self, c: char) -> Vec<String> {
        if c == '#' { 
            self.insert_node(&self.search.clone(), 1);
            self.search.clear();
            return vec![] 
        }

        self.search.push(c);
        let mut cur = &mut self.root;

        for c in self.search.chars() {
            let cur_index = Self::get_index(c);

            if let None = cur.children[cur_index] {
                //self.insert_node(&self.search.clone(), 1);
                return vec![];
            }

            cur = cur.children[cur_index].as_mut().unwrap();
        }

        let mut sentences = Self::get_sentences(&cur, &self.search);
        sentences.sort_by_key(|result| result.word.clone());
        sentences.sort_by(|a, b| b.hot_degree.cmp(&a.hot_degree));
        sentences.truncate(3);

        let mut results = vec![];

        for result in sentences {
            results.push(result.word);
        }

        results
    }

    // Get a sentence from the tree, and return it along with its hot degree
    fn get_sentences(cur: &Node, word_so_far: &String) -> Vec<SearchResult> {
        let mut sentences = vec![];

        if cur.is_word {
            let result = SearchResult {
                word: word_so_far.to_string(),
                hot_degree: cur.hot_degree,
            };
            sentences.push(result);
        }

        for i in 0..cur.children.len() {
            if let Some(next) = &cur.children[i] {
                let mut new_word = word_so_far.clone();
                new_word.push(Self::get_char(i));
                let mut results_at_char = Self::get_sentences(&next, &new_word);
                sentences.append(&mut results_at_char);
            }
        }

        sentences
    }

    // Convert a char to a usize index
    fn get_index(c: char) -> usize {
        let c_byte = match c {
            ' ' => 'z' as u8 + 1,
            other => other as u8,
        };

        (c_byte - 'a' as u8) as usize
    }

    // Convert a usize index to a char
    fn get_char(i: usize) -> char {
        if i == Self::get_index(' ') {
            return ' ';
        }

        (i as u8 + 'a' as u8) as char
    }
}

/**
 * Your AutocompleteSystem object will be instantiated and called as such:
 * let obj = AutocompleteSystem::new(sentences, times);
 * let ret_1: Vec<String> = obj.input(c);
 */
