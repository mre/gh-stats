use anyhow::Result;
use hubcaps::repositories::{OrgRepoType, OrganizationRepoListOptions, Repo, Type};
use hubcaps::Github;
use hubcaps::{organizations::Org, repositories::UserRepoListOptions};
use regex::RegexSet;
use tokio::stream::StreamExt;

pub async fn get_org_repos(
    github: &Github,
    stars: u64,
    filtered: &Option<RegexSet>,
) -> Result<Vec<Repo>> {
    let orgs = github.orgs().list().await?;
    let valid_orgs: Vec<Org> = orgs
        .into_iter()
        .filter(|org| {
            if let Some(filtered) = filtered {
                !filtered.is_match(&*org.login)
            } else {
                true
            }
        })
        .collect();

    let opts = OrganizationRepoListOptions::builder()
        .repo_type(OrgRepoType::Public)
        .build();

    let mut found: Vec<Repo> = vec![];
    for org in valid_orgs {
        let repos: Vec<Result<Repo, _>> = github.org_repos(org.login).iter(&opts).collect().await;
        for repo in repos {
            let repo = repo?;
            if let Some(filtered) = filtered {
                if filtered.is_match(&repo.full_name) {
                    continue;
                }
            }
            if repo.stargazers_count > stars {
                found.push(repo);
            }
        }
    }
    Ok(found)
}

pub async fn get_user_repos(
    github: &Github,
    stars: u64,
    filtered: &Option<RegexSet>,
) -> Result<Vec<Repo>> {
    let opts = UserRepoListOptions::builder()
        .repo_type(Type::Public)
        .build();
    let mut found = vec![];
    let repos: Vec<Result<Repo, _>> = github.user_repos("mre").iter(&opts).collect().await;
    for repo in repos {
        let repo = repo?;
        if let Some(filtered) = filtered {
            if filtered.is_match(&repo.full_name) {
                continue;
            }
        }
        if repo.stargazers_count > stars {
            found.push(repo);
        }
    }
    Ok(found)
}
