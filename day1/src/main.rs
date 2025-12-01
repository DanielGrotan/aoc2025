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
        let mut chars = line.chars();

        let direction = chars.next().ok_or("empty line")?;
        let sign = match direction {
            'L' => -1,
            'R' => 1,
            c => return Err(format!("unexpected direction '{c}'").into()),
        };

        let mut distance: i32 = chars.as_str().parse()?;

        let wraps = distance / 100;
        password += wraps;
        distance %= 100;

        if sign == -1 && cursor != 0 && distance >= cursor {
            password += 1;
        } else if sign == 1 && distance >= 100 - cursor {
            password += 1;
        }

        let delta = (sign * distance + 100) % 100;
        cursor = (cursor + delta) % 100;
    }

    println!("Password: {password}");

    Ok(())
}
