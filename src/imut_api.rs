use std::process::Command; // for executing commands
use lazy_static::lazy_static;

lazy_static! {
    static ref PATHS : Vec<&'static str> = vec!["/usr", "/lib", "/sbin", "/bin"];
}

fn check_fs(path: String) -> bool {
    if path == "/usr" {
        let tmp = Command::new("lsattr").arg("/usr")
            .output()
            .expect("failed to execute process");

        let tmp = String::from_utf8_lossy(&tmp.stdout);
        let tmp = tmp.trim();
        let tmp = tmp.split_whitespace().collect::<Vec<&str>>();
        let tmp = tmp[0];
        let tmp = tmp.to_string();

        return tmp.contains("i");
    }

    let true_path = Command::new("ls").arg("-l").arg(path).output().unwrap();
    let true_path = String::from_utf8_lossy(&true_path.stdout);
    let true_path = true_path.trim();
    let true_path = true_path.split_whitespace().collect::<Vec<&str>>();
    let true_path = true_path[10];
    let true_path = true_path.to_string();

    let true_path = "/".to_owned() + &true_path;

    let tmp = Command::new("lsattr").arg(true_path)
        .output()
        .expect("failed to execute process");

    let tmp = String::from_utf8_lossy(&tmp.stdout);
    let tmp = tmp.trim();
    let tmp = tmp.split_whitespace().collect::<Vec<&str>>();
    let tmp = tmp[0];
    let tmp = tmp.to_string();

    return tmp.contains("i");
}

pub(crate) fn getmode() -> bool {
    for path in PATHS.iter() {
        if !check_fs(path.to_string()) {
            return false;
        }
    }

    return true;
}



pub(crate) fn enterro() -> i32 {
    for path in PATHS.iter() {
        Command::new("chattr").arg("+i").arg("-R").arg(path)
            .output()
            .expect("failed to execute process");
    }

    return 0;
}

pub(crate) fn enterrw() -> i32 {
    for path in PATHS.iter() {
        Command::new("chattr").arg("-i").arg("-R").arg(path)
            .output()
            .expect("failed to execute process");
    }

    return 0;
}