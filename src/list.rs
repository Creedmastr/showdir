use std::path::Path;

pub fn list_dir(dir_name: &String) -> String {
    let mut result = String::new();

    for path in std::fs::read_dir(dir_name).unwrap() {
        let name = path.as_ref().unwrap().file_name();

        if path.as_ref().unwrap().path().is_dir() {
            let mut fname = dir_name.to_string();
            fname.push_str("/");
            fname.push_str(&name.to_str().unwrap());
            fname.push_str("/");

            if name == "./.git/" {
                continue;
            }

            let listed_subdir = list_dir(&fname);

            result.push_str("  ");
            result.push_str(&listed_subdir);
        } else {
            result.push_str(&format!("  ↳  {}\n", name.to_str().unwrap()));
        }
    }

    return result;
}
