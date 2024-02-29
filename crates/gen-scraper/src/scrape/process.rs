use std::fs;

use chrono::NaiveDate;
use gen_types::conclusion::Person;
// use miette::IntoDiagnostic;
use scraper::{Html, Selector};

use crate::ProcessError;

// use persons::application::service::AddPerson;

pub fn process(src: &str) -> Result<(), ProcessError> {
    let html = Html::parse_document(src);
    // dbg!(&html);
    let selector = Selector::parse("article.hitRow").unwrap();
    let post_header_selector = Selector::parse("div.post_header").unwrap();
    let post_title_selector = Selector::parse("h1.post_title").unwrap();
    let post_type_selector = Selector::parse("div.post_type").unwrap();

    let mut new_person = Person::default();

    for post in html.select(&selector) {
        // dbg!(&post.html());
        let (post_title, post_type, post_year) = {
            if let Some(post_header) = post.select(&post_header_selector).next() {
                let post_title = post_header
                    .select(&post_title_selector)
                    .next()
                    .unwrap()
                    .text()
                    .next()
                    .unwrap();
                let post_type = post_header
                    .select(&post_type_selector)
                    .next()
                    .unwrap()
                    .text()
                    .next()
                    .unwrap();
                let post_date = post_type
                    .split(" ")
                    .map(|s| s.trim())
                    .filter(|s| s.chars().all(|c| c.is_digit(10)))
                    .next()
                    .unwrap();
                // let year: i32 = year_str.parse().unwrap();
                // let post_date = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
                (post_title, post_type, post_date)
            } else {
                panic!("no post_header");
            }
        };
        // let td_ledtext_selector = Selector::parse("div.post_ledtext").unwrap();
        let div_faltdata_selector = Selector::parse("div.post_faltdata").unwrap();
        let b_selector = Selector::parse("b").unwrap();
        let a_selector = Selector::parse("a").unwrap();
        for div_faltdata in post.select(&div_faltdata_selector) {
            if let Some(field) = div_faltdata.select(&b_selector).next() {
                if let Some(link) = div_faltdata
                    .select(&a_selector)
                    .next()
                    .map(|l| l.value().attr("href"))
                {
                    dbg!(&link);
                } else if let Some(field) = field.text().next() {
                    let field = field.trim();
                    if field.is_empty() {
                        continue;
                    }
                    let texts: Vec<_> = div_faltdata
                        .text()
                        .map(|t| t.trim())
                        .filter(|s| !s.is_empty())
                        .filter(|s| *s != field)
                        .collect();
                    dbg!(&field);
                    dbg!(&div_faltdata.html());
                    dbg!(&texts);
                    if field == "Namn" {
                        new_person.name(texts[0]);
                    } else if field == "Hemförsamling" {
                        // new_person.fact()
                    } else {
                        return Err(ProcessError::UnknownField(field.into()));
                    }
                    dbg!(&new_person);
                }
            } else {
                continue;
            }
            // dbg!(&div_faltdata.text().collect::<String>());
        }
    }
    Err(ProcessError::UnknownError("return something".into()))
}
