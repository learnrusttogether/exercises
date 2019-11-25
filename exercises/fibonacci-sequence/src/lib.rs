pub fn fibonacci(n: i32) -> Option<i32> {
    let mut a: i32 = 1;
    let mut b: i32 = 1;
    let mut c: i32;

    if n == 0 {
        return Some(0);
    } else if n == 1 {
        return Some(1);
    } else if n > 1 {
        for _i in 2..n {
            c = a + b;
            a = b;
            b = c;
        }
        return Some(b);
    }

    None
}
