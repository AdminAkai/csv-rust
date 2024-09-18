use std::io;

fn main() {
    // CSV parser that reads data from stdin
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // Loop over each record
    for result in rdr.records() {
        let record = result.expect("a CSV record");
        // print debug version of the record
        println!("{:?}", record)
    }
}
