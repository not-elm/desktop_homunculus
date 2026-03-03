use homunculus_http_server::create_openapi;
use std::path::PathBuf;

const DEFAULT_OUTPUT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../docs/website/static/api/open-api.yml"
);

fn print_usage() {
    eprintln!("Usage: gen_openapi [--output <path>]");
}

fn parse_output_path() -> PathBuf {
    let mut args = std::env::args().skip(1);
    let mut output_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let value = args.next().unwrap_or_else(|| {
                    eprintln!("Missing value for --output");
                    print_usage();
                    std::process::exit(2);
                });
                output_path = Some(PathBuf::from(value));
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unexpected argument: {arg}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    output_path.unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT_PATH))
}

fn main() {
    let output = parse_output_path();
    let api = create_openapi();
    let yaml = api
        .to_yaml()
        .expect("Failed to serialize OpenAPI spec to YAML");

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    std::fs::write(&output, &yaml).expect("Failed to write OpenAPI spec file");
    println!("OpenAPI spec written to {}", output.display());
}
