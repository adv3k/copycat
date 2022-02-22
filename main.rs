use std::cmp::Ordering;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use std::path::Path;


#[derive(Debug)]
struct Query(String);


impl Query {
    //new() creates an instance of Comms with the commands given in cmd line
    fn new(args: &[String]) -> Result<Query, &str> {
        let filename = match Path::new(&args[1]).exists() {
            true => args[1].clone(),
            false => panic!("not a valid path"),
        };
        println!("{:?}", filename);
        match args.len().cmp(&2) {
            Ordering::Equal => Ok(Query(filename)),
            _ => panic!("incorrect number of arguments"),
        }

    }
}


//prints out each line in a specified file
fn read_file(filename: String) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let commands = Query::new(&args).unwrap();


    println!("{:?}", read_file(commands.0));
    //match commands.query to function, and more to add later
}
