use scraper::{Html, Selector};

const TOP_LEVEL_DICTIONARY_DIV_SELECTOR: &str = ".css-1avshm7, .e16867sm0";
const PART_OF_SPEECH_SECTION_SELECTOR: &str = ".css-109x55k, .e1hk9ate4";

pub mod parse {
    use scraper::ElementRef;
    use scraper::node::Element;
    use super::*;

    pub fn get_definition(html_string: &str) {
        let mut html_div = get_top_level_definition_html_div(html_string);
        get_defintions(html_div.as_str());
    }

    fn get_top_level_definition_html_div(html_string: &str) -> String {
        let mut document: Html = Html::parse_document(html_string);
        let selector = Selector::parse(TOP_LEVEL_DICTIONARY_DIV_SELECTOR).unwrap();
        document.select(&selector).next().unwrap().inner_html()
    }
    fn get_defintions(html_string: &str)  {
        let mut document: Html = Html::parse_document(html_string);
        let selector = Selector::parse(PART_OF_SPEECH_SECTION_SELECTOR).unwrap();
        document.select(&selector).for_each(|item| {
            item.text().for_each(|text| {
                match &text[..1] {
                    "." => print!(""),
                    " " => print!(""),
                    _   => print!("{} ", text)
                }
            })
        })
    }
}