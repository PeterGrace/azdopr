
#[derive(Serialize, Default)]
pub struct CompletionOptions {
    #[serde(rename = "deleteSourceBranch")]
    pub delete_source_branch: bool
}

#[derive(Serialize, Default)]
pub struct PROptions {
    #[serde(rename = "sourceRefName")]
    pub source_ref: String,
    #[serde(rename = "targetRefName")]
    pub target_ref: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "completionOptions")]
    pub completion_options: CompletionOptions
}
