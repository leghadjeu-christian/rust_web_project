use reqwest::blocking::multipart;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(parse(from_os_str))]
    infile: Vec<PathBuf>,

    // #[structopt(short = "l", long = "level")]
    // level_option: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();
    println!("{:?}", opts);
    for file_path in opts.infile {
        let file = file_path.to_str().unwrap();
        println!("{}", file);

        let form = multipart::Form::new().file(format!("{}", file), format!("{}", file))?;

        let client = reqwest::blocking::Client::new();
        println!("{:#?}", form);
        let resp = client
            .post("http://localhost:8000/uploads/upload")
            .multipart(form)
            .send()?;
        println!("{:?}", resp);
    }
    Ok(())
}
