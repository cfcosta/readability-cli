use std::fs;
use std::io::prelude::*;
use std::io::stdin;
use std::path::PathBuf;

use dirs::cache_dir;
use failure::{Error, Fail};
use readability::extractor;
use structopt::StructOpt;
use url::{Host, Url};

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(name = "URL", default_value = "-")]
    target: String,

    #[structopt(name = "url")]
    /// Sets the URL for when input comes from stdin
    url: Option<String>,
}

#[derive(Debug, Fail)]
enum Errors {
    #[fail(display = "failed to get cache dir")]
    FailedToGetCacheDir,
    #[fail(display = "invalid url provided")]
    InvalidUrl,
}

fn main() -> Result<(), Error> {
    let options = Options::from_args();
    let url = Url::parse(&options.url.unwrap_or("http://localhost".into()))?;
    let mut cache_path: PathBuf = cache_dir().ok_or(Errors::FailedToGetCacheDir)?;

    match options.target.trim() {
        "-" => {
            let extractor = extractor::extract(&mut stdin(), &url)?;
            println!("<h1>{}</h1>{}", extractor.title, extractor.content);
        }
        url => {
            let parsed = Url::parse(url)?;
            let host = match parsed.host().ok_or(Errors::InvalidUrl)? {
                Host::Domain(domain) => Ok(domain),
                _ => Err(Errors::InvalidUrl),
            }?;
            let paths = parsed
                .path_segments()
                .map(|c| c.collect::<Vec<_>>())
                .ok_or(Errors::InvalidUrl)?;

            cache_path.push("readability-cli");
            cache_path.push(String::from(host));

            if !cache_path.is_dir() {
                fs::create_dir_all(&cache_path)?;
            }

            cache_path.push(format!("{}.html", paths.join(".")));

            if cache_path.is_file() {
                let mut file = fs::File::open(cache_path)?;
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;

                println!("{}", &buf);
            } else {
                let extractor = extractor::scrape(&url)?;

                let mut file = fs::File::create(cache_path)?;
                write!(file, "<h1>{}</h1>{}", extractor.title, extractor.content)?;
            }
        }
    }

    Ok(())
}
