use minigrep_clap::parse_cli;

fn main() {
    let cli = parse_cli();
    let file = &cli.name_file.clone();
    let query = &cli.query.clone();

    cli.run(&file, &query);
}
