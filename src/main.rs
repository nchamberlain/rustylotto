// Reads a superlotto csv file from https://www.lotteryusa.com/california/super-lotto-plus/year
// and writes out a new csv file with the date in a different format and the numbers comma separated
// instead of being embedded in a single dbl-quoted string.
use chrono::NaiveDate;
use std::fs::File;
use std::fs;
use std::io::{self, BufRead, BufReader, LineWriter, Write};
use std::path::Path;
use log::{error, warn, info, debug, trace, log_enabled, Level};
use dotenvy::dotenv;
use plotters::prelude::*;
use image::{ImageFormat, imageops::FilterType};

const SUPERLOTTO_MAIN_COORDS: [(i32, i32); 48] =
    [(0,0),(104, 390),(178, 390),(252, 390),(326, 390),(400, 390),(474, 390),
           (104, 427),(178, 427),(252, 427),(326, 427),(400, 427),(474, 427),
           (104, 464),(178, 464),(252, 464),(326, 464),(400, 464),(474, 464),
           (104, 501),(178, 501),(252, 501),(326, 501),(400, 501),(474, 501),
           (104, 538),(178, 538),(252, 538),(326, 538),(400, 538),(474, 538),
           (104, 575),(178, 575),(252, 575),(326, 575),(400, 575),(474, 575),
           (104, 612),(178, 612),(252, 612),(326, 612),(400, 612),(474, 612),
           (104, 649),(178, 649),(252, 649),(326, 649),(400, 649),];
const SUPERLOTTO_MEGA_COORDS: [(i32, i32); 28] =
    [(0,0),(547, 392),(621, 392),(695, 392),(769, 392),(843, 392),
           (547, 428),(621, 428),(695, 428),(769, 428),(843, 428),
           (547, 464),(621, 464),(695, 464),(769, 464),(843, 464),
           (547, 500),(621, 500),(695, 500),(769, 500),(843, 500),
           (547, 536),(621, 536),(695, 536),(769, 536),(843, 536),
           (547, 572),(621, 572),];

fn main() -> io::Result<()> {
    // initialize the environment If .env isn't found then it will default to debug level
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    if log_enabled!(Level::Debug) {
        for (key, value) in dotenvy::vars() { // prints out EVERY evn variable
            println!("{}: {}", key, value);   // not just the ones in the .env file
        }
    }   

    if let Ok(()) = process_csv_file("csv/slnumbers1.csv") {
        let _ = gen_lotto_charts("csv/csv_out.csv");
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
// reads existing csv output file and generates charts showing the numbers that were picked
fn gen_lotto_charts(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Generating charts from CSV file: {}", csv_file);
    //let csv_file_name = csv_file;
    if let Ok(lines) = read_lines(csv_file) {
        for line in lines.flatten() {
            debug!("CSV Line: {}", line);
            let date_field = &line[1..line.find(",").unwrap_or(line.len())-1];
            if date_field.starts_with("at") {
                //skip header line - it begins with Date or slice = at
                continue;
            }
            let ext: &str = ".png";
            let file_name = format!("chart/{date_field}{ext}");
            let path = Path::new(&file_name);
            match fs::metadata(path) {
                Ok(_) => info!("{} exists", file_name),
                Err(_) => {
                    info!("creating  chart {}", file_name);
                    //let chart_file = "chart/lotto_chart.png";
                    let root = BitMapBackend::new(&file_name, (1024, 768)).into_drawing_area();
                    root.fill(&WHITE).expect("Failed to fill drawing area");

                    let mut chart = ChartBuilder::on(&root)
                        .caption(date_field, ("sans-serif", 30))
                        .margin(5)
                        .set_label_area_size(LabelAreaPosition::Left, 40)
                        .set_label_area_size(LabelAreaPosition::Bottom, 40)
                        .build_cartesian_2d(0.0..1.0, 0.0..1.0)?;

                    chart.configure_mesh().disable_mesh().draw().expect("Failed to disable mesh");

                    let (w, h) = chart.plotting_area().dim_in_pixel();
                    let lottoimage = image::load(
                        BufReader::new(File::open("img/slPlayslip.png").map_err(|e| {
                            eprintln!("Unable to open folder plotters-doc-data, please make sure folder exists");
                            e 
                        })?),
                        ImageFormat::Png,
                    )?
                    .resize_exact(w - w / 10, h - h / 10, FilterType::Nearest); 

                    let lottoelem: BitMapElement<_> = ((0.05, 0.95), lottoimage).into();
                    chart.draw_series(std::iter::once(lottoelem))?; // Draw playslip image

                    //let file = File::create(file_name).unwrap();
                    debug!("{}", date_field);
                    let numbers_field = &line[line.find(",").unwrap() + 1..];
                    let numbers_vec: Vec<&str> = numbers_field.split(',').collect();
                    debug!("Numbers_vec: {:?}", numbers_vec);
                    let mut x1: i32;
                    let mut y1: i32;
                    let x2: i32;  // x2 & y2 are only used once
                    let y2: i32;
                    let mut index_str: &str;
                    let mut index_number: i32;
                    for i in 0..5 {
                        index_str = numbers_vec[i].trim();
                        debug!("i: {} index_str: {}", i, index_str);
                        index_number = index_str.parse::<i32>().unwrap_or(10);
                        debug!("i: {} index_number: {}", i, index_number);
                        x1 = SUPERLOTTO_MAIN_COORDS[index_number as usize].0;
                        y1 = SUPERLOTTO_MAIN_COORDS[index_number as usize].1;
                        debug!("i: {} x1: {} y1: {}", i, x1, y1);
                        let _ = root.draw(&Rectangle::new([(x1, y1), (x1 + 45, y1 + 30)],
                                Into::<ShapeStyle>::into(&BLUE).stroke_width(4),))
                                .expect("Failed to draw rectangle");
                    }
                    debug!("Mega: {}", numbers_vec[5]);
                    let mega_number = numbers_vec[5].parse::<i32>().unwrap_or(0);
                    x2 = SUPERLOTTO_MEGA_COORDS[mega_number as usize].0;
                    y2 = SUPERLOTTO_MEGA_COORDS[mega_number as usize].1;
                    let _ = root.draw(&Rectangle::new([(x2, y2), (x2 + 45, y2 + 30)],
                            Into::<ShapeStyle>::into(&GREEN).stroke_width(4),))
                            .expect("Failed to draw rectangle");
                    //let mut line_writer = LineWriter::new(file);
                    //writeln!(line_writer, "{}", numbers_field)?;
                    debug!("Numbers_field: {}", numbers_field);
                    root.present().expect("Failed to write .png file");
                }
            }
            debug!("Result has been saved to {}", file_name);
        }
    }

    Ok(())
}
// reads the lines of csv input file and formats for csv output file
fn process_csv_file(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("csv/csv_out.csv")?;
    let mut line_writer = LineWriter::new(file);
    if let Ok(lines) = read_lines(csv_file) {
        for line in lines.flatten() {
            if line.starts_with("\"Wed") {
                let w_slice: &str = &line[1..24]; //skip the dbl-quotes so it is recognized as a valid date

                // reformat the full date string into a shorter format
                let ndin =
                    NaiveDate::parse_from_str(w_slice, "%A, %b %d, %Y").expect("Invalid Date");
                //write out Ymd-day format to use for file names to keep them in chronological order
                let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());

                // get the rest of the numbers being sure to remove the one dbl-quote embedded in slice
                // also remove jackpot field
                let comma_index = line.rfind(",");
                let nbr_slice: &str = &line[27..comma_index.unwrap_or(line.len())].replace("\"", "");

                debug!("Date: {} Numbers: {}", ndout, nbr_slice);

                // insert dbl-quotes around the date and add the numbers without dbl-quotes
                let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                writeln!(line_writer, "{}", out_line).expect("Error writing Wednesday line");
            } else if line.starts_with("\"Sat") {
                let s_slice: &str = &line[1..23]; //skip the dbl-quotes so it is recognized as a valid date

                // reformat the full date string into a shorter format
                let ndin =
                    NaiveDate::parse_from_str(s_slice, "%A, %b %d, %Y").expect("Invalid Date");
                //write out Ymd-day format to use for file names to keep them in chronological order
                let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());

                // get the rest of the numbers being sure to remove the one dbl-quote embedded in slice
                // also remove jackpot field
                let comma_index = line.rfind(",");
                let nbr_slice: &str = &line[26..comma_index.unwrap_or(line.len())].replace("\"", "");

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
    Ok(())
}
// Returns an iterator over the lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

