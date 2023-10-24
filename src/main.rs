mod cmd;

fn main() {
    let args = std::env::args();

    cmd::Command::from_args(args).run();
}
