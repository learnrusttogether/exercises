// use fibonacci_sequence::*;

// #[test]
// fn test_fibonacci_sequence() {
//     let a = vec![
//         [0, 0],
//         [1, 1],
//         [2, 1],
//         [3, 2],
//         [4, 3],
//         [5, 5],
//         [6, 8],
//         [7, 13],
//         [8, 21],
//         [9, 34],
//         [10, 55],
//         [11, 89],
//         [12, 144],
//         [13, 233],
//         [14, 377],
//         [15, 610],
//         [16, 987],
//         [17, 1597],
//         [18, 2584],
//         [19, 4181],
//         [20, 6765],
//         [21, 10946],
//         [22, 17711],
//         [23, 28657],
//         [24, 46368],
//         [25, 75025],
//         [26, 121393],
//         [27, 196418],
//         [28, 317811],
//         [29, 514229],
//         [30, 832040],
//         [31, 1346269],
//         [32, 2178309],
//         [33, 3524578],
//         [34, 5702887],
//         [35, 9227465],
//         [36, 14930352],
//         [37, 24157817],
//         [38, 39088169],
//         [39, 63245986],
//         [40, 102334155],
//     ];

//     for i in a.iter() {
//         let result: u32 = match fibonacci(i[0]) {
//             Some(number) => number,
//             _ => {
//                 println!("Invalid Number!");
//                 0
//             }
//         };
//         assert_eq!(i[1], result);
//     }
// }
