use reqwest::{get, Response};
const DICTIONARY_URL: &str = "https://www.dictionary.com/browse/";
const THESAURUS_URL: &str = "https://www.thesaurus.com/browse/";

use super::parse::*;

pub mod fetch {
    use reqwest::StatusCode;
    use super::*;

    #[derive(PartialEq)]
    enum Kind {
        DICTIONARY,
        SYNONYM,
        ANTONYM,
    }

    pub fn get_definition(value: &String) ->  String {
        // print!("Fetching in defintion {}", fetch_web_page(Kind::DICTIONARY, value));
        parse::get_definition(&*fetch_web_page(Kind::DICTIONARY, value));
        return "hello".to_string();
    }

    pub fn get_synonym(value: &String) {
        let formatted = format!("{}{}", THESAURUS_URL, value);
    }

    pub fn get_antonym(value: &String) {
        let formatted = format!("{}{}", THESAURUS_URL, value);
    }

    fn fetch_web_page(kind: Kind, value: &String) -> String {
        let formatted: String;
        if kind == Kind::DICTIONARY {
            formatted = format!("{}{}", DICTIONARY_URL, value);
        } else {
            formatted = format!("{}{}", THESAURUS_URL, value);
        }

        let body = reqwest::blocking::get(
            formatted,).unwrap()
            .text()
            .unwrap();
        body
    }

}
