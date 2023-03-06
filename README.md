# Linear Templater

[![Build Status](https://github.com/alanvardy/linear_templater/workflows/ci/badge.svg)](https://github.com/alanvardy/linear_templater) [![codecov](https://codecov.io/gh/alanvardy/linear_templater/branch/main/graph/badge.svg?token=9FBJK1SU0K)](https://codecov.io/gh/alanvardy/linear_templater) [![Crates.io](https://img.shields.io/crates/v/linear_templater.svg)](https://crates.io/crates/linear_templater)

Create Linear Tickets from TOML files, it currently supports creating 1 parent ticket and unlimited child tickets per TOML. Supports handlebars syntax.

## Install from Crates.io

[Install Rust](https://www.rust-lang.org/tools/install)

```bash
# Linux and MacOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install Linear Templater

```bash
cargo install linear_templater
```

## Usage

You need to have the environment variable `LINEAR_TOKEN` set. I am currently implementing this way so that a `.envrc` file can be used per TOML directory and a person can seamlessly transition between multiple Linear workspaces. You can obtain your linear token from the API section of your account settings.

Start with the help flag to get the latest commands

```bash
> linear_templater -h

Create Linear Tickets from TOML files

Usage: linear_templater [OPTIONS]

Options:
  -f, --fetch_ids <JSON FILE OUTPUT PATH>
          Fetch ids for player and teams, and output to provided path as a JSON file
  -c, --create_issues <PATH TO TOML FILE>
          Read a TOML file and create a issues from it
  -h, --help
          Print help
  -V, --version
          Print version
```

### Examples

#### Fetch all the IDs needed to fill out some TOML files

Command

```bash
LINEAR_TOKEN=xxxx linear_templater -f ~/Documents/output.json
```

Result

```json
{
  "data": {
    "viewer": {
      "id": "xxxxxx",
      "name": "Batman",
      "teamMemberships": {
        "nodes": [
          {
            "team": {
              "id": "yyyyyy",
              "name": "Justice League",
              "projects": {
                "nodes": [
                  {
                    "id": "zzzzzz",
                    "name": "Upgrade infrastructure"
                  }
                ]
              }
            }
          }
        ]
      }
    }
  }
}
```

#### Create a series of tickets from a TOML file

Input file (uses [handlebars-style variables](https://handlebarsjs.com/))

```toml
# build_batcave.toml
[variables]
name = "Alfred"

[parent]
title = "This is a parent issue"
team_id = "yyyyyy"
# optional
assignee_id = "xxxxxx"
project_id = "zzzzzz"
description = """
We need to create a batcave

See child tickets
"""

[[children]]
title = "This is a child issue for {{name}} to complete"
# optional
team_id = "yyyyyy"
assignee_id = "xxxxxx"
description = """
Figure out where to put the batcave

 - Some place dark and dingy
 - Make sure to coordinate with {{name}}
"""

[[children]]
title = "This is a second child issue that will be linked to the parent issue"
# optional
team_id = "yyyyyy"
assignee_id = "xxxxxx"
description = """
Make sure that we have enough bats

### Acceptance Criteria

- [ ] They can't bite too much
- [ ] At least a dozen
- [ ] Don't overdo it this time
"""

```

Command

```bash
LINEAR_TOKEN=xxxx linear_templater -c ~/Documents/build_batcave.toml
```

#### Create a series of tickets from all TOML files in a directory

When passed a directory, Linear Templater will recursively walk through the directory and all sub-directories and create tickets from all the TOML files that are not `Cargo.toml`.

Command

```bash
# Create tickets from all TOML files in the current directory
LINEAR_TOKEN=xxxx linear_templater -c .
```

#### Use direnv to manage your LINEAR_TOKEN

Install [direnv](https://github.com/direnv/direnv) using your package manager of choice

```bash
echo export LINEAR_TOKEN=xxxx > .envrc
```

This removes the need to prefix all your commands with LINEAR_TOKEN=xxxx
