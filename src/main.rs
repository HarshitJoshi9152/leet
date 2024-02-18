
mod api;
mod file_builder;
mod graphql;


#[tokio::main]
async fn main() {
    // Parse Cli arguments and take input from user (cookies and problem id / url)
    // pass variables to api client, which passes to query builder
    // ReqBody -> Query, operationName is unchanged always
    //            variables is changing, so we can generate that when returning


    let csrf_token = "";
    let leetcode_session = "";
    let problem_slug =  "sliding-window-maximum";

    let pi = api::Api::new(csrf_token, leetcode_session).unwrap();
    let res = pi.get_test_cases(problem_slug).await;
    dbg!(res);

    // random-uuid
    // 	1cfdecc7-469e-ac27-bb4b-6e9b34baa198
    // Fetch the requests and get the starter code + testcases + description 
}