use std::collections::BTreeMap;
use std::fmt::Display;
use url::Url;

use serde::Deserialize;

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

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(&format!(
        "https://rest.opensubtitles.org/search/{}",
        QueryParams::default()
            .query("brooklyn nine nine")
            .season(7)
            .episode(8)
            .sublanguageid("eng")
            .query_path()
    ))?;

    let client = reqwest::blocking::Client::builder().build()?;

    let result: Vec<SubTitleResult> = client
        .get(url)
        .header("X-User-Agent", "TemporaryUserAgent")
        .send()?
        .json()?;

    for i in 0..5 {
        println!("{} : {}", result[i].MovieReleaseName, result[i].SubDownloadLink);
    }

    
    Ok(())
}
