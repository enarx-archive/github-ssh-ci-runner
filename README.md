# github-ssh-ci-runner

Run a hosted github actions runner.

Create a new ssh-key. The public and private part are both exposed.

Create a `cirunner` user.

`~cirunner/.ssh/authorized_keys` with `<ssh-pub-key>` replaced:
```
command="$HOME/bin/github-ssh-ci-runner $HOME/ci-config.toml",no-agent-forwarding,no-port-forwarding,no-pty,no-user-rc,no-X11-forwarding <ssh-pub-key>
```

Copy the release binary of `github-ssh-ci-runner` in `~cirunner/bin/`

`~cirunner/ci-config.toml`: 
```
runner = [ "/path/to/real/cirunner", "arg1", "arg2" ]
repos = [ "userfoo/repobar", "userfoo/repobaz" ]
```

Ideally the real `cirunner` is guarded with `/bin/timeout` and runs a rootless `podman` test container.

The test container can then checkout the git repo via the environment variables:
- `GITHUB_REPOSITORY`
- `GITHUB_SHA`
- `GITHUB_REF`

Also you get a `GITHUB_TOKEN`

On the github actions side you have to run (with `<YOUR_CI_HOSTNAME>` replaced):
```yaml
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
        exec ssh -T -o "StrictHostKeyChecking no" -i .ssh/id_github_test <YOUR_CI_HOSTNAME> -- "$GITHUB_TOKEN" "$GITHUB_REPOSITORY" "$GITHUB_SHA" "$GITHUB_REF"
```

check in the private ssh key in your repo under `.ssh/id_github_test`.

For extra security on the CI host side, configure systemd logind with 
```properties
KillUserProcesses=true
KillOnlyUsers=cirunner
```
