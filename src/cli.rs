use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "minigrep")]
#[command(about = "Search patterns within files")]
pub struct Arguments {
    pub pattern: String,

    #[arg(required = true)]
    pub paths: Vec<String>,

    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    #[arg(short = 'r', long = "recursive")]
    pub recursive: bool,

    #[arg(short = 'c', long = "count")]
    pub count: bool,

    #[arg(short = 'l', long = "show-line", default_value_t = true)]
    pub show_line_number: bool,

    #[arg(short = 'v', long = "invert-match", default_value_t = false)]
    pub invert_match: bool,

    #[arg(short = 'A', long = "after-context")]
    pub after_context: Option<usize>,

    #[arg(short = 'B', long = "before-context")]
    pub before_contex: Option<usize>,

    #[arg(short = 'C', long = "context")]
    pub contex: Option<usize>,
}
