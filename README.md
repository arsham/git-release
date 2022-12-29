# Git Release

[![Continues Integration](https://github.com/arsham/git-release/actions/workflows/integration.yml/badge.svg)](https://github.com/arsham/git-release/actions/workflows/integration.yml)
![License](https://img.shields.io/github/license/arsham/git-release)

This program can set the release information based on all commits of a tag. To
see the example visit [Releases](https://github.com/arsham/git-release/releases)
page.

This is a clone of the [gitrelease] project written in Rust.

1. [Requirements](#requirements)
2. [Installation](#installation)
3. [Usage](#usage)
4. [License](#license)

## Requirements

Uses your github token with permission scope: **repo**

## Installation

To install:

```bash
cargo install git-release
```

Export your github token:
`export GITHUB_TOKEN="ghp_yourgithubtoken"`

Assuming the binary path is in the your `PATH`, `git` automatically picks this
up as a subcommand.

## Usage

After you've made a tag, you can publish the current release documents by just
running:

```bash
git release
```

If you want to release an old tag:

```bash
git release -t v0.1.2
```

If you want to use a different remote other than the `origin`:

```bash
git release -r upstream
```

## License

Licensed under the MIT License. Check the [LICENSE](./LICENSE) file for details.

<!--
vim: foldlevel=1
-->

## References

[gitrelease]: https://github.com/arsham/gitrelease
