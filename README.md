# Simple example reading batches of lines from a text file
Input file content:
```
1
2
3
4
5
a
b
c
d
e
```

Test code:
```rust
#[rstest]
#[case(
    "test_fixtures/batch_5.txt",
    5,
    vec![
        vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string()
        ],
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string()
        ],
    ]
)]
fn test(
    #[case] file_path: &str,
    #[case] batch_size: usize,
    #[case] line_batches: Vec<Vec<String>>,
) {
    let expected = line_batches
        .into_iter()
        .map(|lines| LineBatch { lines })
        .collect::<Vec<_>>();
    let mut reader = LinesReader::new(File::open(file_path).unwrap(), batch_size);

    for lb in expected {
        assert_eq!(reader.next(), Some(lb));
    }
}
```