// SPDX-License-Identifier: Apache-2.0

use crate::{CiRunner, Config};

#[test]
fn sanity_check_token() {
    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_ok());

    assert!(CiRunner {
        github_token: "v2.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());

    assert!(CiRunner {
        github_token: "v1.he25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea811".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());
}

#[test]
fn sanity_check_repo() {
    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_ok());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/$(false)".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());
}

#[test]
fn sanity_check_sha() {
    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "abcdef1234567890abcdef1234567890abcdef12".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_ok());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "gbcdef1234567890abcdef1234567890abcdef12".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "abcdef1234567890abcdef1234567890abcdef123".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "abcdef123456789".to_string(),
        git_ref: "refs/heads/test".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());
}

#[test]
fn sanity_check_refs_pr() {
    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/pull/123/merge".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_ok());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/987654321😜😜😜😜".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_ok());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/987654321 😜😜😜😜".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/987654321/😜😜😜😜".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_ok());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/$(cat/etc/passwd)".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/(cat/etc/passwd)".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/heads/`cat/etc/passwd`".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());

    assert!(CiRunner {
        github_token: "v1.8e25f0724a2f1ba79ca74fdf69abf71140d5ea81".to_string(),
        git_repo: "test/test".to_string(),
        git_sha: "662623e66b20784bb7abf05feab4c30f309f679d".to_string(),
        git_ref: "refs/pull/123/foo".to_string(),
        config: Config {
            runner: vec![],
            repos: None,
        },
    }
    .sanity_check_params()
    .is_err());
}
