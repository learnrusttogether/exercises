# Leap

[<img src="https://img.shields.io/badge/difficulty-easy-green">](https://t.me/learnrusttogether)

Given a year, report if it is a leap year.

The tricky thing here is that a leap year in the Gregorian calendar occurs:

```text
on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
```

For example, 1997 is not a leap year, but 1996 is. 1900 is not a leap
year, but 2000 is.

## Notes

Minute explanation of the whole leap year
phenomenon: [this youtube video][video].

[video]: http://www.youtube.com/watch?v=xX96xng7sAE

You may use the [`arithmetic remainder` operator](https://doc.rust-lang.org/book/appendix-02-operators.html) to test for divisibility.

## Code the Test

Execute the tests with:

```bash
$ make test # from root dir
```

Open the tests source file which is located in the `exercisestests/<exercise>` directory
and remove the `#[ignore]` flag from the necessary test and get the tests to pass.

After you get the first test to pass, open the tests source file which is located in the `tests` directory
and remove the `#[ignore]` flag from the next test and get the tests to pass
again. Each separate test is a function with `#[test]` flag above it.
Continue, until you pass every test.

To run a specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test # from ./exercises dir
```

If the specific test is ignored use:

```bash
$ cargo test some_test -- --ignored # from ./exercises dir
```

## Submitting the solution

Create new Pull Request to [exercises](https://github.com/learnrusttogether/exercises) repo and get feedback from community members.

## Source

JavaRanch Cattle Drive, exercise 3 [http://www.javaranch.com/leap.jsp](http://www.javaranch.com/leap.jsp)

## Submitting Incomplete Solutions

It's possible to submit an incomplete solution so you can get feedback to complete the exercise.
