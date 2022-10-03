const DICTIONARY_URL: &str = "https://www.dictionary.com/browse/";
const THESAURUS_URL: &str = "https://www.thesaurus.com/browse/";

pub mod fetch {
    use super::*;
    
    pub fn get_definition(value: &String) {
        let formatted = format!("{}{}", DICTIONARY_URL, value);
        println!("{}", formatted.to_string());
    }

    pub fn get_synonym(value: &String) {
        let formatted = format!("{}{}", THESAURUS_URL, value);
    }

    pub fn get_antonym(value: &String) {
        let formatted = format!("{}{}", THESAURUS_URL, value);
    }

}
