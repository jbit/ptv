use super::*;

impl From<::reqwest::Error> for Error {
    fn from(value: ::reqwest::Error) -> Self {
        Error::Other(value.to_string())
    }
}

/// Implementation of abstract HTTP client for the [reqwest](https://crates.io/crates/reqwest) library
impl PTVHttpClient for ::reqwest::Client {
    fn api_get<T: DeserializeOwned + Send + 'static>(&self, url: Url) -> FutureResult<T> {
        let request = self.get(url).send();
        Box::pin(async {
            let response = request.await?;
            let status = response.status();
            let text = response.text().await?;
            if status.is_success() {
                Ok(serde_json::from_str(&text)?)
            } else if let Ok(error) = serde_json::from_str(&text) {
                Err(Error::API(status, error))
            } else {
                Err(Error::HTTP(status, text))
            }
        })
    }
}

/// `reqwest` specific functions
impl PTV<::reqwest::Client> {
    /// Creates a new PTV client using the [reqwest](https://crates.io/crates/reqwest) HTTP library
    ///
    /// * `devid`: The Developer ID provided by PTV (i.e. `"0"`)
    /// * `key`: The Developer API key provided by PTV (i.e. `"00000000-0000-0000-0000-000000000000"`)
    /// * `user_agent`: The HTTP user agent that will be used (pick something unique for your app)
    ///
    /// See the [Official PTV Timetable API Page](https://www.ptv.vic.gov.au/footer/data-and-reporting/datasets/ptv-timetable-api/) for instructions on how to register for an API key
    pub fn new(devid: impl ToString, key: impl ToString, user_agent: impl ToString) -> Self {
        let client = ::reqwest::ClientBuilder::new()
            .user_agent(user_agent.to_string())
            .build()
            .expect("Failed to build reqwest Client for PTV API");
        Self {
            http_client: client,
            devid: devid.to_string(),
            key: key.to_string(),
            base: Url::parse(BASE_URL).expect("Failed to create base URL"),
        }
    }
}
