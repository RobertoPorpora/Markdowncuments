use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};

mod html_from_md;
mod pdf_from_html;

fn main() -> io::Result<()> {
    // Get the path of the executable file
    let current_dir = std::env::current_dir()?;

    // Read all files in the current directory with extension "md"
    let md_files: Vec<_> = fs::read_dir(&current_dir)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| if ext == "md" { Some(e.path()) } else { None })
            })
        })
        .collect();

    // Loop over each .md file
    for md_file in md_files {
        // Read the content of the file
        let mut file = File::open(&md_file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // Convert md to html
        let title: &str = filename_as_str(&md_file);
        let html_content = html_from_md::convert(title, content);

        // Create the path for the new .html file
        let html_file_path = md_file.with_extension("html");

        // Create a new .html file and write the processed content
        let mut html_file = File::create(&html_file_path)?;
        html_file.write_all(html_content.as_bytes())?;

        println!("HTML file created: {:?}", html_file_path);

        match pdf_from_html::convert_and_write(&html_file_path) {
            Err(err) => eprintln!("Error while creating PDF: {}", err),
            Ok(_) => println!("Pdf file created!"),
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

use std::ffi::OsStr;

fn filename_as_str(path: &PathBuf) -> &str {
    path.file_stem().and_then(OsStr::to_str).unwrap_or_default()
}
