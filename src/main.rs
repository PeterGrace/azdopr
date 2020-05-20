use reqwest::Error;
use clap;
use clap::{App, Arg};

fn main() -> Result<(), Error> {
    let _matches = App::new("azdopr")
        .version("0.1.0")
        .about("Creates Azure DevOps PR based on arguments")
        .arg(
            Arg::with_name("source_branch")
            .short("s")
            .long("source_branch")
            .help("source branch to use for PR")
            .required(true)
            .takes_value(true)
            )
        .arg(
            Arg::with_name("destination_branch")
            .short("d")
            .long("destination_branch")
            .help("destination branch to use for PR")
            .default_value("master")
            )
        .get_matches();
/*
 * export REPOID=$(curl -L -u azdo:$(System.AccessToken) -XGET https://dev.azure.com/barracudanetworks/Janus/_apis/git/repositories/${{ parameters.destinationRepository}}?api-version=4.1|jq -r .id)
 * curl -L -u azdo:$(System.AccessToken) -XPOST -H "Content-Type: application/json" -d@$(Build.StagingDirectory)/pr.json https://dev.azure.com/barracudanetworks/Janus/_apis/git/repositories/${REPOID}/pullrequests?api-version=5.1
*/
        let organization = "barracudanetworks";
        let project = "Janus";
        let repository = "ccb-nonprod-flux-control";
        let pat = "pat";
        let params = [ ("api-version", "4.1")];
        let client = reqwest::blocking::Client::new();
        let url = format!("https://dev.azure.com/{}/{}/_apis/git/repositories/{}", organization, project, repository);
        let getreq = client.get(&url)
            .basic_auth(pat,Some(pat))
            .form(&params)
            .send()?;

        if getreq.status().is_success() {
            let response = getreq.json()?;
            println!("response: {:?}", response);
        } else {
            println!("failure getting repo id");
        }

        Ok(())
}
