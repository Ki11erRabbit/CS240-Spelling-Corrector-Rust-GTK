mod trie;


use std::fs;

use std::collections::HashSet;

#[derive(Clone)]
pub struct SpellCorrector {
    dictionary: trie::Trie,
}

impl SpellCorrector {

    pub fn new() -> Self {
        Self {dictionary: trie::Trie::new()}
    }
    pub fn use_dictionary(&mut self, dictionary_file_name: String) {
        self.dictionary = trie::Trie::new();
        let file = fs::read_to_string(dictionary_file_name);
        
        for line in file.expect("Unable to find dictionary").lines() {
            
            for word in line.split(' ') {
                let mut lower_word = word.to_lowercase();
                lower_word = lower_word.trim_end().to_string();
                self.dictionary.add(&lower_word.to_string());
            }
            
        }
    }

    pub fn suggest_similar_word(&mut self, input_word: String) -> Result<String, String> {
        let lower_word = input_word.to_lowercase();

        match self.dictionary.find(&lower_word) {
            None => {},
            Some(_v) => return Ok(lower_word)
        }

        let mut edit_dist1 : HashSet<Box<String>> = HashSet::new();
        self.gen_edit_dist1(&mut edit_dist1, lower_word);
        //println!("{:?}", self.edit_dist1);
        //println!("edit dist 1 size: {}",edit_dist1.len());
        //println!("Finding Matches in Edit Distance 1");
        let mut matches = Vec::new();
        for word in edit_dist1.iter() {
            match self.dictionary.find(&word) {
                None => {},
                Some(_v) => matches.push(word)
            }
        }

        let mut output : (Option<String>, u32) = (None, 0);
        //println!("finding Highest Freq in matches");
        for matched_word in matches.iter() {
            let pair;
            match self.dictionary.find(&matched_word) {
                None => continue,
                Some(node) => pair = (Some(matched_word.to_string()), node.get_freq())
            }

            if output.1 < pair.1 {
                output = pair;
            }
            else if output.1 == pair.1 {
                if output.0 > pair.0 {
                    output = pair;
                }
            }

            
        }
        //println!("{:?}", matches); 
        if output.0.is_some() {
            return Ok(output.0.unwrap().to_string());
        }

        let mut matches2 = Vec::new();
        
        let mut edit_dist2 : HashSet<Box<String>> = HashSet::new();
        self.gen_edit_dist2(&mut edit_dist2,&edit_dist1);
        //println!("edit dist 2 size: {}",edit_dist2.len());
        //println!("Finding Matches in Edit Distance 2");
        for word in edit_dist2.iter() {
            //println!("{}",word);
            match self.dictionary.find(&word) {
                None => {},
                Some(_v) => matches2.push(word)
            }
        }
        
        //println!("{:?}", self.edit_dist2);
        //println!("finding Highest Freq in matches");
        
        for matched_word in matches2.iter() {
            let pair;
            match self.dictionary.find(&matched_word) {
                None => continue,
                Some(node) => pair = (Some(matched_word.to_string()), node.get_freq())
            }

            if output.1 < pair.1 {
                output = pair;
            }
            else if output.1 == pair.1 {
                if output.0 > pair.0 {
                    output = pair;
                }
            }
            
        }
       
        //println!("{:?}", matches); 
        if output.0.is_some() {
            return Ok(output.0.unwrap().to_string());
        }
        
        Err("Unable to find word \"".to_string() + &input_word + &"\"".to_string())
    }

    fn delete_char(&mut self,words: &mut HashSet<Box<String>> , word: & String) {
        //println!("length of word {}",word.chars().count());
        for i in 0..(word.chars().count()) {
            let mut new_word = Box::new(word.clone());
            new_word.drain(i..i+1);
            
            words.insert(new_word);
        }
    }

    fn transpose_char(&mut self,words: &mut HashSet<Box<String>> ,word: &String) {
        for i in 0..(word.chars().count()) {
            let char1 = word.chars().nth(i).unwrap();
            for j in 1..(word.chars().count()) {
                let char2 = word.chars().nth(j).unwrap();
                if char1 == char2 {
                    continue;
                }
                //println!("{} {}",char1.to_string(), char2.to_string());
                let mut new_word = Box::new(word.clone());

                new_word.replace_range(i..i+1, &char2.to_string());
                new_word.replace_range(j..j+1, &char1.to_string());
                //println!("\t{} {}",word,new_word);
                words.insert(new_word);
            }
        }
    }

    fn alternate_char(&mut self,words: &mut HashSet<Box<String>> ,word: &String) {
        for i in 0..(word.chars().count()) {
            for c in 'a'..'z' {
                let mut new_word = Box::new(word.clone());
                
                new_word.replace_range(i..i+1, &c.to_string());

                words.insert(new_word);
            }
        }
    }

    fn insert_char(&mut self,words: &mut HashSet<Box<String>> ,word: &String) {
        for i in 0..=(word.chars().count()) {
            for c in 'a'..='z' {
                let mut new_word = Box::new(word.clone());

                new_word.insert(i, c);

                words.insert(new_word);
            }
        }

    }

    fn gen_edit_dist1(&mut self, edit_dist1: &mut HashSet<Box<String>>, word: String)  {
        self.delete_char(edit_dist1, &word);
        self.transpose_char(edit_dist1, &word);
        self.alternate_char(edit_dist1, &word);
        self.insert_char(edit_dist1, &word);


/*
        for new_word in self.delete_char(&word) {
            edit_dist1.insert(new_word);
        }
        for new_word in self.transpose_char(&word) {
            edit_dist1.insert(new_word);
        }
        for new_word in self.alternate_char(&word) {
            edit_dist1.insert(new_word);
        }
        for new_word in self.insert_char(&word) {
            edit_dist1.insert(new_word);
        }*/
    }

    fn gen_edit_dist2(&mut self, edit_dist2: &mut HashSet<Box<String>>, words: &HashSet<Box<String>>) {

        for word in words.iter() {
            self.delete_char(edit_dist2, &word);
            self.transpose_char(edit_dist2, &word);
            self.alternate_char(edit_dist2, &word);
            self.insert_char(edit_dist2, &word);
        }
        /*for word in words.iter() {
            for new_word in self.delete_char(&word) {
                edit_dist2.insert(new_word);
            }
            for new_word in self.transpose_char(&word) {
                edit_dist2.insert(new_word);
            }
            for new_word in self.alternate_char(&word) {
                edit_dist2.insert(new_word);
            }
            for new_word in self.insert_char(&word) {
                edit_dist2.insert(new_word);
            }
        }*/
    }
    
}

#[cfg(test)]
mod tests {

    use super::*;

    const WORD_FILENAME :&str = "word.txt";
    const WORDS_FILENAME :&str = "words.txt";
    const BIG_FILENAME :&str = "notsobig.txt";
    const WORD :&str = "yea";

    fn setup() -> SpellCorrector {
        return SpellCorrector::new();
    }

    fn test(filename: &str, word: &str, corrector: &mut SpellCorrector) -> Option<String> {
        let suggestion;

        corrector.use_dictionary(filename.to_string());

        suggestion = corrector.suggest_similar_word(word.to_string());

        match suggestion {
            Ok(word) => return Some(word),
            Err(_v) => return None,
        }
    }

    fn create_error_message(guess: &str, expected: &str, suggested: &String) -> String {
        return format!("Guessed: {} Expected: {} Actual: {}",guess,expected,suggested);
    }

    #[test]
    fn test_valid_word() {
        let mut corrector = setup();
       
        let suggested_word = test(WORD_FILENAME, WORD, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "Same spelling of expected word.");
        
        let suggested_word = test(WORD_FILENAME, &WORD.to_lowercase(), &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "Lower case of expected word.");
        
        let suggested_word = test(WORD_FILENAME, &WORD.to_uppercase(), &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "Upper case of expected word.");
        
        let suggested_word = test(WORD_FILENAME, WORD, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "Same spelling of expected word.");
    }

    #[test]
    fn test_insertion() {
        let mut corrector = setup();

        let guess: &str = "ye"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "ea"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "ya"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }

    #[test]
    fn test_deletion() {
        let mut corrector = setup();

        let guess: &str = "yaea"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "ybea"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "ryea"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "ygea"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }

    #[test]
    fn test_alteration() {
        let mut corrector = setup();

        let guess: &str = "flobt"; 
        let suggested_word = test(WORDS_FILENAME, guess, &mut corrector);
        assert_eq!("float",suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, "float",&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "bloat"; 
        let suggested_word = test(WORDS_FILENAME, guess, &mut corrector);
        assert_eq!("float",suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, "float",&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "reah"; 
        let suggested_word = test(WORDS_FILENAME, guess, &mut corrector);
        assert_eq!("yeah",suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, "yeah",&suggested_word.as_ref().unwrap()));
    }

    #[test]
    fn test_transposition() {
        let mut corrector = setup();

        let guess: &str = "yaeh"; 
        let suggested_word = test(WORDS_FILENAME, guess, &mut corrector);
        assert_eq!("yeah",suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, "yeah",&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "flaot"; 
        let suggested_word = test(WORDS_FILENAME, guess, &mut corrector);
        assert_eq!("float",suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, "float",&suggested_word.as_ref().unwrap()));    
    }

    #[test]
    fn test_insertion_insertion() {
        let mut corrector = setup();

        let guess: &str = "e"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "a"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "y"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
    
    #[test]
    fn test_insertion_deletion() {
        let mut corrector = setup();

        let guess: &str = "yez"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "efa"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "rya"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
    
    #[test]
    fn test_insertion_alteration() {
        let mut corrector = setup();

        let guess: &str = "er"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "qa"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "yf"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
    
    #[test]
    fn test_insertion_transposition() {
        let mut corrector = setup();

        let guess: &str = "ae"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "ey"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
    
    #[test]
    fn test_deletion_insertion() {
        let mut corrector = setup();

        let guess: &str = "yar"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "fya"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "yad"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
     
    #[test]
    fn test_deletion_deletion() {
        let mut corrector = setup();

        let guess: &str = "yeakg"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "jkyea"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "vyfea"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "cyean"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
    
    #[test]
    fn test_deletion_alteration() {
        let mut corrector = setup();

        let guess: &str = "ydef"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "vyga"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "ymca"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
    
    #[test]
    fn test_deletion_transposition() {
        let mut corrector = setup();

        let guess: &str = "yade"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "epya"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "yame"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
    
    #[test]
    fn test_alteration_insertion() {
        let mut corrector = setup();

        let guess: &str = "fe"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "va"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "yy"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
     
    #[test]
    fn test_alteration_deletion() {
        let mut corrector = setup();

        let guess: &str = "feia"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "yqex"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "yqax"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
      
    #[test]
    fn test_alteration_alteration() {
        let mut corrector = setup();

        let guess: &str = "vda"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "xel"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "yhb"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
      
    #[test]
    fn test_alteration_transposition() {
        let mut corrector = setup();

        let guess: &str = "yac"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "gya"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "eja"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
     
    #[test]
    fn test_transposition_insertion() {
        let mut corrector = setup();

        let guess: &str = "ay"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "ae"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "ey"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
     
    #[test]
    fn test_transposition_deletion() {
        let mut corrector = setup();

        let guess: &str = "ycae"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "eyae"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "eyma"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
     
    #[test]
    fn test_transposition_alteration() {
        let mut corrector = setup();

        let guess: &str = "yac"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "bya"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "yle"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
    }
     
    #[test]
    fn test_transposition_transposition() {
        let mut corrector = setup();

        let guess: &str = "eay"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "aye"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert_eq!(WORD,suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, WORD,&suggested_word.as_ref().unwrap()));    
    }
    
    #[test]
    fn test_no_similar_words() {
        let mut corrector = setup();

        let guess: &str = ""; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert!(suggested_word.is_none(), "Guessed empty String");
        
        let guess: &str = "lol"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert!(suggested_word.is_none(), "Guessed dissimilar string of same length");
        
        let guess: &str = "abcdefghijklmnopqrstuvqxyz"; 
        let suggested_word = test(WORD_FILENAME, guess, &mut corrector);
        assert!(suggested_word.is_none(), "Guessed dissimilar string of much longer length");
    }
     
    #[test]
    fn test_choose_closest_word() {
        let mut corrector = setup();

        let suggested_word = test(WORDS_FILENAME, "ye", &mut corrector);
        assert_eq!("yea",suggested_word.as_ref().unwrap(), "Choosing edit distance one before two");
        
        let suggested_word = test(WORDS_FILENAME, "yes", &mut corrector);
        assert_eq!("yea",suggested_word.as_ref().unwrap(), "Choosing edit distance one before two");    
        
        let suggested_word = test(WORDS_FILENAME, "yeaz", &mut corrector);
        assert_eq!("yeah",suggested_word.as_ref().unwrap(), "Choosing word with higher frequency");
        
        let suggested_word = test(WORDS_FILENAME, "yeahj", &mut corrector);
        println!("{:?}",corrector.dictionary);
        assert_eq!("yeah",suggested_word.as_ref().unwrap(), "Choosing first word alphabetically when equal frequency");    
    }
     
    #[test]
    fn test_big_file() {
        let mut corrector = setup();

        let guess: &str = "Jason"; 
        let suggested_word = test(BIG_FILENAME, guess, &mut corrector);
        assert_eq!("jason",suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, "jason",&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "is"; 
        let suggested_word = test(BIG_FILENAME, guess, &mut corrector);
        assert_eq!("is",suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, "is",&suggested_word.as_ref().unwrap()));    

        let guess: &str = "zupem"; 
        let suggested_word = test(BIG_FILENAME, guess, &mut corrector);
        assert_eq!("super",suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, "super",&suggested_word.as_ref().unwrap()));
        
        let guess: &str = "cooool"; 
        let suggested_word = test(BIG_FILENAME, guess, &mut corrector);
        assert_eq!("cool",suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, "cool",&suggested_word.as_ref().unwrap()));    
        
        let guess: &str = "absolustly"; 
        let suggested_word = test(BIG_FILENAME, guess, &mut corrector);
        assert_eq!("absolutely",suggested_word.as_ref().unwrap(), "{}", create_error_message(guess, "absolutely",&suggested_word.as_ref().unwrap()));    
    }
    
}
