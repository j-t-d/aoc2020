use attohttpc;
use config::{Config, File};
use snafu::{ResultExt, Snafu};
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Debug, Snafu)]
pub enum Error {
    Configuration { source: config::ConfigError },
    ParseUrl { source: url::ParseError },
    HttpGet { source: attohttpc::Error },
    Caching { source: std::io::Error, path: String },
    GetFailed { status: String },
}

pub struct Input {
    cache_path: PathBuf,
    url: Url,
    session: String,
}

impl Input {
    pub fn open<P: AsRef<Path>>(config: P) -> Result<Self, Error> {
        let mut settings = Config::default();
        settings
            .merge(File::with_name(config.as_ref().to_str().expect("Path")))
            .context(Configuration)?;
        let cache_path = settings.get_str("cache_path").context(Configuration)?;
        let url = settings.get_str("url").context(Configuration)?;
        let session = settings.get_str("session").context(Configuration)?;

        Ok(Self {
            cache_path: PathBuf::from(cache_path),
            session: session.to_string(),
            url: Url::parse(&url).context(ParseUrl)?,
        })
    }

    pub fn get(&self, day: u8) -> Result<String, Error> {
        let day = day.to_string();
        let input_path = self.cache_path.join(Path::new(&day)).join("input");
        let dir_path = self.cache_path.join(Path::new(&day));
        match fs::read_to_string(&input_path) {
            Ok(input) => Ok(input),
            Err(_) => {
                let mut new_url = self.url.clone();
                new_url.path_segments_mut().expect("Is base URL").extend(&["day", &day, "input"]);
                let input = attohttpc::get(new_url.as_str())
                    .header_append(attohttpc::header::COOKIE, format!("session={}", &self.session))
                    .send()
                    .context(HttpGet)?;
                if input.is_success() {
                    let input = input.text().context(HttpGet)?;
                    fs::create_dir_all(&dir_path).context(Caching {
                        path: dir_path.to_string_lossy(),
                    })?;
                    fs::write(&input_path, &input).context(Caching {
                        path: input_path.to_string_lossy(),
                    })?;
                    Ok(input)
                } else {
                    Err(Error::GetFailed {
                        status: input.status().to_string(),
                    })
                }
            }
        }
    }
}
