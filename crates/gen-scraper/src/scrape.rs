mod process;

use std::fs;

use gen_types::Batch;

use crate::ScrapeError;

pub fn scrape(
    out: &mut dyn std::io::Write,
    err: &mut dyn std::io::Write,
    start_urls: Vec<String>,
) -> Result<(), ScrapeError> {
    writeln!(err, "scrape")?;
    let url = &start_urls[0];
    // let body: String = ureq::get(url).call()?.into_string()?;
    // println!("{}", body);
    // fs::write("output.html", &body).into_diagnostic()?;
    let body = fs::read_to_string("output.html")?;
    let mut batch = Batch::new();
    process::process(&mut batch, &url, &body)?;
    dbg!(&batch);
    Ok(())
}
