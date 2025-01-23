use std::{
    env,
    error::Error,
    ffi::OsString,
    process
};

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let output_path = get_second_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut wtr = csv::Writer::from_path(output_path)?;
    // Write out header
    // Codepipeline removed, but exists as record[4] in old data, so after "codepipeline" starts as record[5] and so on
    wtr.write_record(&["teamId", "accounts", "applicationCI", "codepipeline", "provider", "costCenter", "createDate", "description", "ldap", "organization", "ownerAmgenId", "ownerId", "teamName", "wbsCode"])?;
    for result in rdr.records() {
        let record = result?;

        let accounts_copy = String::from(&record[1]);
        let substrings = accounts_copy.split("\"");

        let mut prev_string = "";
        let mut application_ci = String::new();
        
        for substring in substrings {
            if prev_string == ":" && substring != "{" {
                let mut new_application_ci = "\"".to_owned();
                new_application_ci = new_application_ci + &record[2] + "\"";
                application_ci.push_str(&new_application_ci)
            } else {
                match substring {
                    "Dev" | "Test" | "Prod" | "S" => {
                        let mut current_string = "\"".to_owned();
                        current_string =  current_string + &substring + "\"";

                        application_ci.push_str(&current_string);                        
                    }
                    _ => {
                        application_ci.push_str(&substring);
                    }
                }
            }
            prev_string = &substring;
        }

        println!("{:?}", application_ci);

        wtr.write_record(&[&record[0], &record[1], &application_ci, &record[3], "AWS", &record[5], &record[6], &record[8], &record[9], &record[10], &record[11], &record[12], &record[14], &record[15]])?;
        
        // println!("{:?}", accounts)
    }

    wtr.flush()?;
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path)
    }
}

fn get_second_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(2) {
        None => Err(From::from("expected 2 arguments, but got none")),
        Some(file_path) => Ok(file_path)
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}