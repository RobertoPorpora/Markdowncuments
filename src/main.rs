use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

// -----------------------------------------------------------------------------

mod command_line;
mod file_explorer;
mod html_from_md;
mod pdf_from_html;

// -----------------------------------------------------------------------------

fn main() -> Result<(), i32> {
    println!("");
    let version = env!("CARGO_PKG_VERSION");
    println!("Markdowncuments version {}", version);
    println!("");

    let user_request = match command_line::parse() {
        Ok(ur) => ur,
        Err(e) => {
            eprintln!("Error while parsing command line: {}", e);
            eprintln!("Consider using \"markdowncuments --help\".");
            return Err(1);
        }
    };

    if user_request.help {
        command_line::print_help();
        return Ok(());
    }

    if user_request.tricks {
        command_line::print_tricks();
        return Ok(());
    }

    if user_request.version {
        return Ok(());
    }

    println!("User request = {:#?}", user_request);
    println!("");

    let puml_files = file_explorer::find_all("puml", &user_request.folder, true);

    for puml_file in puml_files {
        println!("Found .puml file: {:?}", puml_file);

        match std::process::Command::new("plantuml")
            .arg(puml_file)
            .status()
        {
            Ok(_) => {
                println!("SVG file created.");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    let md_files = file_explorer::find_all("md", &user_request.folder, true);

    for md_file in md_files {
        println!("Found .md file: {:?}", md_file);

        let mut file = match File::open(&md_file) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error while opening Markdown file: {}", e);
                continue;
            }
        };
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while reading Markdown file: {}", e);
                continue;
            }
        }

        // convert md to html
        let title: &str = filename_as_str(&md_file);
        let html_content = html_from_md::convert(title, content);

        // create the path for the new .html file
        let html_file_path = md_file.with_extension("html");

        // create a new .html file and write the processed content
        let mut html_file = match File::create(&html_file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error while creating HTML file: {}", e);
                continue;
            }
        };
        match html_file.write_all(html_content.as_bytes()) {
            Ok(_) => println!("HTML file created."),
            Err(e) => {
                eprintln!("Error while writing HTML file: {}", e);
                continue;
            }
        }
        drop(html_file); // close the file

        println!("HTML file created.");

        // convert the html file
        if user_request.pdf {
            match pdf_from_html::convert_and_write(&html_file_path, &user_request) {
                Ok(_) => println!("PDF file created."),
                Err(e) => eprintln!("Error while creating PDF file: {}", e),
            }
        }

        if !user_request.html {
            match std::fs::remove_file(&html_file_path) {
                Ok(_) => println!("HTML file removed."),
                Err(e) => eprintln!("Error while removing HTML file: {}", e),
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

use std::ffi::OsStr;

fn filename_as_str(path: &PathBuf) -> &str {
    path.file_stem().and_then(OsStr::to_str).unwrap_or_default()
}
