# Learn Rust Together

[![Build Status](https://travis-ci.org/learnrusttogether/exercises.svg?branch=master)](https://travis-ci.org/learnrusttogether/exercises)
[<img src="https://img.shields.io/badge/-Telegram-blue?logo=telegram">](https://t.me/learnrusttogether)

The project helps to share your Rust code for review via pull requests. Complete exercises and get feedback from the community.

## Background Requirements

- Complete [The Rust Programming Language for Beginners](https://www.udemy.com/course/the-rust-programming-language-for-beginners/) course or read [The Rust Programming Language](https://doc.rust-lang.org/book/).
- Enthusiasm and a willingness to take your Rust Programming skills to the next level.

## Available Exercises

- [factorial](https://github.com/learnrusttogether/exercises/tree/master/exercises/factorial)
- [fibonacci-sequence](https://github.com/learnrusttogether/exercises/tree/master/exercises/fibonacci-sequence)
- [hello-world](https://github.com/learnrusttogether/exercises/tree/master/exercises/hello-world)

## Getting Started

### Fork a Repo

[Fork Tutorial](https://help.github.com/en/github/getting-started-with-github/fork-a-repo)

### Clone a Repo

```bash
$ git clone https://github.com/learnrusttogether/exercises.git
```

### Go to Repo

```bash
$ cd exercises
```

### Unlock Necessary Tests

Open the tests source file which is located in the `./exercises/tests` directory
and remove the `#[ignore]` flag from the necessary test and get the tests to pass.

```rust
use hello_world::*;

#[test]
#[ignore] // <- remove
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

### Push Changes

```bash
$ git add .

$ git commit -m "hello-world"

$ git push
```

### Create New Pull Request

[Pull Request Tutorial](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request)

## Getting Updates

```bash
$ make update
```

If you have an alert `Please enter a commit message to explain why this merge is necessary, especially if it merges an updated upstream into a topic branch.` read [this](https://stackoverflow.com/questions/19085807/please-enter-a-commit-message-to-explain-why-this-merge-is-necessary-especially) tutorial.

## Contributing a New Exercise

[Soon...]

## Community

You can ask a question in our [Telegram](https://t.me/learnrusttogether) chat or the issues section.
