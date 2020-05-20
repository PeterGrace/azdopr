#[macro_use]
extern crate clap;
extern crate config;
extern crate serde_json;
use config::FileFormat;
use std::error::Error;
use clap::{App};
use dirs::home_dir;
use log::{warn};

mod repo_settings;


fn main() -> Result<(),Box<dyn(Error)>> {
    let config_path = home_dir().unwrap().join(".config/azdopr.yaml");
    let config_path_str = config_path.to_str().to_owned().unwrap();
    let mut settings = config::Config::default();
    settings
        .merge(config::File::new(&config_path_str, FileFormat::Yaml)).unwrap()
        .merge(config::Environment::with_prefix("AZDOPR")).unwrap();

    let cli_yaml = load_yaml!("cli.yaml");
    let _matches = App::from_yaml(cli_yaml).get_matches();

/*
 * curl -L -u azdo:$(System.AccessToken) -XPOST -H "Content-Type: application/json" -d@$(Build.StagingDirectory)/pr.json https://dev.azure.com/barracudanetworks/Janus/_apis/git/repositories/${REPOID}/pullrequests?api-version=5.1
*/
        let rs = repo_settings::RepoSettings::from_settings_and_args(settings, _matches);
        

        let params = [ ("api-version", "4.1")];
        let client = reqwest::blocking::Client::new();
        let url = format!("https://dev.azure.com/{}/{}/_apis/git/repositories/{}", rs.organization, rs.project, rs.repository);
        let getreq = client.get(&url)
            .basic_auth(&rs.pat,Some(&rs.pat))
            .form(&params)
            .send()?;

        if getreq.status().is_success() {
            if let r = serde_json::from_str(getreq.text().unwrap().as_str())? {
                println!("{:#?}", r);
            } else {
              warn!("could not decode json")
            }
        } else {
            println!("failure getting repo id");
        }

        Ok(())
}
