mod flags;

fn main() {
    let args: flags::Arguments = match flags::Arguments::new() {
        Ok(args) => args,
        Err(err) => panic!("Invalid arguments:\n[ERROR] {}", err.message()),
    };

    println!("Arguments: {:?}", args)
}
