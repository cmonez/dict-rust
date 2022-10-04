use scraper::*;

pub mod parse {
    pub fn get_definition(html_string: &str) {
        let document = scraper::Html::parse_document(html_string);
    }
}