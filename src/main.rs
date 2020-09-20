mod lib;
mod opt;

use anyhow::Result;
use hubcaps::{repositories::Repo, Credentials, Github};
use lib::{get_org_repos, get_user_repos};
use opt::Opt;
use regex::{Regex, RegexSet};
use std::{env, fs};
use structopt::StructOpt;
use tera::{Context, Tera};

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

    let user_repos = get_user_repos(&github, opt.stars, &filtered).await?;
    let org_repos = get_org_repos(&github, opt.stars, &filtered).await?;

    let mut context = Context::new();
    let mut repos: Vec<Repo> = user_repos
        .into_iter()
        .chain(org_repos.into_iter())
        .collect();
    repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    context.insert("repos", &repos);
    let tpl = Tera::one_off(&fs::read_to_string("template.md")?, &context, true)?;
    println!("{}", tpl);

    Ok(())
}

fn get_regexset(regexes: Vec<Regex>) -> Result<RegexSet> {
    Ok(RegexSet::new(regexes.into_iter().map(|r| r.to_string()))?)
}
