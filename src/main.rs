#[macro_use]
extern crate clap;
extern crate config;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use config::FileFormat;
use std::error::Error;
use clap::{App};
use dirs::home_dir;
use serde_json::{Value};
use anyhow::{anyhow, Result};
use reqwest::header::ACCEPT;

mod repo_settings;
mod pr_options;

fn get_repo_id(client: &reqwest::blocking::Client, rs: &repo_settings::RepoSettings) -> Result<String,anyhow::Error> {
        let params = [ ("api-version", "4.1")];
        let repo_id_url = format!("https://dev.azure.com/{}/{}/_apis/git/repositories/{}", rs.organization, rs.project, rs.repository);
        let getreq = client.get(&repo_id_url)
            .basic_auth(&rs.pat,Some(&rs.pat))
            .header(ACCEPT, "application/json; api-version=4.1")
            .send()?;

        if getreq.status().is_success() {
            let r: Value = serde_json::from_str(getreq.text().unwrap().as_str())?;
            let id = r["id"].to_string();
            Ok(id)
        } else {
            return Err(anyhow!("Did not succeed at repo id lookup"));
        }
}

fn create_pr(client: &reqwest::blocking::Client, rs: &repo_settings::RepoSettings) -> Result<String,anyhow::Error> {
/*
 * curl -L -u azdo:$(System.AccessToken) -XPOST -H "Content-Type: application/json" -d@$(Build.StagingDirectory)/pr.json https://dev.azure.com/barracudanetworks/Janus/_apis/git/repositories/${REPOID}/pullrequests?api-version=5.1
*/
    let create_url = format!("https://dev.azure.com/{}/{}/_apis/git/repositories/{}/pullRequests",
                             rs.organization,
                             rs.project,
                             rs._repo_id);

    let mut pr_completion: pr_options::CompletionOptions = Default::default();
    pr_completion.deleteSourceBranch = true;

    let mut pr_opts: pr_options::PROptions = Default::default();
    pr_opts.sourceRefName = format!("refs/heads/{}", rs.source_branch);
    pr_opts.targetRefName = format!("refs/heads/{}", rs.destination_branch);
    pr_opts.title = format!("{}", rs.title);
    pr_opts.description = format!("{}", rs.description);
    pr_opts.completionOptions = pr_completion;
    let jsonbody: String = serde_json::to_string(&pr_opts).unwrap();
    println!("{:#?}", jsonbody);
    let res = client.post(&create_url)
        .header(ACCEPT, "application/json; api-version=5.1")
        .basic_auth(&rs.pat,Some(&rs.pat))
        .json(&jsonbody)
        .send()?;

    if res.status().is_success() {
        println!("{}", res.text().unwrap());
        return Ok(String::from("good"));
//        Ok(res.text().unwrap())
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

        let mut rs = repo_settings::RepoSettings::from_settings_and_args(settings, _matches);
        let client = reqwest::blocking::Client::new();
        let repo_id:String = get_repo_id(&client,&rs)?;
        rs._repo_id = repo_id;
        create_pr(&client, &rs);


        Ok(())
}
