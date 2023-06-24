fn ignore_element(ce: &str, args: &Vec<String>) -> bool {
    if args.contains(&ce.to_string()) {
        return true;
    } else {
        return false;
    }
}

pub fn list_dir(dir_name: &String, args: &Vec<String>) -> String {
    let mut result = String::new();

    for path in std::fs::read_dir(dir_name).unwrap() {
        let name = path.as_ref().unwrap().file_name();
        let name = name.to_str().unwrap();

        if ignore_element(name, args) {
            continue;
        };

        if path.as_ref().unwrap().path().is_dir() {
            let fname = dir_name.to_string() + "/" + name + "/";

            // Ignores the git dir as it is, in most cases, not to be seen by the user
            if name == "./.git/" { 
                continue;
            }
            
            // Lists the sub-dir (current element)
            let listed_subdir = list_dir(&fname, args);

            // Add the space to show that we're in a subdir
            result += "  ";
            result += &listed_subdir;
        } else {
            result += &format!("  ↳  {}\n", name);
        }
    }

    return result;
}
