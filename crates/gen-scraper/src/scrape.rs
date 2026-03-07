mod process;

use std::{collections::HashMap, fs};

use gen_services::{repositories::SharedPersonRepository, service::Service};
use gen_types::Batch;
use ulid::Ulid;

use crate::{scrape::process::Context, ScrapeError};

pub fn scrape(
    service: Service,
    _out: &mut dyn std::io::Write,
    err: &mut dyn std::io::Write,
    start_urls: Vec<String>,
    fetched: &mut HashMap<String, String>,
) -> Result<(), ScrapeError> {
    writeln!(err, "scrape")?;
    let mut ctx = Context::with_start_urls(start_urls);
    let mut batch = Batch::new();
    while let Some(url) = ctx.next_url() {
        println!("requesting '{url}' ...");
        let body: String = ureq::get(&url).call().unwrap().into_string().unwrap();
        // println!("{}", body);
        let path = format!("{}.html", Ulid::new().to_string());
        fs::write(&path, &body).unwrap();
        fetched.insert(url.clone(), path);
        // .into_diagnostic()?;
        // let body = fs::read_to_string("output.html")?;
        process::process(&mut ctx, &url, &body)?;
        dbg!(&batch);
        dbg!(&ctx);
    }
    Ok(())
}
