pub struct CliOptions {
    pub example: bool,
}

impl CliOptions {
    pub fn from_args(args: &[String]) -> Self {
        let example = args.contains(&"--example".to_string());
        Self { example }
    }
}