<!-- markdownlint-configure-file {
  "MD013": {
    "code_blocks": false,
    "tables": false
  },
  "MD033": false,
  "MD041": false
} -->

<div align="center">

# rcd

rcd is a **smarter cd command**, inspired by z and zoxide.

> :warning: **Disclaimer**: This tool is not intended to replace z, zoxide, autojump or any other cd-type cli tool! It was just a fun way to start with rust and play a little a round.

[Getting started](#getting-started) •
[Installation](#installation)

</div>

## Getting started

```sh
rcd foo              # cd into highest ranked directory matching foo
```

## Installation

### *Step 1: Install rcd*
Right now, rcd is only installable via crate.io or homebrew

<details>
<summary>MacOS</summary>

To install rcd, run these commands in your terminal:

```sh
brew tap konrad-amtenbrink/tap
brew install rcd
```
</details>


### *Step 2: Add rcd to you shell*

<details>
<summary>MacOS</summary>

To add rcd to you shell, run these commands in your terminal:

```sh
echo 'function rcd() {cd $(/usr/local/Cellar/rcd/0.1.0/bin/ripcd $1)}' >> ~/.zshrc
echo  'export -f rcd' >> ~/.zshrc
```
</details>


