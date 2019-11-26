pub fn fibonacci(n: u32) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 1;
    let mut c: u32;

    match n {
        0 => 0,
        1 => 1,
        _ => {
            for _ in 2..n {
                c = a + b;
                a = b;
                b = c;
            }

            b
        }
    }
}
