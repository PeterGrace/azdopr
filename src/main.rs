#[macro_use]
extern crate clap;
extern crate config;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod repo_settings;
mod pr_options;

use config::FileFormat;
use std::error::Error;
use clap::App;
use dirs::home_dir;
use anyhow::{anyhow, Result};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::blocking::Client;
use repo_settings::RepoSettings;
use pr_options::{
    CompletionOptions,
    PROptions
};

fn create_pr(client: &Client, rs: &RepoSettings) -> Result<String,anyhow::Error> {
    let create_url = format!("https://dev.azure.com/{}/{}/_apis/git/repositories/{}/pullRequests",
                             rs.organization,
                             rs.project,
                             rs.repository);

    let mut pr_completion: CompletionOptions = Default::default();
    pr_completion.delete_source_branch = true;

    let mut pr_opts: PROptions = Default::default();
    pr_opts.source_ref = format!("refs/heads/{}", rs.source_branch);
    pr_opts.target_ref = format!("refs/heads/{}", rs.destination_branch);
    pr_opts.title = format!("{}", rs.title);
    pr_opts.description = format!("{}", rs.description);
    pr_opts.completion_options = pr_completion;
    let jsonbody: String = serde_json::to_string(&pr_opts).unwrap();
    println!("{:#?}", jsonbody);
    let res = client.post(&create_url)
        .header(ACCEPT, "application/json;api-version=5.1")
        .header(CONTENT_TYPE, "application/json")
        .basic_auth(&rs.pat,Some(&rs.pat))
        .body(jsonbody)
        .send()?;

    if res.status().is_success() {
        println!("{}", res.text().unwrap());
        return Ok(String::from("good"));
        //Ok(res.text().unwrap())
    } else {
        println!("{}", res.text().unwrap());
        return Err(anyhow!("post command failed."));
    }

}

fn main() -> Result<(),Box<dyn(Error)>> {
    let config_path = home_dir().unwrap().join(".config/azdopr.yaml");
    let config_path_str = config_path.to_str().to_owned().unwrap();
    let mut settings = config::Config::default();
    settings
        .merge(config::File::new(&config_path_str, FileFormat::Yaml)).unwrap()
        .merge(config::Environment::with_prefix("AZDOPR")).unwrap();

    let cli_yaml = load_yaml!("cli.yaml");
    let _matches = App::from_yaml(cli_yaml).get_matches();

        let rs = repo_settings::RepoSettings::from_settings_and_args(settings, _matches);
        let client = reqwest::blocking::Client::new();
        create_pr(&client, &rs)?;


        Ok(())
}
