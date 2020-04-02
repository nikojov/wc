

mod wlc_count;
extern crate clap;
use clap::*;
fn main() {

    let matches=App::new("My version of wc command")
        .version("1.0")
        .about("This is my implementation of wc command")
        .author("Nikola Pesic <nikolapesic17500@gmail.com")
        .arg(Arg::with_name("w")
            .short("w")
            .help("Count words in a file"))
        .arg(Arg::with_name("l")
            .short("l")
            .help("Count lines in a file"))
        .arg(Arg::with_name("c")
            .short("c")
            .help("Count bytes in a file"))
        .arg(Arg::with_name("INPUT")
            .index(1)
            .help("Sets input file[s] to use")
            .required(true)
            .multiple(true)).get_matches();

let mut l_flag=matches.is_present("l");
    let mut c_flag=matches.is_present("c");
    let mut w_flag=matches.is_present("w");
    if l_flag==false && c_flag==false && w_flag==false {
        l_flag=true;
        c_flag=true;
        w_flag=true;
    }
    if let Some(input_files)=matches.values_of("INPUT"){
    for input_file in input_files {
        wlc_count::print_output(c_flag, w_flag, l_flag, input_file);
    }
    }

}
