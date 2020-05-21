
#[derive(Serialize, Default)]
pub struct CompletionOptions {
    pub deleteSourceBranch: bool
}

#[derive(Serialize, Default)]
pub struct PROptions {
    pub sourceRefName: String,
    pub targetRefName: String,
    pub title: String,
    pub description: String,
    pub completionOptions: CompletionOptions
}
