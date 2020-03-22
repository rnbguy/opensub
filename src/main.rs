use clap::{App, Arg};
use std::collections::BTreeMap;
use std::fmt::Display;
use url::Url;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct SubTitleResult {
    IDSubtitleFile: String,
    SubFileName: String,
    MovieReleaseName: String,
    MovieName: String,
    SubDownloadLink: String,
}

macro_rules! create_function {
    ($key:ident) => {
        fn $key<T: Display>(mut self, value: T) -> Self {
            self.0.insert(stringify!($key).into(), format!("{}", value));
            self
        }
    };
}

#[derive(Debug, Clone, Default)]
struct QueryParams(BTreeMap<String, String>);

#[allow(dead_code)]
impl QueryParams {
    // episode (number)
    create_function!(episode);

    // imdbid (always format it as sprintf("%07d", $imdb) - when using imdb you can add /tags-hdtv for example.
    create_function!(imdbid);

    // moviebytesize (number)
    create_function!(moviebytesize);

    // moviehash (should be always 16 character, must be together with moviebytesize)
    create_function!(moviehash);

    // query (use url_encode, make sure " " is converted to "%20")
    create_function!(query);

    // season (number)
    create_function!(season);

    // sublanguageid (if ommited, all languages are returned)
    create_function!(sublanguageid);

    // tag (use url_encode, make sure " " is converted to "%20")
    create_function!(tag);

    fn query_path(self) -> String {
        let v: Vec<_> = self.0.iter().map(|(k, v)| format!("{}-{}", k, v)).collect();
        v.join("/")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("opensub")
        .about("Subtitles from OpenSubtitles.org")
        .arg(
            Arg::with_name("query")
                .value_name("QRY")
                .help("Query name")
                .required(true)
                .multiple(true)
                .index(1),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .value_name("S E")
                .help("Session & episode number")
                .multiple(true)
                .number_of_values(2),
        )
        .arg(
            Arg::with_name("language")
                .short("l")
                .long("language")
                .value_name("LANG")
                .help("Subtitle language")
                .default_value("eng"),
        )
        .get_matches();

    let mut params = QueryParams::default();

    if let Some(q) = matches.values_of("query") {
        let qv: Vec<_> = q.collect();
        params = params.query(qv.join(" "));
    }

    if let Some(mut v) = matches.values_of("number") {
        params = params.season(v.next().unwrap());
        params = params.episode(v.next().unwrap());
    }

    if let Some(l) = matches.value_of("language") {
        params = params.sublanguageid(l);
    }

    let url = Url::parse(&format!(
        "https://rest.opensubtitles.org/search/{}",
        params.query_path()
    ))?;

    let client = reqwest::blocking::Client::builder().build()?;

    let result: Vec<SubTitleResult> = client
        .get(url)
        .header("X-User-Agent", "TemporaryUserAgent")
        .send()?
        .json()?;

    for i in 0..std::cmp::min(5, result.len()) {
        println!(
            "{} : {}",
            result[i].MovieReleaseName, result[i].SubDownloadLink
        );
    }

    Ok(())
}
