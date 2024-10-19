use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

mod html_from_md;
mod pdf_from_html;
mod user;

// -----------------------------------------------------------------------------

fn parse_command_line() -> Result<user::Options, String> {
    let mut options = user::Options::default();
    let mut args = std::env::args().skip(1); // Skip the program name
    let mut folder_provided = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-f" | "--full" => options.full = true,
            "-t" | "--html" => options.html = true,
            "-x" | "--no-pdf" => options.pdf = false,
            "-h" | "--help" => {
                options.help = true;
            }
            _ => {
                if !arg.starts_with('-') {
                    options.folder = PathBuf::from(arg);
                    folder_provided = true;
                } else {
                    return Err(format!("Unknown option: {}", arg));
                }
            }
        }
    }

    // Set the folder to the current working directory if not provided
    if !folder_provided {
        options.folder = match std::env::current_dir() {
            Ok(p) => p,
            Err(e) => {
                return Err(format!("Error while trying to resolve cwd: {}", e));
            }
        };
    }

    Ok(options)
}

fn print_help() {
    println!(
        "\
Usage: markdowncuments [folder] [options]

Options:
    -f, --full               Use full template with header and footer
    -t, --html               Output in HTML format also
    -x, --no-pdf             Disable PDF output
    -h, --help               Display this help message"
    );
}

// -----------------------------------------------------------------------------

fn main() -> Result<(), i32> {
    const SEPARATOR: &str = "";

    println!("{}", SEPARATOR);

    let version = env!("CARGO_PKG_VERSION");
    println!("Markdowncuments version {}", version);

    println!("{}", SEPARATOR);

    let user = match parse_command_line() {
        Ok(uo) => uo,
        Err(e) => {
            eprintln!("Error while parsing command line: {}", e);
            eprintln!("Consider using \"markdowncuments --help\".");
            return Err(1);
        }
    };

    if user.help {
        print_help();
        return Ok(());
    }

    println!("User input = {:?}", user);

    // read all files in the current directory with extension "md"
    let md_files: Vec<_> = match std::fs::read_dir(&user.folder) {
        Ok(rd) => rd,
        Err(e) => {
            eprintln!("Error while reading the directory: {}", e);
            return Err(1);
        }
    }
    .filter_map(|entry| {
        entry.ok().and_then(|e| {
            e.path()
                .extension()
                .and_then(|ext| if ext == "md" { Some(e.path()) } else { None })
        })
    })
    .collect();

    // loop over each .md file
    for md_file in md_files {
        println!("{}", SEPARATOR);
        println!("Found .md file: {:?}", md_file);

        // read the content of the file
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

        // convert the html file
        if user.pdf {
            match pdf_from_html::convert_and_write(&html_file_path, &user) {
                Ok(_) => println!("PDF file created."),
                Err(e) => eprintln!("Error while creating PDF file: {}", e),
            }
        }

        if !user.html {
            match std::fs::remove_file(&html_file_path) {
                Ok(_) => println!("HTML file removed."),
                Err(e) => eprintln!("Error while removing HTML file: {}", e),
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

use std::ffi::OsStr;

fn filename_as_str(path: &PathBuf) -> &str {
    path.file_stem().and_then(OsStr::to_str).unwrap_or_default()
}
