#[derive(Debug)]
pub struct Node {
    data: char,
    freq: u32,
    children: Vec<Option<Box<Node>>>,
}

impl Node {
    pub fn new(data: char) -> Self {
        Self {data: data, freq: 0, children: vec![None;26]}
    }
    pub fn get_value(&self) -> char {
        self.data
    }

    pub fn get_freq(&self) -> u32 {
        self.freq
    }
    pub fn get_children(&mut self) -> &mut Vec<Option<Box<Node>>> {
        &mut self.children
    }
    pub fn increment_freq(&mut self) {
        self.freq += 1;
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {data: '\0', freq: 0, children: vec![None;26]}
    }
}
impl Clone for Node {
    fn clone(&self) -> Self {
        Self {data: self.data, freq: self.freq, children: self.children.to_owned() }
    }
}

#[derive(Debug,Clone)]
pub struct Trie {
    root: Box<Node>,
    num_nodes: u32,
    num_words: u32
}

impl Default for Trie {
    fn default() -> Self {
        Self {root: Default::default(), num_nodes: 1, num_words: 0}
    }
}
impl Trie {
    pub fn new() -> Self {
        Self {root: Default::default(), num_nodes: 1, num_words: 0}
    }
    pub fn add(&mut self, word: &String) {
        let lower_word = word.to_lowercase();
        let mut curr_node = &mut self.root;// self.root;
       
        let mut i = 0;
        for letter in lower_word.as_str().chars() {
            let index :usize = letter.to_digit(36).unwrap() as usize - 10;

            if curr_node.get_children()[index].is_some() {
                curr_node = curr_node.get_children()[index].as_mut().unwrap();
            }
            else {
                    curr_node.get_children()[index] = Some(Box::new(Node::new(letter)));
                    curr_node = curr_node.get_children()[index].as_mut().unwrap(); 
                    self.num_nodes += 1;
            }
            if curr_node.get_freq() < 1 && i == lower_word.len() -1 {
                self.num_words += 1;
                curr_node.increment_freq();
            }
            else if curr_node.get_freq() > 0 && i == lower_word.len() -1 {
                curr_node.increment_freq();
            }
            i += 1;
        }
    }

    pub fn find(&mut self, word: &String) -> Option<&Node> {
        let lower_word = word.as_str().to_lowercase();
        let mut curr_node = &mut self.root;

        for letter in lower_word.chars() {
            let index :usize = letter.to_digit(36).unwrap() as usize - 10;
            match &mut curr_node.get_children()[index] {
                Some(next_node) => {
                    curr_node =  next_node; 
                }
                None => {
                    return None;
                }
            }
        }

        if curr_node.get_freq() >= 1 {
            return Some(&*curr_node);
        }

        return None;
    }

    pub fn get_word_count(&self) -> u32 {
        self.num_words
    }
    pub fn get_node_count(&self) -> u32 {
        self.num_nodes
    }
    fn to_string_helper(mut curr_node: Node, holder: &mut String, out: &mut String) {
        for i in 0..25 {
            
            match &curr_node.get_children()[i] {
                Some(next_node) => {
                    holder.push(next_node.get_value());
                    if next_node.get_freq() > 0 {
                        let output = holder.as_str().to_owned() + "\n";
                        *out += output.as_str();
                        //print!("{}",out);
                    }
                    Trie::to_string_helper(*next_node.clone(), holder, out);
                    holder.pop();
                }
                None => {
                    continue;
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        let mut holder = String::new();
        
        Self::to_string_helper(*self.root.clone(),&mut holder,&mut out);
        out.pop(); 

        out
    }
    
    pub fn hash_code(&mut self) -> i32 {
        let mut sum :i32 = 0;
        for i in 0..25 {
            if self.root.get_children()[i].is_some() {
                sum += i as i32;
            }
        }

        sum | self.num_nodes as i32 & self.num_words as i32
    }
    fn compare(mut base_node: Node, mut test_node: Node) -> bool {
        if base_node.get_freq() != test_node.get_freq() {
            return false;
        }

        for i in 0..25 {
            if base_node.get_children()[i].is_some() && test_node.get_children()[i].is_some() {

                if base_node.get_freq() != test_node.get_freq() {
                    return false;
                }
                else if !Trie::compare(*base_node.get_children()[i].as_ref().unwrap().clone(), *test_node.get_children()[i].as_ref().unwrap().clone()) {
                    return false;
                }
            }
        }
        true
    }
}
impl Eq for Trie {}

impl PartialEq for Trie {
    fn eq(&self, other: &Self) -> bool {
        if self.num_words != other.num_words {
            return false;
        }
        if self.num_nodes != other.num_nodes {
            return false;
        }

        Trie::compare(*self.root.clone(), *other.root.clone())
    }
}




#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
   
    
    fn setup() -> (Trie, Trie) {
        let trie1 = Trie::new();
        let trie2 = Trie::new();

        return (trie1,trie2);
    }

    #[test]
    fn test_single_word() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        
        assert_eq!(trie1.get_word_count(), 0, "Incorrect word count on empty trie.");
        trie1.add(&"cares".to_string());
        assert!(trie1.find(&"cares".to_string()).is_some(), "First word added wasn't found (\"cares\")");
        assert_eq!(6, trie1.get_node_count(), "Incorrect node count after 1 add");
        assert_eq!(1, trie1.get_word_count(), "Incorrect word count after 1 add");
    }

    #[test]
    fn test_two_words() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        
        trie1.add(&"cares".to_string());
        trie1.add(&"caress".to_string());
        assert!(trie1.find(&"caress".to_string()).is_some(), "Second word added wasn't found (\"caress\")");
        assert_eq!(7, trie1.get_node_count(), "Incorrect node count after second add");
        assert_eq!(2, trie1.get_word_count(), "Incorrect word count after 2 adds");
    }

    #[test]
    fn test_new_path() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        
        trie1.add(&"cares".to_string());
        trie1.add(&"caress".to_string());
        trie1.add(&"baboon".to_string());
        assert!(trie1.find(&"baboon".to_string()).is_some(), "New word added not found (\"baboon\")");
        assert_eq!(13, trie1.get_node_count(), "Incorrect node count after adding completely new word");
        assert_eq!(3, trie1.get_word_count(), "Incorrect word count after 3 adds");
    }

    #[test]
    fn test_prefix() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        
        trie1.add(&"cares".to_string());
        trie1.add(&"caress".to_string());
        trie1.add(&"baboon".to_string());
        trie1.add(&"car".to_string());
        assert!(trie1.find(&"car".to_string()).is_some(), "Prefix of first word not found (\"car\")");
        assert_eq!(13, trie1.get_node_count(), "Incorrect node count after adding no new nodes");
        assert_eq!(4, trie1.get_word_count(), "Incorrect word count after 4 adds");
    }

    #[test]
    fn test_equals() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        let trie2 = &mut pair.1;
        
        trie1.add(&"cares".to_string());
        trie1.add(&"caress".to_string());
        trie1.add(&"baboon".to_string());

        trie2.add(&"cares".to_string());
        trie2.add(&"caress".to_string());
        trie2.add(&"baboon".to_string());

        trie1.add(&"car".to_string());
        assert_eq!(trie1,trie1, "Trie found not equal to itself");
        assert_ne!(trie2, trie1, "Unequal Trie objects found equal (trie1 has word \"car\", where other trie doesn't)");
        
        trie2.add(&"car".to_string());

        assert_eq!(trie2, trie1, "Equal Trie objects found unequal");
        assert_eq!(trie2 == trie1, trie1 == trie2, "Trie's == operator is not commutative");

        trie2.add(&"car".to_string());

        assert_ne!(trie2, trie1, "Unequal trie objects found equal (both have word \"car\", but word frequency is different)");
    }

    #[test]
    fn test_more_equals() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        let trie2 = &mut pair.1;
        
        assert_eq!(trie2, trie1, "Two empty tries not found equal");

        for c in 'a'..='z' {
            trie1.add(&c.to_string());
        }
        
        assert_ne!(trie1, trie2, "One empty trie found equal to un-empty trie.");
        assert_ne!(trie2,trie1,"One empty trie found equal to un-empty trie.");
        
        for c in 'a'..='z' {
            trie2.add(&c.to_string());
        }

        assert_eq!(trie2,trie1,"Tries with a-z on root found unequal");

        trie1.add(&"jack".to_string());
        trie1.add(&"jackson".to_string());
        trie1.add(&"jackblack".to_string());
        trie1.add(&"janitor".to_string());
        trie2.add(&"jack".to_string());
        trie2.add(&"jackson".to_string());
        trie2.add(&"jackblack".to_string());
        trie2.add(&"janitor".to_string());

        assert_eq!(trie2,trie1, "Two equal branching tries found un-equal");

        trie1.add(&"jackblanco".to_string());

        assert_ne!(trie1,trie2, "Two un-equal branching tries found equal.");
        assert_ne!(trie2,trie1, "Two un-equal branching tries found equal.");
    }

    #[test]
    fn test_duplicate_nodes() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        

        trie1.add(&"cares".to_string());
        trie1.add(&"caress".to_string());
        trie1.add(&"baboon".to_string());
        trie1.add(&"car".to_string());
        trie1.add(&"car".to_string());

        
        assert_eq!(13, trie1.get_node_count(), "Incorrect node count after duplicate nodes");
        assert_eq!(4, trie1.get_word_count(), "Incorrect word count after duplicate adds");
    }

    #[test]
    fn test_find() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        

        trie1.add(&"cares".to_string());
        trie1.add(&"caress".to_string());
        trie1.add(&"baboon".to_string());
        trie1.add(&"car".to_string());
        trie1.add(&"car".to_string());

        assert_eq!(trie1.find(&"vnjklnasldkgnmb".to_string()).is_none(), true, "Found nonsense word (should have returned None)");
        assert_eq!(trie1.find(&"caresses".to_string()).is_none(), true, "Found \"caresses\" (chould have returned None)");
        assert_eq!(trie1.find(&'c'.to_string()).is_none(), true, "Found first letter of first word (chould have returned None)");
        assert_eq!(trie1.find(&"ca".to_string()).is_none(), true, "Found \"ca\" (prefix of first word) (chould have returned None)");
        assert_eq!(trie1.find(&"care".to_string()).is_none(), true, "Found \"care\" (prefix of first word) (chould have returned None)");
    }

    #[test]
    fn test_hash_code() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        let trie2 = &mut pair.1;
        

        trie1.add(&"cares".to_string());
        trie1.add(&"caress".to_string());
        trie1.add(&"baboon".to_string());
        trie1.add(&"car".to_string());
        trie1.add(&"car".to_string());

        trie2.add(&"cares".to_string());
        trie2.add(&"caress".to_string());
        trie2.add(&"baboon".to_string());
        trie2.add(&"car".to_string());
        trie2.add(&"car".to_string());

        assert_eq!(trie1.hash_code(), trie1.hash_code(), "Same Trie does not return the same hash code");
        assert_eq!(trie2.hash_code(), trie1.hash_code(), "Equal Trie object return uneqaul hash codes");
    }

    #[test]
    fn test_more_hash_code() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        let trie2 = &mut pair.1;
        

        trie1.add(&"dat".to_string());
        trie2.add(&"far".to_string());
        assert_ne!(trie2.hash_code(), trie1.hash_code(), "The hash code is too simple. Different Tries return same hash code");


        trie2.add(&"dat".to_string());
        trie1.add(&"far".to_string());
        assert_eq!(trie2.hash_code(),trie1.hash_code(), "Equal Tries of different construction history return different hash code");

        trie2.add(&"da".to_string());
        assert_ne!(trie2.hash_code(),trie1.hash_code(), "Tries of differing word cound return same hash code.");

        trie1.add(&"date".to_string());
        assert_ne!(trie2.hash_code(), trie1.hash_code(), "Tries of differing node count return same hash code");
        
        trie1.add(&"d".to_string());
        assert_ne!(trie2.hash_code(), trie1.hash_code(), "Different tries of same node count and word count return same hash code");
    }

    #[test]
    fn test_to_string() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        let trie2 = &mut pair.1;
        
        const TRIE_STRING :&str = "baboon\ncar\ncares\ncaress";
        const WRONG_TRIE_STRING :&str = "baboon\ncar\ncar\ncares\ncaress";

        trie1.add(&"cares".to_string());
        trie1.add(&"caress".to_string());
        trie1.add(&"baboon".to_string());
        trie1.add(&"car".to_string());
        trie1.add(&"car".to_string());

        trie2.add(&"cares".to_string());
        trie2.add(&"caress".to_string());
        trie2.add(&"baboon".to_string());
        trie2.add(&"car".to_string());
        trie2.add(&"car".to_string());
        
        assert_ne!(trie1.to_string().to_lowercase() == WRONG_TRIE_STRING || 
                   trie1.to_string().to_lowercase() == WRONG_TRIE_STRING.to_owned() + "\n", true, 
                   "Trie to_string() method has wrong count for (\"car\")");
        
        assert_eq!(trie1.to_string().to_lowercase() == TRIE_STRING || 
                   trie1.to_string().to_lowercase() == TRIE_STRING.to_owned() + "\n", true, 
                   "Trie to_string() method returns incorrect String\n\n");
        assert_eq!(trie1.to_string().to_lowercase() == trie2.to_string().to_lowercase(),true, "Equal Trie objects' to_string() methods return different Strings");
    }

    #[test]
    fn test_large_trie() {
        let mut pair = setup();
        let trie1 = &mut pair.0;
        let trie2 = &mut pair.1;
        
        const FILENAME :&str = "notsobig.txt";
        
        trie1.add(&"cares".to_string());
        trie1.add(&"caress".to_string());
        trie1.add(&"baboon".to_string());
        trie1.add(&"car".to_string());
        trie1.add(&"car".to_string());

        trie2.add(&"cares".to_string());
        trie2.add(&"caress".to_string());
        trie2.add(&"baboon".to_string());
        trie2.add(&"car".to_string());
        trie2.add(&"car".to_string());
         
        let file = fs::read_to_string(FILENAME);

        for line in file.expect("Unable to find dictionary").lines() {
            
            for word in line.split(' ') {
                let mut lower_word = word.to_lowercase();
                lower_word = lower_word.trim_end().to_string();
                trie1.add(&lower_word);
                trie2.add(&lower_word);
            }    
        }
        
        assert_eq!(78891,trie1.get_node_count(), "Incorrect node count after million+ word add (including many duplicates)");
        assert_eq!(trie2.get_node_count(),trie1.get_node_count(), "Equal Trie objects found unequal during million+ word add (including many duplicates)");
        assert_eq!(29157,trie1.get_word_count(), "Incorrect word count after many adds");
    }
}

