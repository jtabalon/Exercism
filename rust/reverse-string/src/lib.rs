extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let drow: String = input
        .graphemes(true)
        .rev()
        .collect();
    
    drow
    
    // println!("{}", drow);


}
