#[derive(Debug)]
pub struct Options {
    pub help: bool,
    pub folder: std::path::PathBuf,
    pub html: bool,
    pub pdf: bool,
    pub full: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            help: false,
            folder: std::path::PathBuf::from("."),
            html: false,
            pdf: true,
            full: false,
        }
    }
}
