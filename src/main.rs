#[macro_use] extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jabir", about = "Chemical elements analyser")]
enum Jabir {
    #[structopt(name = "show", about = "Show element")]
    Show {
        element: String,
    }
}

fn main() {
    let matches = Jabir::from_args();

    println!("{:?}", matches);
    
    match matches {
        Jabir::Show { element }  => {
            println!("Element is {}", element);
        }
    }
}
