// Store all the request Body data here
// Better to make it into functions right now.....
// or maybe use some find/replace thing

use std::collections::HashMap;

// This is bad, this only exists to make the conversion to a HashMap less awkward
// which itself exists to be used as JSON
struct GraphqlQuery<'a> {
    query: &'a str,
    operation_name: &'a str,
    // variables itself is a map technically so to store it correctly we have to use some other HashMap and then Convert that to a JSON String to use here
    variables: &'a str
}

impl<'a> GraphqlQuery<'a> {
    fn new(query: &'a str, operation_name: &'a str, variables: &'a str ) -> Self
    {
        GraphqlQuery {
            query, operation_name, variables
        }
    }
}

impl<'a> Into<HashMap<String, String>> for GraphqlQuery<'a> {
    fn into(self) -> HashMap<String, String> {
        HashMap::from_iter([
            ("query".to_owned(), self.query.to_owned()),
            ("operationName".to_owned(), self.operation_name.to_owned()),
            ("variables".to_owned(), self.variables.to_owned()),
        ].into_iter())
    }
}


pub fn test_case_body(problem_slug: &str) -> HashMap<String, String>
{

    let query = "query consolePanelConfig($titleSlug: String!) {
                    question(titleSlug: $titleSlug) {
                        questionId
                        questionFrontendId
                        questionTitle
                        enableDebugger
                        enableRunCode
                        enableSubmit
                        enableTestMode
                        exampleTestcaseList
                        metaData
                    }
                }";
    let operation_name = "consolePanelConfig";
    let variables = format!("{{\"titleSlug\": \"{}\"}}", problem_slug);
    //

    let test_case_query = GraphqlQuery::new(
        query,
        operation_name,
        variables.as_str()
    );

    test_case_query.into()
}