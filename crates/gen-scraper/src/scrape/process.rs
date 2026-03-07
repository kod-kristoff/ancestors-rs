use hashbrown::{HashMap, HashSet};

use gen_types::{
    entities::{PersonBody, PlaceBody},
    shared::IriRef,
    value_objects::{Date, Fact, FactType, Identifier, IdentifierType, MemberInfo},
    Batch, Household, Person, Place,
};
// use gen_types::{conclusion::Person, Batch, Fact, FactType, Family};
// use miette::IntoDiagnostic;
use scraper::{Html, Selector};

use crate::ProcessError;

#[derive(Debug, Clone, Default)]
pub struct Context {
    visited_urls: HashSet<String>,
    to_visit: Vec<String>,
    persons: HashMap<String, Person>,
    households: HashMap<String, Household>,
    places: HashMap<String, Place>,
}

impl Context {
    pub fn with_start_urls(start_urls: Vec<String>) -> Self {
        Self {
            to_visit: start_urls,
            ..Default::default()
        }
    }

    pub fn next_url(&mut self) -> Option<String> {
        self.to_visit.pop()
    }
}
// use persons::application::service::AddPerson;

pub fn process(ctx: &mut Context, url: &str, src: &str) -> Result<(), ProcessError> {
    let html = Html::parse_document(src);
    // dbg!(&html);

    let ident = parse_ident(url).unwrap();
    dbg!(&ident);
    let riksarkivet_ns = Some(NS_RIKSARKIVET);
    let curr_person_opt: Option<Person> = ctx.persons.remove(ident);
    // batch.persons_mut().iter_mut().find(|p| {
    //     p.body()
    //         .identifiers()
    //         .iter()
    //         .any(|i| i.namespace() == riksarkivet_ns && i.id() == ident)
    // });
    dbg!(&curr_person_opt);
    let mut person = curr_person_opt.unwrap_or_else(Person::default);
    let mut curr_household = Household::default();
    extract_person(ctx, &mut person, &mut curr_household, &html)?;
    ctx.persons.insert(ident.to_string(), person);
    Ok(())
}

const BASE_URL: &str = "https://sok.riksarkivet.se";
const NS_RIKSARKIVET: &str = "http://riksarkivet.se";

fn extract_person(
    ctx: &mut Context,
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
                    .map(|s| s.trim())
                    .find(|s| s.chars().all(|c| c.is_ascii_digit()))
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
        // let mut hemort = None;
        // let mut hemparish = None;
        // let mut kontrakt = None;
        // let mut län = None;
        let mut mapping = HashMap::new();
        for div_faltdata in post.select(&div_faltdata_selector) {
            if let Some(field) = div_faltdata.select(&b_selector).next() {
                if let Some(link) = div_faltdata.select(&a_selector).next()
                // .map(|l| l.value().attr("href"))
                {
                    // dbg!(&link.html());
                    // dbg!(&div_faltdata.html());
                    if let Some(href) = link.attr("href") {
                        dbg!(&href);
                        if let Some(ident) = parse_ident(href) {
                            dbg!(&ident);
                            let new_url = format!("{}{}", BASE_URL, href);
                            if !ctx.visited_urls.contains(&new_url) {
                                ctx.to_visit.push(new_url.clone());
                            }
                            let iri = IriRef::parse(new_url).unwrap();
                            if !ctx.persons.contains_key(ident) {
                                let ident = ident.to_string();
                                let mut new_ident = Identifier::new(IdentifierType::Primary, iri);
                                new_ident.set_namespace(
                                    IriRef::parse(NS_RIKSARKIVET.to_string()).unwrap(),
                                );
                                new_ident.set_id(IriRef::parse(ident.clone()).unwrap());
                                let body = PersonBody::default().identifier(new_ident);
                                let new_person = Person::new(body, "scraper");
                                curr_household.update_body("scraper", |body| {
                                    body.add_member(MemberInfo::with_id(new_person.id()))
                                });
                                ctx.persons.insert(ident.to_string(), new_person);
                            }
                        }
                    }

                    let mut texts = link.text();
                    if let Some(field) = texts.next() {
                        dbg!(&field);
                    }
                    if let Some(field) = texts.next() {
                        dbg!(&field);
                    }
                    let mut div_texts = div_faltdata.text();
                    while let Some(text) = div_texts.next() {
                        if text.trim().is_empty() {
                            continue;
                        }
                        dbg!(&text);
                    }
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
                    mapping.insert(field.to_string(), texts[0].to_string());
                    dbg!(&mapping);
                    match field {
                        "Civilstånd" => {}
                        "Fam. nr 1" => {}
                        "Familj nr" => {}
                        "Famstkod" => {}
                        "Födelseförsamling" => {}
                        "Födelseår" => {}
                        "Hemförsamling" => {
                            // curr_household.add_fact(Fact::new(FactType::Living));
                            // hemparish = Some(texts[0].to_string());
                        }
                        "Hemort" => {
                            // hemort = Some(texts[0].to_string());
                        }
                        "Kontrakt" => {
                            // kontrakt = Some(texts[0].to_string());
                        }
                        "Kön" => {}
                        "Län" => {
                            // län = Some(texts[0].to_string());
                        }
                        "Namn" => {
                            new_person.update_body("scraper", |p| p.add_name(texts[0].into()));
                        }
                        "Om hushållet" => {}
                        "Rad" => {}
                        "Sida" => {}
                        "Upprättad av" => {}
                        "Yrke" => {}

                        _ => return Err(ProcessError::UnknownField(field.into())),
                    }
                    dbg!(&new_person);
                    dbg!(&curr_household);
                }
            } else {
                continue;
            }
            dbg!(&ctx);
            // dbg!(&div_faltdata.text().collect::<String>());
        }
        let birthplace = if let Some(födelseförsamling) = mapping.get("Födelseförsamling") {
            ctx.places
                .entry(födelseförsamling.to_string())
                .or_insert_with(|| {
                    Place::new(PlaceBody::new().original(födelseförsamling), "scraper")
                });
            ctx.places.get(födelseförsamling)
        } else {
            None
        };
        new_person.update_body("scraper", |body| {
            dbg!(&mapping);
            let mut birth = Fact::new(FactType::Birth);
            if let Some(birthyear) = mapping.get("Födelseår") {
                birth.set_date(Date::new().original(birthyear));
            }
            if let Some(birthplace) = birthplace {
                birth.set_place(birthplace.into());
            }
            let hemort = mapping.get("Hemort");
            let hemparish = mapping.get("Hemförsamling");
            let kontrkat = mapping.get("Kontrakt");
            let län = mapping.get("Län");
            if let Some(yrke) = mapping.get("Yrke") {
                body.add_fact(Fact::new(FactType::Occupation).value(yrke));
            }
        });
    }
    dbg!(&ctx);
    dbg!(&curr_household);
    Ok(())
    // Err(ProcessError::UnknownError("return something".into()))
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
