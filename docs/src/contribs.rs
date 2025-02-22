use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Write;

use serde::Deserialize;

use super::Html;

/// Build HTML detailing the contributors between two tags.
pub fn contributors(from: &str, to: &str) -> Option<Html> {
    let staff = ["laurmaedje", "reknih"];

    let url = format!("https://api.github.com/repos/typst/typst/compare/{from}...{to}");
    let response: Response = ureq::get(&url)
        .set("X-GitHub-Api-Version", "2022-11-28")
        .call()
        .ok()?
        .into_json()
        .ok()?;

    // Determine number of contributions per person.
    let mut contributors = HashMap::<String, Contributor>::new();
    for commit in response.commits {
        contributors
            .entry(commit.author.login.clone())
            .or_insert_with(|| Contributor {
                login: commit.author.login,
                avatar: commit.author.avatar_url,
                contributions: 0,
            })
            .contributions += 1;
    }

    // Keep only non-staff people.
    let mut contributors: Vec<_> = contributors
        .into_values()
        .filter(|c| !staff.contains(&c.login.as_str()))
        .collect();

    // Sort by highest number of commits.
    contributors.sort_by_key(|c| Reverse(c.contributions));
    if contributors.is_empty() {
        return None;
    }

    let mut html = "Thanks to everyone who contributed to this release!".to_string();
    html += "<ul class=\"contribs\">";

    for Contributor { login, avatar, contributions } in contributors {
        let login = login.replace('\"', "&quot;").replace('&', "&amp;");
        let avatar = avatar.replace("?v=", "?s=64&v=");
        let s = if contributions > 1 { "s" } else { "" };
        write!(
            html,
            r#"<li>
              <a href="https://github.com/{login}" target="_blank">
                <img
                  width="64"
                  height="64"
                  src="{avatar}"
                  alt="GitHub avatar of {login}"
                  title="@{login} made {contributions} contribution{s}"
                  crossorigin="anonymous"
                >
              </a>
            </li>"#
        )
        .unwrap();
    }

    html += "</ul>";

    Some(Html::new(html))
}

#[derive(Debug)]
struct Contributor {
    login: String,
    avatar: String,
    contributions: usize,
}

#[derive(Debug, Deserialize)]
struct Response {
    commits: Vec<Commit>,
}

#[derive(Debug, Deserialize)]
struct Commit {
    author: Author,
}

#[derive(Debug, Deserialize)]
struct Author {
    login: String,
    avatar_url: String,
}
