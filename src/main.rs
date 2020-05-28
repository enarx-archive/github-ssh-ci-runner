// SPDX-License-Identifier: Apache-2.0

use github_ssh_ci_runner::CiRunner;

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
