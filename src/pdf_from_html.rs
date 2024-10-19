use anyhow::Result;
use chrono;
use headless_chrome::{types::PrintToPdfOptions, Browser};
use std::{fs, path::PathBuf};

use crate::command_line;

const TITLE_TEMPLATE: &str = "\
<div style=\"font-size: 0.3cm; margin-left: 1cm;\">\
<span class='title'></span>\
</div>";

const DATE_TEMPLATE: &str = "\
<div style=\"font-size: 0.3cm; margin-left: auto; margin-right: 1cm;\">\
%DATE%\
</div>";

const NUMBERING_TEMPLATE: &str = "\
<div style=\"font-size: 0.3cm; margin: 0 auto;\">\
<span class='pageNumber'></span> / <span class='totalPages'></span>\
</div>";

pub fn convert_and_write(html_path: &PathBuf, options: &command_line::Options) -> Result<()> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let file_path = format!("file://{}", html_path.to_str().unwrap());

    // Browse to the file url and render a pdf of the web page.
    let mut pdf_options = PrintToPdfOptions::default();
    if options.title | options.date | options.numbering {
        pdf_options.display_header_footer = Some(true);

        let mut header = String::new();
        let mut footer = String::new();

        if options.title {
            header.push_str(TITLE_TEMPLATE);
        }

        if options.date {
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            header.push_str(&DATE_TEMPLATE.replace("%DATE%", &today));
        }

        if options.numbering {
            footer.push_str(NUMBERING_TEMPLATE);
        }

        pdf_options.header_template = Some(header);
        pdf_options.footer_template = Some(footer);
    }

    let local_pdf = tab
        .navigate_to(&file_path)?
        .wait_until_navigated()?
        .print_to_pdf(Some(pdf_options))?;

    fs::write(html_path.with_extension("pdf"), local_pdf)?;
    println!("PDF successfully created from local web page.");

    Ok(())
}
