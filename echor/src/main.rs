use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Reads from Cargo

struct Cli {

    //blabla
    #[arg(short = 'n', required = false, help = "Omit new line")]
    omit_newline: bool,

    #[arg(required = true, help = "Output text")]
    text: Vec<String>
}

fn main() {
    let cli = Cli::parse();

    let omit_newline = cli.omit_newline;
    let text = cli.text;

    print!("{}{}", text.join(" "), if omit_newline {""} else {"\n"});
}
