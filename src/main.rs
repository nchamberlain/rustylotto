// Reads a superlotto csv file from https://www.lotteryusa.com/california/super-lotto-plus/year
// and writes out a new csv file with the date in a different format and the numbers comma separated
// instead of being embedded in a single dbl-quoted string.
use chrono::NaiveDate;
use std::fs::File;
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, LineWriter, Write};
use std::path::Path;
use log::{error, warn, info, debug, trace, log_enabled, Level};
use dotenvy::dotenv;
use plotters::prelude::*;
use image::{ImageFormat, imageops::FilterType};

fn main() -> io::Result<()> {
    // initialize the environment If .env isn't found then it will default to debug level
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    if log_enabled!(Level::Debug) {
        for (key, value) in dotenvy::vars() { // prints out EVERY evn variable
            println!("{}: {}", key, value);   // not just the ones in the .env file
        }
    }   

    let args: Vec<String> = env::args().collect();
    debug!("Args: {:?}", args);
    
    if args.len() == 2 {
        let arg1 = args[1].as_str();
        info!("Arg1: {}", arg1);
        match arg1 {
            "fant" => {
                debug!("Ready to read csv/fant/csv_in.csv");
                if let Ok(()) = process_fantasy5_csv_file("csv/fant/csv_in.csv") {
                    let _ = gen_fantasy5_charts("csv/fant/csv_out.csv");
                }
            }
            "mega" => {
                info!("Ready to read csv/mega/csv_in.csv");
                if let Ok(()) = process_megam_csv_file("csv/mega/csv_in.csv") {
                    let _ = gen_megam_charts("csv/mega/csv_out.csv");
                }
            }
            "power" => {
                info!("Ready to read csv/power/csv_in.csv");
                if let Ok(()) = process_power_csv_file("csv/power/csv_in.csv") {
                    let _ = gen_power_charts("csv/power/csv_out.csv");
                }
            }
            "super" => {
                info!("Ready to read csv/super/csv_in.csv");
                if let Ok(()) = process_super_csv_file("csv/super/csv_in.csv") {
                    let _ = gen_superlotto_charts("csv/super/csv_out.csv");
                }
            }
            _ => {
                info!("Unknown arg: {}. Please enter 'fant', 'mega', 'power', or 'super'", arg1);
            }
        }
    } else {
        info!("Please provide a command line argument: 'fant', 'mega', 'power', or 'super'");
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
fn gen_megam_charts(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Reading MegaMillion CSV file: {} and generating charts", csv_file);
    //if true {
    //    debug!("Debug level is active (lots of details)");
    //    return Ok(());
    //}
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
            let file_name = format!("chart/mega/{date_field}{ext}");
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
                        BufReader::new(File::open("img/megaPlayslip.png").map_err(|e| {
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
                        x1 = MEGAMILLION_MAIN_COORDS[index_number as usize].0;
                        y1 = MEGAMILLION_MAIN_COORDS[index_number as usize].1;
                        debug!("i: {} x1: {} y1: {}", i, x1, y1);
                        let _ = root.draw(&Rectangle::new([(x1, y1), (x1 + 44, y1 + 30)],
                                Into::<ShapeStyle>::into(&BLUE).stroke_width(4),))
                                .expect("Failed to draw rectangle");
                    }
                    debug!("Mega: {}", numbers_vec[5]);
                    let mega_number = numbers_vec[5].parse::<i32>().unwrap_or(0);
                    x2 = MEGAMILLION_MEGA_COORDS[mega_number as usize].0;
                    y2 = MEGAMILLION_MEGA_COORDS[mega_number as usize].1;
                    let _ = root.draw(&Rectangle::new([(x2, y2), (x2 + 44, y2 + 30)],
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
// reads the lines of mega csv input file and formats for csv output file
fn process_megam_csv_file(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("csv/mega/csv_out.csv")?;
    let mut line_writer = LineWriter::new(file);
    if let Ok(lines) = read_lines(csv_file) {
        for line in lines.flatten() {
            if line.starts_with("\"Tue") {
                let w_slice: &str = &line[1..22]; //skip the dbl-quotes so it is recognized as a valid date

                // reformat the full date string into a shorter format
                let ndin =
                    NaiveDate::parse_from_str(w_slice, "%A, %b %d, %Y").expect("Invalid Date");
                //write out Ymd-day format to use for file names to keep them in chronological order
                let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());

                // get the rest of the numbers being sure to remove the one dbl-quote embedded in slice
                // also remove jackpot field
                let comma_index = line.find(",N/A");
                let nbr_slice: &str = &line[25..comma_index.unwrap_or(line.len())].replace("\"", "");

                info!("Date: {} Numbers: {}", ndout, nbr_slice);

                // insert dbl-quotes around the date and add the numbers without dbl-quotes
                let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                writeln!(line_writer, "{}", out_line).expect("Error writing Tuesday line");
            } else if line.starts_with("\"Fri") {
                let s_slice: &str = &line[1..21]; //skip the dbl-quotes so it is recognized as a valid date

                // reformat the full date string into a shorter format
                let ndin =
                    NaiveDate::parse_from_str(s_slice, "%A, %b %d, %Y").expect("Invalid Date");
                //write out Ymd-day format to use for file names to keep them in chronological order
                let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());

                // get the rest of the numbers being sure to remove the one dbl-quote embedded in slice
                // also remove jackpot field
                let comma_index = line.find(",N/A");
                let nbr_slice: &str = &line[24..comma_index.unwrap_or(line.len())].replace("\"", "");

                info!("Date= {} Numbers= {}", ndout, nbr_slice);
                
                // insert dbl-quotes around the date and add the numbers without dbl-quotes
                let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                writeln!(line_writer, "{}", out_line).expect("Error writing Saturday line");
            } else if line.starts_with("Date") { // processes only 1st line with column headers
                debug!("Date,C1,C2,C3,C4,C5,Mega");
                let out_line = "Date,C1,C2,C3,C4,C5,Mega".to_string(); //new column headers
                writeln!(line_writer, "{}", out_line).expect("Error writing column headers");
            } else if line.trim().is_empty() { //skip blank lines
                debug!("Blank line skipped");   
            } else {
                eprintln!("Mega Line NOT Recognized"); //should never happen
            }
        }
    }
    Ok(())
}
// reads existing power csv output file and generates charts showing the numbers that were picked
fn gen_power_charts(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Reading PowerBall CSV file: {} and generating charts", csv_file);
    if let Ok(lines) = read_lines(csv_file) {
        for line in lines.flatten() {
            debug!("CSV Line: {}", line);
            let date_field = &line[1..line.find(",").unwrap_or(line.len())-1];
            if date_field.starts_with("at") {
                continue;
            }
            let ext: &str = ".png";
            let file_name = format!("chart/power/{date_field}{ext}");
            let path = Path::new(&file_name);
            match fs::metadata(path) {
                Ok(_) => info!("{} exists", file_name),
                Err(_) => {
                    info!("creating  chart {}", file_name);
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
                        BufReader::new(File::open("img/pbPlayslip.png").map_err(|e| {
                            eprintln!("Unable to open folder plotters-doc-data, please make sure folder exists");
                            e 
                        })?),
                        ImageFormat::Png,
                    )?
                    .resize_exact(w - w / 10, h - h / 10, FilterType::Nearest); 
                    let lottoelem: BitMapElement<_> = ((0.05, 0.95), lottoimage).into();
                    chart.draw_series(std::iter::once(lottoelem))?; // Draw playslip image
                    info!("Date_field: {}", date_field);
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
                        x1 = POWERBALL_MAIN_COORDS[index_number as usize].0;
                        y1 = POWERBALL_MAIN_COORDS[index_number as usize].1;
                        debug!("i: {} x1: {} y1: {}", i, x1, y1);
                        let _ = root.draw(&Rectangle::new([(x1, y1), (x1 + 34, y1 + 34)],
                                Into::<ShapeStyle>::into(&BLUE).stroke_width(4),))
                                .expect("Failed to draw rectangle");
                    }
                    info!("Mega: {}", numbers_vec[5]);
                    let mega_number = numbers_vec[5].parse::<i32>().unwrap_or(0);
                    x2 = POWERBALL_MEGA_COORDS[mega_number as usize].0;
                    y2 = POWERBALL_MEGA_COORDS[mega_number as usize].1;
                    let _ = root.draw(&Rectangle::new([(x2, y2), (x2 + 34, y2 + 34)],
                            Into::<ShapeStyle>::into(&GREEN).stroke_width(4),))
                            .expect("Failed to draw rectangle");
                    info!("Numbers_field: {}", numbers_field);
                    root.present().expect("Failed to write .png file");
                }
            }
            debug!("Result has been saved to {}", file_name);
        }
    }

    Ok(())
}
// reads the lines of power csv input file and formats for csv output file
fn process_power_csv_file(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("csv/power/csv_out.csv")?;
    let mut line_writer = LineWriter::new(file);
    let mut day_starts_with: &str;
    if let Ok(lines) = read_lines(csv_file) {
        for line in lines.flatten() {
            if line.trim().is_empty() { //skip blank lines  
                info!("Blank line skipped");   
                continue;
            }
            day_starts_with = &line[1..3];
            match day_starts_with {
                "Mo" => {debug!("Line starts with Monday");
                    let w_slice: &str = &line[1..21]; //skip the dbl-quotes so it is parsed ok
                    let ndin =
                        NaiveDate::parse_from_str(w_slice, "%A, %b %d, %Y").expect("Invalid Date");
                    let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());
                    let comma_index = line.find(",x");
                    let nbr_slice: &str = &line[24..comma_index.unwrap_or(line.len())].replace("\"", "");
                    info!("Date: {} Numbers: {}", ndout, nbr_slice);
                    let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                    writeln!(line_writer, "{}", out_line).expect("Error writing Monday line");
                },
                "Sa" => {debug!("Line starts with Saturday");
                    let w_slice: &str = &line[1..23]; //skip the dbl-quotes so it is parsed ok
                    let ndin =
                        NaiveDate::parse_from_str(w_slice, "%A, %b %d, %Y").expect("Invalid Date");
                    let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());
                    let comma_index = line.find(",x");
                    let nbr_slice: &str = &line[26..comma_index.unwrap_or(line.len())].replace("\"", "");
                    info!("Date: {} Numbers: {}", ndout, nbr_slice);
                    let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                    writeln!(line_writer, "{}", out_line).expect("Error writing Saturday line");
                    },
                "We" => { debug!("Line starts with Wednesday"); 
                    let w_slice: &str = &line[1..24]; //skip the dbl-quotes so it is parsed ok
                    let ndin =
                        NaiveDate::parse_from_str(w_slice, "%A, %b %d, %Y").expect("Invalid Date");
                    let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());
                    let comma_index = line.find(",x");
                    let nbr_slice: &str = &line[27..comma_index.unwrap_or(line.len())].replace("\"", "");
                    info!("Date: {} Numbers: {}", ndout, nbr_slice);
                    let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                    writeln!(line_writer, "{}", out_line).expect("Error writing Wednesday line");
                    },
                "at" => { debug!("Line starts with Date"); 
                    info!("Date,C1,C2,C3,C4,C5");
                    let out_line = "Date,C1,C2,C3,C4,C5,Mega".to_string(); //new column headers
                    writeln!(line_writer, "{}", out_line).expect("Error writing column headers");
                    },
                _ => eprintln!("Fantasy5 Line NOT Recognized"),
            }
        }
    }
    Ok(())
}
// reads existing csv output file and generates charts showing the numbers that were picked
fn gen_fantasy5_charts(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Reading Fantasy5 CSV file: {} and generating charts", csv_file);
    if let Ok(lines) = read_lines(csv_file) {
        for line in lines.flatten() {
            debug!("CSV Line: {}", line);
            let date_field = &line[1..line.find(",").unwrap_or(line.len())-1];
            if date_field.starts_with("at") { //skip header line - it begins with Date or slice = at     
                continue;
            }
            let ext: &str = ".png";
            let file_name = format!("chart/fant/{date_field}{ext}");
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
                        BufReader::new(File::open("img/f5Playslip.png").map_err(|e| {
                            eprintln!("Unable to open folder plotters-doc-data, please make sure folder exists");
                            e 
                        })?),
                        ImageFormat::Png,
                    )?
                    .resize_exact(w - w / 10, h - h / 10, FilterType::Nearest); 
                    let lottoelem: BitMapElement<_> = ((0.05, 0.95), lottoimage).into();
                    chart.draw_series(std::iter::once(lottoelem))?; // Draw playslip image

                    info!("{}", date_field);
                    let numbers_field = &line[line.find(",").unwrap() + 1..];
                    let numbers_vec: Vec<&str> = numbers_field.split(',').collect();
                    debug!("Numbers_vec: {:?}", numbers_vec);
                    let mut x1: i32;
                    let mut y1: i32;
                    let mut index_str: &str;
                    let mut index_number: i32;
                    for i in 0..5 {
                        index_str = numbers_vec[i].trim();
                        debug!("i: {} index_str: {}", i, index_str);
                        index_number = index_str.parse::<i32>().unwrap_or(10);
                        debug!("i: {} index_number: {}", i, index_number);
                        x1 = FANTASY5_MAIN_COORDS[index_number as usize].0;
                        y1 = FANTASY5_MAIN_COORDS[index_number as usize].1;
                        debug!("i: {} x1: {} y1: {}", i, x1, y1);
                        let _ = root.draw(&Rectangle::new([(x1, y1), (x1 + 34, y1 + 33)],
                                Into::<ShapeStyle>::into(&BLUE).stroke_width(4),))
                                .expect("Failed to draw rectangle");
                    }
                    root.present().expect("Failed to write .png file");
                }
            }
            debug!("Result has been saved to {}", file_name);
        }
    }

    Ok(())
}
// reads the lines of fantasy 5 csv input file and formats for csv output file
fn process_fantasy5_csv_file(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("csv/fant/csv_out.csv")?;
    let mut line_writer = LineWriter::new(file);
    let mut day_starts_with: &str;
    if let Ok(lines) = read_lines(csv_file) {
        for line in lines.flatten() {
            if line.trim().is_empty() { //skip blank lines  
                debug!("Blank line skipped");   
                continue;
            }
            day_starts_with = &line[1..3];
            match day_starts_with {
                "Su" | "Mo" | "Fr" => {debug!("Line starts with Sunday, monday, or Friday");
                    let w_slice: &str = &line[1..21]; //skip the dbl-quotes so it is parsed ok
                    let ndin =
                        NaiveDate::parse_from_str(w_slice, "%A, %b %d, %Y").expect("Invalid Date");
                    let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());
                    let comma_index = line.rfind(",\"$");
                    let nbr_slice: &str = &line[24..comma_index.unwrap_or(line.len())].replace("\"", "");
                    info!("Date: {} Numbers: {}", ndout, nbr_slice);
                    let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                    writeln!(line_writer, "{}", out_line).expect("Error writing Wednesday line");
                },
                "Tu" => {debug!("Line starts with Tue");
                    let w_slice: &str = &line[1..22]; //skip the dbl-quotes so it is parsed ok
                    let ndin =
                        NaiveDate::parse_from_str(w_slice, "%A, %b %d, %Y").expect("Invalid Date");
                    let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());
                    let comma_index = line.rfind(",\"$");
                    let nbr_slice: &str = &line[25..comma_index.unwrap_or(line.len())].replace("\"", "");
                    info!("Date: {} Numbers: {}", ndout, nbr_slice);
                    let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                    writeln!(line_writer, "{}", out_line).expect("Error writing Wednesday line");
                },
                "Th" | "Sa" => {debug!("Line starts with Thursday or Saturday");
                    let w_slice: &str = &line[1..23]; //skip the dbl-quotes so it is parsed ok
                    let ndin =
                        NaiveDate::parse_from_str(w_slice, "%A, %b %d, %Y").expect("Invalid Date");
                    let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());
                    let comma_index = line.rfind(",\"$");
                    let nbr_slice: &str = &line[26..comma_index.unwrap_or(line.len())].replace("\"", "");
                    info!("Date: {} Numbers: {}", ndout, nbr_slice);
                    let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                    writeln!(line_writer, "{}", out_line).expect("Error writing Wednesday line");
                    },
                "We" => { debug!("Line starts with Wednesday"); 
                    let w_slice: &str = &line[1..24]; //skip the dbl-quotes so it is parsed ok
                    let ndin =
                        NaiveDate::parse_from_str(w_slice, "%A, %b %d, %Y").expect("Invalid Date");
                    let ndout: String = format!("{}", ndin.format("%Y-%m-%d-%a").to_string());
                    let comma_index = line.rfind(",\"$");
                    let nbr_slice: &str = &line[27..comma_index.unwrap_or(line.len())].replace("\"", "");
                    info!("Date: {} Numbers: {}", ndout, nbr_slice);
                    let out_line = format!("\"{}\",{}", ndout, nbr_slice);
                    writeln!(line_writer, "{}", out_line).expect("Error writing Wednesday line");
                    },
                "at" => { debug!("Line starts with Date"); 
                    info!("Date,C1,C2,C3,C4,C5");
                    let out_line = "Date,C1,C2,C3,C4,C5".to_string(); //new column headers
                    writeln!(line_writer, "{}", out_line).expect("Error writing column headers");
                    },
                _ => eprintln!("Fantasy5 Line NOT Recognized"),
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

// reads existing csv output file and generates charts showing the numbers that were picked
fn gen_superlotto_charts(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Reading SuperLotto CSV file: {} and generating charts", csv_file);
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
            let file_name = format!("chart/super/{date_field}{ext}");
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
fn process_super_csv_file(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("csv/super/csv_out.csv")?;
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
            } else if line.trim().is_empty() { //skip blank lines usually at end of file
                debug!("Blank line skipped");   
            } else {
                eprintln!("Line NOT Recognized"); //should never happen
            }
        }
    }
    Ok(())
}

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
const MEGAMILLION_MAIN_COORDS: [(i32, i32); 71] =
    [(0,0),                                            (400, 361),(472, 361),(545, 361),(618, 361),(692, 361),(767, 361),(838, 361),(914, 361), // 1- 8
           (105, 398),(178, 398),(251, 398),(325, 398),(400, 398),(472, 398),(545, 398),(618, 398),(692, 398),(767, 398),(838, 398),(914, 398), // 9-20
           (105, 436),(178, 436),(251, 436),(325, 436),(400, 436),(472, 436),(545, 436),(618, 436),(692, 436),(767, 436),(838, 436),(914, 436), //21-32
           (105, 472),(178, 472),(251, 472),(325, 472),(400, 472),(472, 472),(545, 472),(618, 472),(692, 472),(767, 472),(838, 472),(914, 472), //33-44
           (105, 510),(178, 510),(251, 510),(325, 510),(400, 510),(472, 510),(545, 510),(618, 510),(692, 510),(767, 510),(838, 510),(914, 510), //45-56
           (105, 548),(178, 548),(251, 548),(325, 548),(400, 548),(472, 548),(545, 548),(618, 548),(692, 548),(767, 548),(838, 548),(914, 548), //57-68
           (105, 584),(178, 584),];
const MEGAMILLION_MEGA_COORDS: [(i32, i32); 25] =
    [(0,0),                                                                                                              (838, 585),(914, 585),
           (105, 621),(178, 621),(251, 621),(325, 621),(400, 621),(472, 621),(545, 621),(618, 621),(692, 621),(767, 621),(838, 621),(914, 621),
           (105, 656),(178, 656),(251, 656),(325, 656),(400, 656),(472, 656),(545, 656),(618, 656),(692, 656),(767, 656),];
const POWERBALL_MAIN_COORDS: [(i32, i32); 70] =
    [(0,0),(144, 250),(184, 250),(222, 250),(260, 250),(298, 250),(333, 250),(370, 250),(406, 250),(442, 250),(478, 250), // 1-10
           (144, 310),(184, 310),(222, 310),(260, 310),(298, 310),(333, 310),(370, 310),(406, 310),(442, 310),(478, 310), //11-20
           (144, 370),(184, 370),(222, 370),(260, 370),(298, 370),(333, 370),(370, 370),(406, 370),(442, 370),(478, 370), //21-30
           (144, 430),(184, 430),(222, 430),(260, 430),(298, 430),(333, 430),(370, 430),(406, 430),(442, 430),(478, 430), //31-40
           (144, 490),(184, 490),(222, 490),(260, 490),(298, 490),(333, 490),(370, 490),(406, 490),(442, 490),(478, 490), //41-50
           (144, 550),(184, 550),(222, 550),(260, 550),(298, 550),(333, 550),(370, 550),(406, 550),(442, 550),(478, 550), //51-60
           (144, 610),(184, 610),(222, 610),(260, 610),(298, 610),(333, 610),(370, 610),(406, 610),(442, 610),];
const POWERBALL_MEGA_COORDS: [(i32, i32); 27] =
    [(0,0),(518, 370),(558, 370),(595, 370),(633, 370),(671, 370),(705, 370),(742, 370),(778, 370),(816, 370),(853, 370),
           (518, 430),(558, 430),(595, 430),(633, 430),(671, 430),(705, 430),(742, 430),(778, 430),(816, 430),(853, 430),
           (518, 490),(558, 490),(595, 490),(633, 490),(671, 490),(705, 490),];
const FANTASY5_MAIN_COORDS: [(i32, i32); 40] =
    [(0,0),(196,  95),(241,  95),(290,  95),(338,  95),
           (196, 154),(241, 154),(290, 154),(338, 154),
           (196, 214),(241, 214),(290, 214),(338, 214),
           (196, 274),(241, 274),(290, 274),(338, 274),
           (196, 334),(241, 334),(290, 334),(338, 334), 
           (196, 394),(241, 394),(290, 394),(338, 394), 
           (196, 452),(241, 452),(290, 452),(338, 452), 
           (196, 511),(241, 511),(290, 511),(338, 511),
           (196, 574),(241, 574),(290, 574),(338, 574),
           (196, 635),(241, 635),(291, 635), ];
// Fantasy 5 doesn't have mega numbers
