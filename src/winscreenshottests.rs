use super::*;

#[test]
fn main_creates_path_parameter_in_cli() {
    let cli = Cli::parse();
    assert_eq!(cli.path, None);
}
