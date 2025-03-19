use reqwest::blocking::multipart;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(parse(from_os_str))]
    infile: PathBuf,

    // #[structopt(parse(from_os_str))]
    // outfile: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();
    let first = opts.infile.to_str().unwrap();
    println!("{}", first);

    println!("{:?}", opts);

    let form = multipart::Form::new()
        // And a file...
        .file("to-d", format!("{}", first))?;

    // // Customize all the details of a Part if needed...
    // let bio = multipart::Part::text("hallo peeps")
    //     .file_name("something")
    //     .mime_str("text/plain")?;

    // Add the custom part to our form...
    // let form = form.part("biography", bio);

    // And finally, send the form
    let client = reqwest::blocking::Client::new();
    println!("{:#?}", form);
    let resp = client
        .post("http://localhost:3000")
        .multipart(form)
        .send()?;
    println!("{:?}", resp);
    Ok(())
}
