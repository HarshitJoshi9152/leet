use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestCaseBody {
    pub enable_debugger: bool,
    pub enable_run_code: bool,
    pub enable_submit: bool,
    pub enable_test_mode: bool,
    pub example_testcase_list: Vec<String>,
    pub meta_data: String,
    pub question_frontend_id: String,

    pub question_id: String,
    pub question_title: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub question: TestCaseBody
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTestCases {
    pub data: Question
}