pub fn verse(num: usize) -> String {
    match num {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_owned(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_owned(),
        _ => format!("{n1} bottles of beer on the wall, {n1} bottles of beer.\nTake one down and pass it around, {n0} bottles of beer on the wall.\n", n1 = num, n0 = num - 1)
    }
}

pub fn sing(start: usize, end: usize) -> String {
    let verses = (end..start+1).
        rev().
        map(|n| verse(n)).collect::<Vec<String>>();

    verses.join("\n")
}
