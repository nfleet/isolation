extern crate isolation;

use std::io::BufReader;
use isolation::isolation::compute;

#[test]
fn test_sccs() {
    let input = r#"10
3 1 0
1 2 0
1 4 1
4 2 0
3 2 0
7 8 0
8 9 0
10 6 1
10 5 0
5 6 0"#.as_bytes();
    let mut rdr = BufReader::new(input);
    let results: Vec<Vec<_>> = compute(&mut rdr);

    assert_eq!(results[0], vec![3, 1, 4, 2]);
    assert_eq!(results[1], vec![7, 8, 9]);
    assert_eq!(results[2], vec![10, 6, 5]);
}
