pub fn fibonacci(n: u32) -> Option<u32> {
    let mut a: u32 = 1;
    let mut b: u32 = 1;
    let mut c: u32;

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
