mod lib;
mod opt;

use anyhow::Result;
use hubcaps::{Credentials, Github};
use lib::{get_org_repos, get_user_repos};
use opt::Opt;
use regex::{Regex, RegexSet};
use std::env;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let github = Github::new(
        String::from("Github Stats"),
        env::var("GITHUB_TOKEN").ok().map(Credentials::Token),
    )?;

    let filtered = match opt.filter {
        Some(filtered) => Some(get_regexset(filtered)?),
        None => None,
    };

    println!("User repos");
    let user_repos = get_user_repos(&github, opt.stars, &filtered).await?;
    user_repos.iter().for_each(|repo| {
        println!("{:?} ★{}", repo.full_name, repo.stargazers_count);
    });

    println!("Org repos");
    let org_repos = get_org_repos(&github, opt.stars, &filtered).await?;
    org_repos.iter().for_each(|repo| {
        println!("{:?} ★{}", repo.full_name, repo.stargazers_count);
    });

    Ok(())
}

fn get_regexset(regexes: Vec<Regex>) -> Result<RegexSet> {
    Ok(RegexSet::new(regexes.into_iter().map(|r| r.to_string()))?)
}
