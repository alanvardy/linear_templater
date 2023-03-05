# Linear Templater

Create Linear Tickets from TOML files, it currently supports creating 1 parent ticket and unlimited child tickets per TOML.

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

You need to have the environment variable `LINEAR_TOKEN` set. I am currently implementing this way so that a `.env` file can be used per TOML directory and a person can seamlessly transition between multiple Linear workspaces. You can obtain your linear token from the API section of your account settings.

Start with the help flag to get the latest commands

```bash
> linear_templater -h

Create Linear Tickets from TOML files

Usage: linear_templater [OPTIONS]

Options:
  -f, --fetch_ids <JSON FILE OUTPUT PATH>
          Fetch ids for player and teams, and output to provided path as a JSON file
  -c, --create_tickets <PATH TO TOML FILE>
          Read a TOML file and create tickets from it
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

Result:

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
              "name": "Justice League"
            }
          }
        ]
      }
    }
  }
}
```

#### Create a series of tickets from TOML files

Input file

```toml
title = "This is a parent issue"
team_id = "yyyyyy"
assignee_id = "xxxxxx"
description = """
We need to create a batcave
"""

[[children]]
title = "This is a child issue that will be linked to the parent issue"
team_id = "yyyyyy"
assignee_id = "xxxxxx"
description = """
Figure out where to put the batcave
"""

[[children]]
title = "This is a second child issue that will be linked to the parent issue"
team_id = "yyyyyy"
assignee_id = "xxxxxx"
description = """
Make sure that we have enough bats
"""

```

Command

```bash
LINEAR_TOKEN=xxxx linear_templater -c ~/Documents/build_batcave.toml
```
