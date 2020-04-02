use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

pub fn byte_count(p:&Path)->u64{
    return std::fs::metadata(&p).unwrap().len();
}

pub fn print_output(c:bool,w:bool,l:bool,file:&str) {
    if c {
        let byte_count = byte_count(file.as_ref());
        print!(" {} ", byte_count);
    }
    if w || l {
        let word_line = word_line_count(file.as_ref());
        if w {
            print!(" {} ", word_line.0);
        }
        if l {
            print!(" {} ", word_line.1);
        }
    }


    println!(" {} ", file);
}
pub fn word_line_count(p:&Path) -> (usize,usize){
    let mut word_count:usize=0;
    let mut line_count:usize=0;

    let f= File::open(&p).unwrap();
    let reader=BufReader::new(f);
    for line in reader.lines(){
        let mut first= false;
        let mut last= false;
        line_count=line_count+1;
        for c in line.unwrap().chars(){
            if !first{
                first=true;
                if c.is_whitespace() {
                    last=true;

                }
                else{
                    last=false;
                    word_count=word_count+1;
                }
                continue;
            }
            else {
                if c.is_whitespace(){
                    if last { continue;}
                    else {last=true;}
                }
                else{
                    if last {
                        word_count=word_count+1;
                        last=false;

                    }
                    else { continue; }
                }
            }
        }
    }


    (word_count,line_count
    )

}