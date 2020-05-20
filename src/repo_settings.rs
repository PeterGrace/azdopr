use config::Config;
use clap::{ArgMatches};
use log::{warn};

#[derive(Default)]
pub struct RepoSettings {
    pub organization: String,
    pub project: String,
    pub repository: String,
    pub pat: String,
    pub source_branch: String,
    pub destination_branch: String
}

impl RepoSettings {
    pub fn from_settings_and_args(settings: Config, matches: ArgMatches) -> RepoSettings {
        let mut rs:RepoSettings=Default::default();
        match settings.get("organization") {
            Ok(organization) => rs.organization=organization,
            Err(e) => warn!("{:?}",e)
        };
        match settings.get("project") {
            Ok(project) => rs.project=project,
            Err(e) => warn!("{:?}",e)
        };
        match settings.get("repository") {
            Ok(repository) => rs.repository=repository,
            Err(e) => warn!("{:?}",e)
        };
        match settings.get("pat") {
            Ok(pat) => rs.pat=pat,
            Err(e) => warn!("{:?}",e)
        };
        if let Some(v) = matches.value_of("source_branch") {
            rs.source_branch = v.to_string();
        }
        if let Some(v) = matches.value_of("destination_branch") {
            rs.destination_branch = v.to_string();
        }
        if let Some(v) = matches.value_of("repository") {
            rs.repository = v.to_string();
        }
        if let Some(v) = matches.value_of("organization") {
            rs.organization = v.to_string();
        }
        if let Some(v) = matches.value_of("project") {
            rs.project = v.to_string();
        }
        rs
    }
}
