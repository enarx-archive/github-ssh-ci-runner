use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header;
use serde_derive::Deserialize;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::time::Duration;
use void::Void;

#[cfg(test)]
mod tests;

#[derive(Debug, Deserialize)]
struct GithubReferenceObject {
    r#type: String,
    sha: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct GithubReference {
    r#ref: String,
    node_id: String,
    url: String,
    object: GithubReferenceObject,
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub runner: Vec<String>,
    pub repos: Option<Vec<String>>,
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Debug)]
pub struct CiRunner {
    pub github_token: String,
    pub git_repo: String,
    pub git_sha: String,
    pub git_ref: String,
    pub config: Config,
}

impl CiRunner {
    pub fn run(config_file: String) -> Result<Void, Box<dyn std::error::Error>> {
        /*
        SSH_ORIGINAL_COMMAND must contain in this order:
        GITHUB_TOKEN
        GITHUB_REPOSITORY
        GITHUB_SHA
        GITHUB_REF
         */
        let mut contents = String::new();
        std::fs::File::open(&config_file)?.read_to_string(&mut contents)?;

        let config = toml::from_str(&contents)?;

        let orig_params = std::env::var("SSH_ORIGINAL_COMMAND")?;

        let params = orig_params.split_ascii_whitespace().collect::<Vec<_>>();

        assert_eq!(params.len(), 4);

        let ci_runner = CiRunner {
            github_token: String::from(params[0]),
            git_repo: String::from(params[1]),
            git_sha: String::from(params[2]),
            git_ref: String::from(params[3]),
            config,
        };
        ci_runner.sanity_check_params()?;

        ci_runner.check_repo_allowed()?;

        ci_runner.check_token()?;

        ci_runner.run_ci()
    }

    pub fn sanity_check_params(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check for valid syntax
        dbg!(&self.git_repo);
        dbg!(&self.git_sha);
        dbg!(&self.git_ref);

        if !Regex::new(r"^v1\.[0-9a-f]{40}$")
            .unwrap()
            .is_match(&self.github_token)
        {
            return Err("Invalid GITHUB_TOKEN".into());
        }

        if !Regex::new(r"^[[:alnum:]\-_]+/[[:alnum:]\-_]+$")
            .unwrap()
            .is_match(&self.git_repo)
        {
            return Err("Invalid GITHUB_REPOSITORY".into());
        }

        if !Regex::new(r"^[0-9a-f]{40}$")
            .unwrap()
            .is_match(&self.git_sha)
        {
            return Err("Invalid GITHUB_SHA".into());
        }

        if !Regex::new(r"^refs/(?:pull/[[:digit:]]+/(?:head|merge)|heads/[\S&&[^$`()]]+)$")
            .unwrap()
            .is_match(&self.git_ref)
        {
            return Err("Invalid GITHUB_REF".into());
        }

        Ok(())
    }

    pub fn check_repo_allowed(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(repos) = self.config.repos.as_ref() {
            if !repos.contains(&self.git_repo) {
                return Err("GITHUB_REPO not allowed".into());
            }
        }
        Ok(())
    }

    pub fn check_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/git/ref/{}",
            &self.git_repo,
            &self.git_ref.replacen("refs/", "", 1)
        );

        let resp: GithubReference = Client::builder()
            .build()?
            .get(&url)
            .bearer_auth(&self.github_token)
            .header(header::USER_AGENT, "github-ssh-ci-runner/1.0")
            .header(header::CONTENT_TYPE, "application/json")
            .timeout(Duration::from_secs(60))
            .send()?
            .json::<GithubReference>()?;

        dbg!(&resp);

        // double check the sha
        if !resp.object.sha.eq(&self.git_sha) {
            return Err("GITHUB_SHA does not match".into());
        }

        Ok(())
    }

    pub fn run_ci(&self) -> Result<Void, Box<dyn std::error::Error>> {
        let exe = CString::new(self.config.runner[0].clone())?;
        let mut c_args = Vec::<CString>::new();
        for arg in &self.config.runner {
            c_args.push(CString::new(arg.clone())?);
        }
        let cstr_args: Vec<&CStr> = c_args.iter().map(|v| v.as_c_str()).collect();
        nix::unistd::execve(
            &exe,
            cstr_args.as_slice(),
            &[
                &CString::new(format!("GITHUB_TOKEN={}", &self.github_token))?,
                &CString::new(format!("GITHUB_REPOSITORY={}", &self.git_repo))?,
                &CString::new(format!("GITHUB_SHA={}", &self.git_sha))?,
                &CString::new(format!("GITHUB_REF={}", &self.git_ref))?,
            ],
        )
        .map_err(|e| e.into())
    }
}
