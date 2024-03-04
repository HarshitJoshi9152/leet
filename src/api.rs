

// Functions for calling the api

use std::sync::Arc;

use reqwest::{cookie::Jar, header::{HeaderMap, HeaderValue}, Client, ClientBuilder, Error, Url};

use crate::graphql::test_case_body;
use crate::responses::ResponseTestCases;

const ENDPOINT_URL: &'static str = "https://leetcode.com/graphql/";
const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0";


pub struct Api {
    client: Client,
}

impl Api {
    // For API initialization we only need csrf_token and leetcode_session
    pub fn new(csrf_token: &str, leetcode_session: &str) -> Result<Self, Error>
    {
        let url = ENDPOINT_URL.parse::<Url>().unwrap();
        let cookie_str = format!("csrftoken={}; LEETCODE_SESSION={}", csrf_token, leetcode_session);

        // Cookies
        let cj = Arc::new(Jar::default());
        cj.add_cookie_str(cookie_str.as_str(), &url);
        // Headers
        let mut headers = HeaderMap::new();
        headers.insert("x-csrf-token", HeaderValue::from_str("src").unwrap());

        let client = ClientBuilder::new()
                        .user_agent(USER_AGENT)
                        .default_headers(headers)
                        .cookie_provider(cj)
                        .build()?;
 
        let api = Api { client };

        Ok(api)
    }
    // Now Implment the api methods

    pub async fn get_test_cases(&self, problem_slug: &str) -> ResponseTestCases
    {
        // https://blog.ediri.io/serialize-and-deserialize-data-in-rust-using-serde-and-serdejson#heading-handling-arbitrary-json-data
        // remove graphql.rs and use serde_json::json!()
        let body = test_case_body(problem_slug);
        // Okay this is the RequestBuilder Pattern
        // I have to use the ClientBuilder 
        let response = self.client.post(ENDPOINT_URL).json(&body)
            .send()
            .await
            .unwrap()
            .json::<ResponseTestCases>().await.unwrap();
        response
    }
}
