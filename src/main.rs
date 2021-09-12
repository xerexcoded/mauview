use clap::AppSettings::ArgRequiredElseHelp;
use clap::{crate_authors,crate_description,crate_name,crate_version,App,Arg};
mod app;
mod config;
use config::Config;

fn main(){
    let matches =App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(ArgRequiredElseHelp)
        .usage("viu [FLAGS] [OPTIONS] [FILE]... 
               when file is - , read standard input.",)
        .arg(Arg::with_name("FILE")
             .help("The image to be displayed")
            .multiple(true)
         )
        .arg(Arg::with_name("name")
             .short("n")
             .long("mirror")
             .help("Display a mirror of the original image"),
             )
        .arg(
            Arg::with_name("once")
            .short("t")
            .long("transparent")
            .help("Display transparent image with transparent background"),

            )

           .arg(
               Arg::with_name("static")
               .short("s")
               .long("static")
               .help("show only first frame of gif"),

               )
           .arg(
               Arg::with_name("width")
               .short("w")
               .long("width")
               .takes_value(true)
               .help("Resize the image to a provided width"),

               )
           .arg(
               Arg::with_name("height")
               .short("h")
               .long("height")
               .takes_value(true)
               .help("Resize the image to a provided height"),
               )
           .arg(
               Arg::with_name("recursive")
               .short("r")
               .long("recusive")
               .help("Recurse dowm directories if passed one"),
               )
           .arg(
               Arg::with_name("frames-per-second")
               .short("f")
               .long("frame-rate")
               .takes_value(true)
               .help("Play gif at the given frame rate")
               ,
               )
           .arg(
               Arg::with_name("blocks")
               .short("b")
               .long("blocks")
               .takes_value(false)
               .help("Force block output"),
               )
           .get_matches();
    let conf =Config::new(&matches);
    if let Err(e) = app::run(conf){
        eprintln!("{:?}",e);
        std::process::exit(1);
    }

}



