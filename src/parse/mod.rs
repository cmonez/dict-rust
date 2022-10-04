use scraper::{Html, Selector};

const DICTIONARY_CLASS_CONTAINER: &str = "css-1avshm7";

pub mod parse {
    use super::*;

    pub fn get_definition(html_string: &str) {
        let document: Html = Html::parse_document(html_string);
        let selector = Selector::parse(format!("div.{}", DICTIONARY_CLASS_CONTAINER).as_str()).unwrap();
    }
}