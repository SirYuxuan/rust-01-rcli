use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "rcli", version = "1.0", author = "Rust CLI",long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[clap(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

/**
 * Verify if the input file exists
 */
fn verify_input_file(file_name: &str) -> Result<String, String> {
    if std::path::Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist".to_string())
    }
}
