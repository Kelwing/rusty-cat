use std::str;
use std::collections::HashMap;
use std::io::File;
use std::os;

#[allow(unstable)]
fn main() {
    let mut options = HashMap::new(); 
    let args = os::args();
    let mut files = Vec::new();
    for x in range(1us, args.len()) {
        let s_slice: &str = args[x].as_slice();
        if !s_slice.starts_with("-") {
            files.push(args[x].clone());
        } else {
            let chars_to_trim: &[char] = &['-'];
            options.insert(s_slice.trim_matches(chars_to_trim), true);
        }
    }
    for x in files.iter() {
        let mut file = match File::open(&Path::new(x.as_bytes())) {
            Err(why) => panic!("{}", why),
            Ok(file) => file,
        };
        let file_data = match file.read_to_end() {
            Ok(data) => data,
            Err(why) => panic!("{}", why),
        };
        let readable_str = match str::from_utf8(file_data.as_slice()) {
            Err(why) => panic!("{}", why),
            Ok(c) => c,
        };
        for character in readable_str.chars() {
            print!("{}",match character {
                '\n' => match options.contains_key(&("E")){
                            true => "$\n".to_string(),
                            false => "\n".to_string(),
                        },   
                _ => character.to_string(),
            });
        }
    } 
}
