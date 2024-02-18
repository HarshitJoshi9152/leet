

// Functions for calling the api

use reqwest::{cookie::Jar, header::{HeaderMap, HeaderValue}, Client, ClientBuilder, Error, Url};

use crate::graphql::test_case_body;

const ENDPOINT_URL: &'static str = "https://leetcode.com/graphql/";
const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0";


pub struct Api {
    // We need to store the 
    // url
    // Client struct to call the api
    // cookies
    // headers required
    client: Client,
    url: Url,
    cookie_jar: Jar,
    headers: HeaderMap
}

impl Api {
    // For API initialization we only need csrf_token and leetcode_session
    pub fn new(csrf_token: &str, leetcode_session: &str) -> Result<Self, Error>
    {
        let url = ENDPOINT_URL.parse::<Url>().unwrap();
        let cookie_jar = Jar::default();
        // construct the cookie string yourself

        let mut headers = HeaderMap::new();
        headers.insert("x-csrf-token", HeaderValue::from_str("src").unwrap());


        let mut api = Api {
            client: Client::new(),
            url: url.clone(),
            cookie_jar,
            headers
        };

        let cookie_str = format!("csrftoken={}; LEETCODE_SESSION={}", csrf_token, leetcode_session);
        api.cookie_jar.add_cookie_str(cookie_str.as_str(), &api.url);

        // api.client = ClientBuilder::new()
        //                 .user_agent(USER_AGENT)
        //                 .default_headers(headers)
        //                 .cookie_store(true)
        //                 .build()?;
        Ok(api)
    }
    // Now Implment the api methods

    pub async fn get_test_cases(&self, problem_slug: &str) -> String
    {
        let body = test_case_body(problem_slug);
        // Okay this is the RequestBuilder Pattern
        // I have to use the ClientBuilder 
        let response = self.client.post(self.url.as_str()).json(&body)
            .headers(self.headers.clone())    // TODO: Do i have to clone this struct everytime ????
            .send()
            .await
            .unwrap()
            .text().await.unwrap();
        response
    }
}
