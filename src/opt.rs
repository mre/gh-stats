use anyhow::Result;
use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;

fn parse_regex(src: &str) -> Result<Regex> {
    Ok(Regex::new(src)?)
}

/// Github Stats
#[derive(StructOpt, Debug)]
pub struct Opt {
    /// Template file
    #[structopt(short, long, parse(from_os_str))]
    pub template: PathBuf,

    /// The minimum number of stars required to display a project
    #[structopt(short, long, default_value = "10")]
    pub stars: u64,

    /// Filter repositories (regex support)
    #[structopt(short, long, parse(try_from_str = parse_regex))]
    pub filter: Option<Vec<Regex>>,
}