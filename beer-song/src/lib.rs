fn bottles(n: u32) -> String {
    match n {
        0 => String::from("no more bottles"),
        1 => format!("{} bottle", n),
        _ => format!("{} bottles", n),
    }
}

pub fn verse(n: u32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        _ => {
            let it = if n > 1 { "one" } else { "it" };
        format!("{bottles} of beer on the wall, {bottles} of beer.\nTake {it} down and pass it around, {bottles_l} of beer on the wall.\n", bottles=bottles(n), bottles_l=bottles(n-1), it=it)
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
