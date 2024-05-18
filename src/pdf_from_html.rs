use anyhow::Result;
use headless_chrome::{types::PrintToPdfOptions, Browser};
use std::{fs, path::PathBuf};

pub fn convert_and_write(html_path: &PathBuf) -> Result<()> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let file_path = format!("file://{}", html_path.to_str().unwrap());

    // Browse to the file url and render a pdf of the web page.
    let pdf_options: Option<PrintToPdfOptions> = None; // use chrome's defaults for this example
    let local_pdf = tab
        .navigate_to(&file_path)?
        .wait_until_navigated()?
        .print_to_pdf(pdf_options)?;

    fs::write(html_path.with_extension("pdf"), local_pdf)?;
    println!("PDF successfully created from local web page.");

    Ok(())
}
