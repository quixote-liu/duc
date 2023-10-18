mod cmd;

fn main() {
    let args = std::env::args();

    let cmd_stance = cmd::Command::from_args(args);
    println!("{:?}", cmd_stance);
    
    // cmd::Command::from_args(args).run();
}
