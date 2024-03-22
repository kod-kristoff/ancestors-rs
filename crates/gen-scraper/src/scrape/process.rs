


use gen_types::{
    value_objects::{Fact, FactType},
    Batch, Household, Person,
};
// use gen_types::{conclusion::Person, Batch, Fact, FactType, Family};
// use miette::IntoDiagnostic;
use scraper::{Html, Selector};

use crate::ProcessError;

// use persons::application::service::AddPerson;

pub fn process(batch: &mut Batch, url: &str, src: &str) -> Result<(), ProcessError> {
    let html = Html::parse_document(src);
    // dbg!(&html);

    let ident = parse_ident(url);
    dbg!(&ident);
    let riksarkivet_ns = Some("http://riksarkivet.se");
    let curr_person_opt: Option<&mut Person> = batch.persons_mut().iter_mut().find(|p| {
        p.identifiers()
            .iter()
            .any(|i| i.namespace() == riksarkivet_ns && i.id() == ident)
    });
    dbg!(&curr_person_opt);
    if let Some(person) = curr_person_opt {
        let mut curr_household = Household::default();
        extract_person(person, &mut curr_household, &html)?;
    } else {
        let mut new_person = Person::default();
        let mut curr_household = Household::default();
        extract_person(&mut new_person, &mut curr_household, &html)?;
    }
    Ok(())
}
fn extract_person(
    new_person: &mut Person,
    curr_household: &mut Household,
    html: &Html,
) -> Result<(), ProcessError> {
    let selector = Selector::parse("article.hitRow").unwrap();
    let post_header_selector = Selector::parse("div.post_header").unwrap();
    let post_title_selector = Selector::parse("h1.post_title").unwrap();
    let post_type_selector = Selector::parse("div.post_type").unwrap();
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
                    .split(' ')
                    .map(|s| s.trim()).find(|s| s.chars().all(|c| c.is_ascii_digit()))
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
                        new_person.add_name(texts[0].into());
                    } else if field == "Hemförsamling" {
                        curr_household.add_fact(Fact::new(FactType::Living));
                    } else {
                        return Err(ProcessError::UnknownField(field.into()));
                    }
                    dbg!(&new_person);
                    dbg!(&curr_household);
                }
            } else {
                continue;
            }
            // dbg!(&div_faltdata.text().collect::<String>());
        }
    }
    Err(ProcessError::UnknownError("return something".into()))
}

fn parse_ident(url: &str) -> Option<&str> {
    let key_values = url.split('?').nth(1)?;
    let key_values = key_values.split('#').next()?;
    for key_value in key_values.split('&') {
        let mut key_value_iter = key_value.split('=');
        if let Some(key) = key_value_iter.next() {
            if key == "postid" {
                return key_value_iter.next();
            }
        }
    }
    None
}
