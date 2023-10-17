mod options;

fn main() {
    let args = std::env::args();
    
    let config = options::Options::new(args);

    println!("config = {:?}", config);
}
