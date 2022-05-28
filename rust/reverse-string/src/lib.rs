extern crate unicode-segmentation;
use unicode-segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let drow: String = word
        .graphemes(true)
        .rev()
        .collect();
    
    drow
    
    println!("{}", drow);


}
