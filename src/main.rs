use list::list_dir;

mod list;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("No arguments specified!");
    }

    let dir_name = &args[1];

    println!("{}", dir_name);

    let listed_dir = list_dir(dir_name);

    println!("{}", listed_dir);
}
