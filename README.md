# Learn Rust Together

[![Build Status](https://travis-ci.org/learnrusttogether/exercises.svg?branch=master)](https://travis-ci.org/learnrusttogether/exercises)
[<img src="https://img.shields.io/badge/-Telegram-blue?logo=telegram">](https://t.me/learnrusttogether)

Exercises in Rust

## Getting started

### Fork a Repo
[Fork Tutorial](https://help.github.com/en/github/getting-started-with-github/fork-a-repo)

### Clone Repo

```bash
$ git clone https://github.com/learnrusttogether/exercises.git
```

### Go to Repo

```bash
$ cd exercises
```

### Uncomment Necessary Tests 

Tests path: `./exercises/tests`

```rust
use hello_world::*;

#[test]
fn test_hello_world() {
    assert_eq!("Hello, World!", hello());
}
```

### Complete the Exercise 

```rust
pub fn hello() -> &'static str {
    "Goodbye, World!"
}

->

pub fn hello() -> &'static str {
    "Hello, World!"
}
```

### Run Tests from Repo Root
Your current path: `~/exersices/`

```bash
$ make test
```

## Push Changes
Your current path: `~/exersices/`

```bash
$ git add .

$ git commit -m "hello-world"

$ git push
```

## Create New Pull Request
[Pull Request Tutorial](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request)

## Getting Updates

```bash
$ make update
```

If you have an alert `Please enter a commit message to explain why this merge is necessary, especially if it merges an updated upstream into a topic branch.` read [this](https://stackoverflow.com/questions/19085807/please-enter-a-commit-message-to-explain-why-this-merge-is-necessary-especially) tutorial.
