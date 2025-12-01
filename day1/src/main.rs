use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("input.txt")?;
    let lines = BufReader::new(input).lines();

    let mut cursor = 50;
    let mut password = 0;

    for line in lines.map_while(Result::ok) {
        println!("{line}");
        let sign = match &line[0..1] {
            "L" => -1,
            "R" => 1,
            c => panic!("unexpected character {c}"),
        };

        let distance: i32 = line[1..].parse()?;
        let delta = (sign * distance + 100) % 100;

        cursor = (cursor + delta) % 100;

        if cursor == 0 {
            password += 1;
        }
    }

    println!("Password: {password}");

    Ok(())
}
