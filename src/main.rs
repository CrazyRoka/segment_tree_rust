use std::io::{self, stdin};

use segment_tree::MaxSliceSumSegmentTree;

fn main() -> io::Result<()> {
    let stdin = stdin();

    let n = read_usize(&stdin)?;
    let sequence = read_sequence(&stdin)?;

    assert_eq!(n, sequence.len());

    let mut tree = MaxSliceSumSegmentTree::build(&sequence);

    let m = read_usize(&stdin)?;
    for _ in 0..m {
        let query = read_sequence(&stdin)?;
        assert_eq!(3, query.len());

        match query[0] {
            0 => {
                let pos = query[1] as usize - 1;
                let value = query[2];
                tree.modify(pos, &value)
                    .expect("Modify operation should always be successful");
            }
            1 => {
                let left = query[1] as usize - 1;
                let right = query[2] as usize - 1;
                let result = tree
                    .get(left, right)
                    .expect("Print operation should always be successful");
                println!("{}", result.answer());
            }
            _ => {
                panic!("Unexpected query type {}", query[0])
            }
        }
    }

    Ok(())
}

fn read_sequence(stdin: &io::Stdin) -> io::Result<Vec<i64>> {
    let a: Vec<i64> = read_line(&stdin)?
        .split(" ")
        .into_iter()
        .map(|x| x.parse().expect("Sequence should consist of numbers"))
        .collect();
    Ok(a)
}

fn read_usize(stdin: &io::Stdin) -> io::Result<usize> {
    let num = read_line(&stdin)?.parse().expect("n should be given");
    Ok(num)
}

fn read_line(stdin: &io::Stdin) -> io::Result<String> {
    let mut input = String::new();
    stdin.read_line(&mut input)?;

    Ok(input.trim().to_string())
}
