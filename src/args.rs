use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(about = "UCI manipulation")]
pub struct CLIargs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// print the list of configuration files available on the system
    ListConfigFiles,
    /// print the whole selected configuration file
    PrintConfigFile { file_path: String },
    /// print the value of your desired section;
    PrintSection { section: String},
    /// create a new section/option in configuration file with different types
    SetSection { section: String, value: String},
    /// delete a section/option in the configuration file;
    DeleteSection { section: String }
}
