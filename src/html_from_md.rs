use comrak::{markdown_to_html, Options};

pub fn convert(title: &str, content: String) -> String {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.render.unsafe_ = true;
    options.extension.tasklist = true;
    options.extension.table = true;
    options.render.github_pre_lang = true;
    let md_to_html = markdown_to_html(&content, &options);

    let mut output = include_str!("html_wrapper/wrapper.html").to_string();
    output = output.replace("/*TITLE*/", &title);
    output = output.replace("/*STYLE*/", include_str!("html_wrapper/style.css"));
    output = output.replace("/*BODY*/", &md_to_html);

    output
}
