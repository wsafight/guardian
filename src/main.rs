use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let action = &args[1];

    if action == "run" && action != "monit" {
        println!("The program currently only supports [run],[monit]");
        return;
    }

    if action == "run" {
        return;        
    }

    if action == "monit" {
        return;
    }

}