mod descriptions;
use descriptions::descriptions;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

macro_rules! pub_struct {
    ($(#[$struct_attr:meta])* struct $name:ident {$($(#[$field_attr:meta])* $field:ident: $t:ty,)*}) => {
        $(#[$struct_attr])*
        pub struct $name {
            $($(#[$field_attr])*
            pub $field: $t,)*
        }
    }
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: LegOffSubcommands,
}

#[derive(Subcommand)]
pub enum LegOffSubcommands {
    #[command(about = "Create new project or create to an existing project")]
    New(NewArgs),

    #[command(about = "Install libraries via conan", long_about = descriptions("install"))]
    Install(InstallArgs),

    #[command(about = "Add {source, header, section, module} to a {section, module}")]
    Add(AddArgs),

    #[command(about = "Delete {source, header, section, module} from a {section, module}")]
    Delete(DeleteArgs),

    #[command(about = "Set a variable to some value")]
    Set {},

    #[command(about = "Add some optional features to your porject")]
    Init {},

    #[command(about = "Build all {modules, files} and the main.{c, cpp} file and run it")]
    Run {},

    #[command(about = "Build all {modules, files} and the main.{c, cpp} file if it dose exist")]
    Build {},
}

#[derive(Clone, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum Lang {
    C,
    Cpp,
}

#[derive(Clone, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum Type {
    App,
    SharedLib,
    StaticLib,
    IncludeLib,
}

pub_struct!(
    #[derive(Args)]
    struct NewArgs {
        #[arg(long, value_enum, help = "Project language")]
        lang: Lang,

        #[arg(long, value_enum, help = "Project type")]
        r#type: Type,

        #[arg(short, long, help = "Project name")]
        name: String,

        #[arg(short = 'G', help = "Build system generator for cmake")]
        g: Option<String>,

        #[arg(long, default_value = ".", help = "Directory for the project")]
        to: PathBuf,

        #[arg(long, help = "Specify a unit testing framework add tests to your project and enable testing")]
        test: bool,

        #[arg(long, help = "Add conanfile.py to be able to install libraries via conan")]
        conan: bool,
    }
);

pub_struct!(
    #[derive(Args)]
    struct InstallArgs {
        #[arg(short, long, help = "Library name")]
        name: String,

        #[arg(short, long, help = "Library version")]
        version: String,
    }
);

pub_struct!(
    #[derive(Args)]
    struct AddArgs {
        #[arg(short, long, help = "File or module name")]
        name: String,

        #[arg(long, default_value = ".", help = "Module(path) for the file or module to be created to")]
        to: PathBuf,

        #[arg(short, long, help = "Create a new file or module or replace the old one without asking")]
        force: bool,
    }
);

pub_struct!(
    #[derive(Args)]
    struct DeleteArgs {
        #[arg(short, long, help = "File or module name")]
        name: String,

        #[arg(long, default_value = ".", help = "Module(path) for the file or module to be deleted from")]
        from: PathBuf,

        #[arg(short, long, help = "Delete a file or module without asking")]
        force: bool,
    }
);