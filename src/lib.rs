use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineBatch {
    pub lines: Vec<String>,
}

#[derive(Debug)]
pub struct LinesReader {
    pub reader: BufReader<File>,
    pub batch_size: usize,
}

impl LinesReader {
    pub fn new(file: File, batch_size: usize) -> LinesReader {
        LinesReader {
            reader: BufReader::new(file),
            batch_size,
        }
    }
}

impl Iterator for LinesReader {
    type Item = LineBatch;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lines = Vec::with_capacity(self.batch_size);

        for _ in 0..self.batch_size {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => lines.push(line.replace('\n', "")),
                Err(e) => panic!("Error reading line: {}", e),
            }
        }

        if lines.is_empty() {
            None
        } else {
            Some(LineBatch { lines })
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

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
}
