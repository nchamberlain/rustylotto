// Reads a superlotto csv file from https://www.lotteryusa.com/california/super-lotto-plus/year
// and writes out a new csv file with the date in a different format and the numbers comma separated
// instead of being embedded in a single dbl-quoted string.
use chrono::NaiveDate;
use std::fs::File;
use std::io::{self, BufRead, LineWriter, Write};
use std::path::Path;
use log::{error, warn, info, debug, trace, log_enabled, Level};
use dotenvy::dotenv;

// Returns an iterator over the lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    // initialize the environment If .env isn't found then it will default to debug level
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    if log_enabled!(Level::Debug) {
        for (key, value) in dotenvy::vars() { // prints out EVERY evn variable
            println!("{}: {}", key, value);   // not just the ones in the .env file
        }
    }   

    let file = File::create("slnumbers1out.csv")?;
    let mut line_writer = LineWriter::new(file);
    if let Ok(lines) = read_lines("slnumbers1.csv") {
        for line in lines.flatten() {
            if line.starts_with("\"Wed") {
                let w_slice: &str = &line[1..24]; //skip the dbl-quotes so it is recognized as a valid date

                // reformat the full date string into a shorter format
                let ndin =
                    NaiveDate::parse_from_str(w_slice, "%A, %b %d, %Y").expect("Invalid Date");
                let ndout: String = format!("{}", ndin.format("%a %m/%d/%y").to_string());

                // get the rest of the numbers being sure to remove the one dbl-quote embedded in slice
                let nbr_slice: &str = &line[27..].replace("\"", "");

                debug!("Date: {} Numbers: {}", ndout, nbr_slice);

                // insert dbl-quotes around the date and add the numbers without dbl-quotes
                let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                writeln!(line_writer, "{}", out_line).expect("Error writing Wednesday line");
            } else if line.starts_with("\"Sat") {
                let s_slice: &str = &line[1..23]; //skip the dbl-quotes so it is recognized as a valid date

                // reformat the full date string into a shorter format
                let ndin =
                    NaiveDate::parse_from_str(s_slice, "%A, %b %d, %Y").expect("Invalid Date");
                let ndout: String = format!("{}", ndin.format("%a %m/%d/%y").to_string());

                // get the rest of the numbers being sure to remove the one dbl-quote embedded in slice
                let nbr_slice: &str = &line[26..].replace("\"", "");

                debug!("Date= {} Numbers= {}", ndout, nbr_slice);
                
                // insert dbl-quotes around the date and add the numbers without dbl-quotes
                let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                writeln!(line_writer, "{}", out_line).expect("Error writing Saturday line");
            } else if line.starts_with("Date") { // processes only 1st line with column headers
                debug!("Date,C1,C2,C3,C4,C5,Mega");
                let out_line = "Date,C1,C2,C3,C4,C5,Mega".to_string(); //new column headers
                writeln!(line_writer, "{}", out_line).expect("Error writing column headers");
            } else {
                eprintln!("Line NOT Recognized"); //should never happen
            }
        }
    }
    if log_enabled!(Level::Trace){ 
        trace!("Trace level is active (most detailed)");
        debug!("debug level is active (lots of details)");
        info!("info is active (std setting)");
        warn!("warning level is set (typical for production)");
        error!("only errors reported (for mature production onlt)");
    }
    Ok(())
}
