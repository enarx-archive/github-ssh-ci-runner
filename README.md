# github-ssh-ci-runner

Run a custom command (like your specialized CI) on a remote server via github actions on push/pull_requests.

## Setup

### Remote Server

- Create a new ssh-key (referenced later with `ssh-pub-key` and `ssh-priv-key`). \
  The public and private part are **both exposed**.
- Create a `cirunner` user. \
  For extra security configure systemd logind with 
  ```properties
  KillUserProcesses=true
  KillOnlyUsers=cirunner
  ```
- create `~cirunner/.ssh/authorized_keys` with `<ssh-pub-key>` replaced:
  ```
  command="$HOME/bin/github-ssh-ci-runner $HOME/ci-config.toml",no-agent-forwarding,no-port-forwarding,no-pty,no-user-rc,no-X11-forwarding <ssh-pub-key>
  ```
- copy the release binary of `github-ssh-ci-runner` in `~cirunner/bin/`
- create `~cirunner/ci-config.toml`: 
  ```
  runner = [ "/path/to/your_ci_script", "arg1", "arg2" ]
  repos = [ "userfoo/repobar", "userfoo/repobaz" ]
  ```
  `runner` is the path and arguments to your custom CI script. \
  `repos` specifies the allowed github repositories, which are allowed to trigger the ci. \
  \
  Ideally the real `your_ci_script` runs a rootless `podman` test container, which is removed after the run. \
  We opened an [issue](https://github.com/containers/libpod/issues/6412) against podman to request a timeout option for `podman run`. \
  \
  The test container can then checkout the git repo via the environment variables:
  - `GITHUB_REPOSITORY`
  - `GITHUB_SHA`
  - `GITHUB_REF`
  - `GITHUB_TOKEN`
  ```
  git clone "$GITHUB_REPOSITORY" testrepo
  cd testrepo
  git fetch git fetch origin "$GITHUB_REF"
  git checkout -b testbranch FETCH_HEAD
  # run your CI with the repo
  […]
  ```
### Github Actions

- In the repo to test check in the private ssh key `ssh-priv-key` under `.ssh/id_github_remote_ci`.
- Create an action (with `<YOUR_CI_HOSTNAME>` replaced):

```yaml
name: remote-ci

on:
  push:
  pull_request:

jobs:
  build:
    runs-on: ubuntu-latest
    timeout-minutes: 60
    steps:
    - uses: actions/checkout@v2
    - name: github-ssh-ci-runner
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      run: |
        chmod 0700 .ssh
        chmod 0600 .ssh/id_github_test
        exec ssh -T -o "StrictHostKeyChecking no" -i .ssh/id_github_remote_ci <YOUR_CI_HOSTNAME> -- "$GITHUB_TOKEN" "$GITHUB_REPOSITORY" "$GITHUB_SHA" "$GITHUB_REF"
```

Watch your remote CI get executed on `push` and `pull_request`.
