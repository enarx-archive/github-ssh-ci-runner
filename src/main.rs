// SPDX-License-Identifier: Apache-2.0

use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::time::Duration;
use void::Void;

#[derive(Debug, Deserialize)]
struct Config {
    runner: Vec<String>,
    repos: Option<Vec<String>>,
}

struct CiRunner {
    github_token: String,
    git_repo: String,
    git_sha: String,
    git_ref: String,
    config: Config,
}

impl CiRunner {
    fn run(config_file: String) -> Result<Void, Box<dyn std::error::Error>> {
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

        // FIXME: maybe use json
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

        ci_runner.check_token()?;

        ci_runner.run_ci()
    }

    fn sanity_check_params(&self) -> Result<(), Box<dyn std::error::Error>> {
        dbg!(&self.git_repo);
        dbg!(&self.git_sha);
        dbg!(&self.git_ref);

        // TODO: check against config.toml
        // Check for valid syntax
        assert!(
            Regex::new(r"^[[:alnum:].]+$")
                .unwrap()
                .is_match(&self.github_token),
            "Invalid GITHUB_TOKEN"
        );
        assert!(
            Regex::new(r"^[^/]+/[^/]+$")
                .unwrap()
                .is_match(&self.git_repo),
            "Invalid GITHUB_REPOSITORY"
        );
        assert!(
            Regex::new(r"^[0-9a-z]+$").unwrap().is_match(&self.git_sha),
            "Invalid GITHUB_SHA"
        );
        assert!(
            Regex::new(r"^refs/pull/[[:digit:]]+/[[:alpha:]]+$")
                .unwrap()
                .is_match(&self.git_ref),
            "Invalid GITHUB_REF"
        );

        if let Some(repos) = self.config.repos.as_ref() {
            assert!(repos.contains(&self.git_repo), "Invalid GITHUB_REPO");
        }

        Ok(())
    }

    fn check_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/actions/runners",
            &self.git_repo
        );

        let resp = Client::builder()
            .build()?
            .get(&url)
            .bearer_auth(&self.github_token)
            .header(header::USER_AGENT, "github-ssh-ci-runner/1.0")
            .header(header::CONTENT_TYPE, "application/json")
            .timeout(Duration::from_secs(60))
            .send()?
            .json::<HashMap<String, String>>()?;

        dbg!(resp);

        Ok(())
    }

    fn run_ci(&self) -> Result<Void, Box<dyn std::error::Error>> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().take(2);

    let prog_name = args.next().unwrap();

    let config_file = args
        .next()
        .expect(&format!("Usage: {} <config.toml>", prog_name));

    CiRunner::run(config_file)?;
    // Either CiRunner::run returned with an error, or never
    Ok(())
}
