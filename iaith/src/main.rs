use iaith::brainf::Program;
use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    args.next();

    let mut prog = match args.next() {
        Some(p) => Program::new(&p),
        None => {
            eprintln!("You must specify a program.");
            process::exit(1);
        }
    };

    let output = prog.execute();
    println!("{}", output);
}
