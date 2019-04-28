use std::error::Error;
use std::io::stdin;

use readability::extractor;
use structopt::StructOpt;
use url::Url;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(name = "URL", default_value = "-")]
    target: String,

    #[structopt(name = "url")]
    /// Sets the URL for when input comes from stdin
    url: Option<String>,
}

fn main() -> Result<(), Box<Error>> {
    let options = Options::from_args();
    let url = Url::parse(&options.url.unwrap_or("http://localhost".into()))?;

    let html = match options.target.trim() {
        "-" => extractor::extract(&mut stdin(), &url),
        url => extractor::scrape(&url),
    }?
    .content;

    println!("{}", html);

    Ok(())
}
