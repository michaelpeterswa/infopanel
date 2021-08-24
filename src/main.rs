fn main() {

    let word: &str = "testing";
    square_text_center(word);

}

// rectangle around supplied text
fn square_text_center(var: &str){

    let tlc: char = '\u{250C}';     // Unicode top-left-corner char
    let blc: char = '\u{2514}';     // Unicode botttom-left-corner char
    let trc: char = '\u{2510}';     // Unicode top-right-corner char
    let brc: char = '\u{2518}';     // Unicode bottom-right-corner char
    let horiz: char = '\u{2500}';   // Unicode horizontal char
    let vert: char = '\u{2502}';    // Unicode vertical char
    let spc: char = ' ';

    let mut first_row: String = "".to_string();
    let mut middle_row: String = "".to_string();
    let mut last_row: String = "".to_string();

    let varlen: usize = var.len();
    let mut cnt: usize = 0;

    first_row = add_chars(first_row, tlc, horiz);
    middle_row = add_chars(middle_row, vert, spc);
    last_row = add_chars(last_row, blc, horiz);

    middle_row.push_str(var);

    while cnt < varlen {
        first_row.push(horiz);
        last_row.push(horiz);
        cnt = cnt + 1
    }

    first_row = add_chars(first_row, horiz, trc);
    middle_row = add_chars(middle_row, spc, vert);
    last_row = add_chars(last_row, horiz, brc);

    println!("{}", first_row);
    println!("{}", middle_row);
    println!("{}", last_row);

}

fn add_chars(mut row: String, chr1: char, chr2: char) -> String {
    row.push(chr1);
    row.push(chr2);

    return row;
}

