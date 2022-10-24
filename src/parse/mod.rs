use scraper::{Html, Selector};

const DICTIONARY_SELECTOR_ONE: &str = "css-1avshm7";
const DICT_SELECTOR_TWO: &str = "e16867sm0";

pub mod parse {
    use scraper::ElementRef;
    use scraper::node::Element;
    use super::*;

    pub fn get_definition(html_string: &str) {
        let mut html_div = get_definition_html_div(html_string);
        println!("Here is the html div {}", html_div)
    }

    fn get_definition_html_div(html_string: &str) -> String {
        let mut document: Html = Html::parse_document(html_string);
        let selector = Selector::parse(format!(".{}, .{}", DICTIONARY_SELECTOR_ONE, DICT_SELECTOR_TWO).as_str()).unwrap();
        document.select(&selector).next().unwrap().inner_html()
    }
}