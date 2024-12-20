#[derive(Debug)]
pub struct Options {
    pub help: bool,
    pub tricks: bool,
    pub version: bool,
    pub folder: std::path::PathBuf,
    pub html: bool,
    pub pdf: bool,
    pub title: bool,
    pub date: bool,
    pub numbering: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            help: false,
            tricks: false,
            version: false,
            folder: std::path::PathBuf::from("."),
            html: false,
            pdf: true,
            title: false,
            date: false,
            numbering: false,
        }
    }
}

pub fn parse() -> Result<Options, String> {
    let mut options = Options::default();
    let mut args = std::env::args().skip(1); // Skip the program name
    let mut folder_provided = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-t" | "--title" => options.title = true,
            "-d" | "--date" => options.date = true,
            "-n" | "--numbering" => options.numbering = true,
            "-f" | "--full" => {
                options.title = true;
                options.date = true;
                options.numbering = true;
            }
            "-m" | "--html" => options.html = true,
            "-x" | "--no-pdf" => options.pdf = false,
            "-h" | "--help" => {
                options.help = true;
            }
            "-r" | "--tricks" => options.tricks = true,
            "-v" | "--version" => options.version = true,
            _ => {
                if !arg.starts_with('-') {
                    options.folder = std::path::PathBuf::from(arg);
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

pub fn print_help() {
    println!(
        "\
Usage: markdowncuments [folder] [options]

Options:
    -t, --title          Add title in header
    -d, --date           Add date in header
    -n, --numbering      Add page numbering in footer
    -f, --full           Add title, date and numbering in header and footer
    -n, --numbering      Use full template with header and footer
    -m, --html           Output in HTML format also
    -x, --no-pdf         Disable PDF output
    -h, --help           Display this help message
    -r, --tricks         Show some tricks
    -v, --version        Just show program version and exit
"
    );
}

pub fn print_tricks() {
    println!(
        "\
Tricks for .md files:
    
    <div class=\"page\"></div>
        Inserts a page break.
    
    <script>document.title=Anything_You_Like</script>
        Changes the document title (in header) to Anything_You_Like.
"
    );
}
