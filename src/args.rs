use clap::*;


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ProgramArgs{
    /// input
    pub input: String,

    /// output file
    pub output_file: String,
}
