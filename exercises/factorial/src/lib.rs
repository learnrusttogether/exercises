pub fn factorial(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    n * factorial(n - 1)
}
