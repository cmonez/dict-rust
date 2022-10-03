use reqwest::{get, Response};
const DICTIONARY_URL: &str = "https://www.dictionary.com/browse/";
const THESAURUS_URL: &str = "https://www.thesaurus.com/browse/";

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
        let formatted = format!("{}{}", DICTIONARY_URL, value);

        let body = reqwest::blocking::get(
            formatted,).unwrap()
            .text()
            .unwrap();

        println!("Here is the body {}", &body);
        fetch_web_page(Kind::DICTIONARY);
        return "hello".to_string();
    }

    pub fn get_synonym(value: &String) {
        let formatted = format!("{}{}", THESAURUS_URL, value);
    }

    pub fn get_antonym(value: &String) {
        let formatted = format!("{}{}", THESAURUS_URL, value);
    }

    fn fetch_web_page(kind: Kind) {
        if kind == Kind::DICTIONARY {
            println!("Were using dictionary")
        }
        else if kind == Kind::SYNONYM {
            println!("Were using synonym")
        }
        else if kind == Kind::ANTONYM {
            println!("Were using antonym")
        }
    }

}
