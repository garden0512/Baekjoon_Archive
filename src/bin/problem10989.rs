use std::io::{self, BufRead, BufWriter, Write};

fn main()
{
    let mut user_input = String::new();
    let mut reader = io::stdin().lock();
    reader
        .read_line(&mut user_input)
        .expect("Error");
    let line_amount:usize = user_input.trim().parse().expect("Error");
    let mut array = [0;10001];
    for _ in 0..line_amount
    {
        let mut input = String::new();
        reader
            .read_line(&mut input)
            .expect("Error");
        let num:usize = input.trim().parse().expect("Error");
        array[num] += 1;
    }
    let mut writer = BufWriter::new(io::stdout().lock());
    for i in 1..=10000
    {
        if array[i] > 0
        {
            for _ in 0..array[i]
            {
                writeln!(writer,"{}", i).expect("Error");
            }
        }
    }
}