use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use rand::Rng;

fn get_line_at(path: &Path, line_num: usize) -> Result<String, Error> {
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("No line found at that position")
}

// todo - Create check_dir and overwrite that allow users to input all the text in the file and then they will get it layer
// fn check_dir(path: &Path) {

// }

// fn overwrite_all(path: &Path, overwrite_text: String) {

// }

pub fn generate_line() -> Result<String, Error> {
    let dir_path = "file.txt";

    let path = Path::new(dir_path); // creating the path
    let line_num = 10usize; // caching number of lines manually
      
    // allocating thread memory and getting the random number
    let mut rng = rand::thread_rng();
      
    let random_code: usize = rng.gen_range(0..line_num); // get a random code from 0 to last_code_index which is the 100 code
    println!("the random number was ? :  {:?}", random_code);
      
    // finally getting the number and printing it to the console
    let line = get_line_at(&path, random_code);
    return line;
}