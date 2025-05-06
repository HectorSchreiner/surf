use core::error::Error;

use repos::{ListVulnerabilities, VulnerabilityRepo};

mod repos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // let vulnerability_repo = VulnerabilityRepo::new().await?;

    //let commits = vulnerability_repo.list_vulnerabilities(ListVulnerabilities {}).await?;
    //let release = vulnerability_repo.get_release_notes("cve_2025-04-27_1400Z").await?;


    let file = tokio::fs::File::open("");

    //println!("{commits:#?}");

    Ok(())
}