use factorial;

#[test]
fn test_factorial() {
    let a: [[i32; 2]; 11] = [
        [0, 1],
        [1, 1],
        [2, 2],
        [3, 6],
        [4, 24],
        [5, 120],
        [6, 720],
        [7, 5040],
        [8, 40320],
        [9, 362880],
        [10, 3628800],
    ];

    for i in &a {
        assert_eq!(i[1], factorial::factorial(i[0]));
    }
}
