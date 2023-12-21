use std::{fs, io, str};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, exit, ExitStatus};

use colour::*;
use rand::Rng;
use tar::Archive;
use version_compare::{Cmp, Version};
use xz2::read::XzDecoder;

fn repair_vpt() {}

pub fn self_test() -> i32 { // test that all the files that vpt needs are where they should be    
    if !(
        Path::new("/etc/vpt").exists() ||
            Path::new("/etc/vpt/vpt.conf").exists() ||
            Path::new("/var/lib/vpt").exists() ||
            Path::new("/var/lib/vpt/local").exists() ||
            Path::new("/var/lib/vpt/local/packages.db").exists() ||
            Path::new("/var/lib/vpt/packages.db").exists() ||
            Path::new("/tmp/vpt").exists()
    ) {
        return 999;
    }


    return 0; // all's good
}

pub fn check_option(option: &str) -> bool {
    let cfg = fs::read_to_string("/etc/vpt/vpt.conf").unwrap();
    let cfg = cfg.split_whitespace();

    let opt1 = format!("{}=true", option);
    let opt2 = format!("{}=false", option);

    for line in cfg {
        if line.eq(&opt1) || line.eq(&opt2) {
            let mut line = line.split('=');
            line.next();
            return line.next().unwrap().contains("true");
        }
    }

    return false;
}

pub fn compare_old_to_new(package: &str) -> bool {
    for i in 0..count_repos() {
        let path = format!("/var/lib/vpt/packages{}.db", i);

        if Path::new(&path).exists() {
            let mut db = sqlite::open(&path).unwrap();

            db.execute(
                "CREATE TABLE if not exists pkglist (name TEXT, version TEXT, description TEXT, files TEXT);"
            ).unwrap();

            let mut oldver = String::new();
            let mut newver = String::new();

            db.iterate("SELECT name, version FROM pkglist", |pairs| {
                let mut chkver = false;

                for &(column, value) in pairs.iter() {
                    if column == "name" && value.unwrap() == package {
                        chkver = true;
                    } else if column == "version" && chkver {
                        oldver = value.unwrap().to_string();
                        chkver = false;
                    }
                }
                true
            }).unwrap();

            let pkgdb = sqlite::open("/var/lib/vpt/local/packages.db").unwrap();

            pkgdb.execute(
                "CREATE TABLE if not exists packages (name TEXT, version TEXT, description TEXT, files TEXT);"
            ).unwrap();

            pkgdb.iterate("SELECT name, version FROM packages", |pairs| {
                let mut chkver = false;

                for &(column, value) in pairs.iter() {
                    if column == "name" && value.unwrap() == package {
                        chkver = true;
                    } else if column == "version" && chkver {
                        newver = value.unwrap().to_string();
                        chkver = false;
                    }
                }
                true
            }).unwrap();

            let oldver = Version::from(&oldver).unwrap();

            let newver = Version::from(&newver).unwrap();

            let mut is_newer = false;

            if oldver.compare(newver) == Cmp::Gt {
                is_newer = true;
            }

            return !is_newer; // return if db2's version is newer than db1's
        }
    }

    return false;
}

pub fn get_pkg_version(package: &str) -> String {
    for i in 0..count_repos() {
        let path = format!("/var/lib/vpt/packages{}.db", i);

        if Path::new(&path).exists() {
            let mut db = sqlite::open(&path).unwrap();

            db.execute(
                "CREATE TABLE if not exists pkglist (name TEXT, version TEXT, description TEXT, files TEXT);"
            ).unwrap();

            let mut ver = String::new();

            db.iterate("SELECT name, version FROM pkglist", |pairs| {
                let mut chkver = false;

                for &(column, value) in pairs.iter() {
                    if column == "name" && value.unwrap() == package {
                        chkver = true;
                    } else if column == "version" && chkver {
                        ver = value.unwrap().to_string();
                        chkver = false;
                    }
                }
                true
            }).unwrap();

            return ver;
        }
    }

    return "".to_string();
}

fn get_pkg_desc(package: &str) -> String {
    for i in 0..count_repos() {
        let path = format!("/var/lib/vpt/packages{}.db", i);

        if Path::new(&path).exists() {
            let mut db = sqlite::open(&path).unwrap();

            db.execute(
                "CREATE TABLE if not exists pkglist (name TEXT, version TEXT, description TEXT, files TEXT);"
            ).unwrap();

            let mut desc = String::new();

            db.iterate("SELECT name, description FROM pkglist", |pairs| {
                let mut chkver = false;

                for &(column, value) in pairs.iter() {
                    if column == "name" && value.unwrap() == package {
                        chkver = true;
                    } else if column == "description" && chkver {
                        desc = value.unwrap().to_string();
                        chkver = false;
                    }
                }
                true
            }).unwrap();

            return desc;
        }
    }

    return "".to_string();
}

pub fn add_pkg_to_db(package: &str, files: String) -> i32 {
    if !db_lock() { return 16; }; // check if database is locked

    let pkgdb = sqlite::open("/var/lib/vpt/local/packages.db").unwrap();

    pkgdb
        .execute("
        CREATE TABLE if not exists packages (name TEXT, version TEXT, description TEXT, files TEXT);
        ", ).unwrap();

    let ver = get_pkg_version(package);

    let desc = get_pkg_desc(package);


    let cmd = "INSERT INTO packages VALUES ('".to_owned()
        + package
        + "', '"
        + &*ver.to_string()
        + "', '"
        + &desc
        + "', '"
        + &files
        + "');";

    pkgdb.execute(cmd).unwrap();

    fs::remove_file("/var/lib/vpt/local/packages.db.lock").unwrap();

    return 0;
}

pub fn debug_add_pkg_to_pkglist(package: &str) {
    let pkgdb = sqlite::open("/var/lib/vpt/packages.db").unwrap();

    let ver = 99999;

    let desc = "Lorem ipsum.";

    let cmd = "INSERT INTO pkglist VALUES ('".to_owned()
        + package
        + "', '"
        + &*ver.to_string()
        + "', '"
        + desc
        + "', '"
        + "')";


    println!("{}", cmd);

    pkgdb.execute("CREATE TABLE if not exists pkglist (name TEXT, version TEXT, description TEXT, files TEXT);").unwrap();
    pkgdb.execute(cmd).unwrap();
}


fn db_lock() -> bool {
    if Path::new("/var/lib/vpt/local/packages.db.lock").exists() {
        return false; // return false if you can't lock database
    }

    let mut file = File::create("/var/lib/vpt/local/packages.db.lock").unwrap();
    file.write_all(b"locked").unwrap();
    return true; // return true if you can lock database
}

pub fn new_snapshot(snapshot_type: &str, snapshot_reason: &str) {
    let reason = snapshot_type.to_lowercase() + " " + snapshot_reason;
    Command::new("snapper")
        .arg("-c")
        .arg("root")
        .arg("create")
        .arg("--description")
        .arg(reason)
        .output()
        .expect("Couldn't take snapshot.");
}

pub fn test_xbps() -> bool {
    // check if xbps can be found in /usr/bin
    return Path::new("/usr/bin/xbps-install").exists();
}

pub fn search_package(pkg_name: &str) -> bool {
    return !get_pkg_version(pkg_name).is_empty(); // return if package is found (true if found, false if not
}

pub fn get_package(pkg: &str, cache: bool, location: &str, tarName: &str) -> ExitStatus {
    let link = {
        // check which link has the package
        let mut link = String::new();

        for i in get_repos() {
            // check if link exists
            if Command::new("curl")
                .arg(i.to_string() + "/pkg/" + pkg + ".tar.xz")
                .status()
                .expect("Error: Couldn't check if package exists.")
                .success()
            {
                link = i.to_string() + "/pkg/" + pkg + ".tar.xz";
                break;
            }
        }

        println!("Downloading package from {}", link);
    };

    // TODO: Check multiple repos for package
    let status = {
        Command::new("curl")
            .arg(&link)// the link that is figured out previously
            .arg("-o")
            .arg("/tmp/vpt/".to_owned() + tarName + ".tar.xz")// name of the tar file
            .status()
            .expect("Error: Couldn't download package.")
    };

    status
}

// install package with given arguments
pub fn install_tar(pkg: &str, root: &str, offline: bool, upgrade: bool, cli: bool) -> i32 {
    let pkglist = list_packages();

    let tmp = pkglist.split(' ');

    let all_packages: Vec<&str> = tmp.collect();

    let mut upgr_ok = false;

    for i in all_packages {
        if i == pkg && !upgrade {
            return 1; // already installed
        } else if i == pkg && upgrade {
            upgr_ok = true;
        }
    }

    if !upgr_ok && upgrade {
        return 2; // not installed
    }

    // return i32 for error codes; 0 - good
    if !root.is_empty() && !Path::new(root).exists() {
        return 404; // return 404 if root directory doesn't exist
    } else if !root.is_empty() {
        // TODO: add ability to install to a different root directory
    }

    let tarName = assign_random_name();

    let dir_name = assign_random_name();

    let temp_dir = "/tmp/vpt/".to_owned() + &dir_name;

    if !offline {
        // offline install tries to install the package off the disk
        get_package(pkg, true, "", &tarName);
    } else {}

    let tar_file = "/tmp/vpt/".to_owned() + &tarName + ".tar.xz";

    fs::create_dir_all(&temp_dir).expect("Couldn't create temp directory.");

    let path = tar_file.as_str();
    let tar_gz = File::open(path).unwrap();
    let tar = XzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&temp_dir).unwrap();

    let usr_binpath = "/tmp/vpt/".to_owned() + &dir_name + "/BINARIES" + "/usr-bin";

    let bin_binpath = "/tmp/vpt/".to_owned() + &dir_name + "/BINARIES" + "/bin";

    let manpath = "/tmp/vpt/".to_owned() + &dir_name + "/MANUALS";

    let etcpath = "/tmp/vpt/".to_owned() + &dir_name + "/CONFIGS" + "/etc";

    let usr_sharepath = "/tmp/vpt/".to_owned() + &dir_name + "/CONFIGS" + "/usr-share";

    let bootpath = "/tmp/vpt/".to_owned() + &dir_name + "/BOOT";

    let libpath = "/tmp/vpt/".to_owned() + &dir_name + "/LIBRARIES" + "/lib";

    let lib64path = "/tmp/vpt/".to_owned() + &dir_name + "/LIBRARIES" + "/lib64";

    let varlibpath = "/tmp/vpt".to_owned() + &dir_name + "/LIBRARIES" + "/var";

    let mut files = String::new();

    if Path::new(&usr_binpath).is_dir() {
        for binary in fs::read_dir(&usr_binpath).unwrap() {
            let binary = binary.unwrap().path();
            let binary = binary.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/BINARIES" + "/usr-bin/";

            let destination = root.to_owned() + &"/usr/bin/".to_owned() + &*binary.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(binary, destination).unwrap();
        }
    }

    if Path::new(&bin_binpath).is_dir() {
        for binary in fs::read_dir(&bin_binpath).unwrap() {
            let binary = binary.unwrap().path();
            let binary = binary.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/BINARIES" + "/bin/";

            let destination = root.to_owned() + &"/bin/".to_owned() + &*binary.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(binary, destination).unwrap();
        }
    }

    if Path::new(&manpath).is_dir() {
        for manual in fs::read_dir(&manpath).unwrap() {
            let manual = manual.unwrap().path();
            let manual = manual.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/MANUALS";

            let destination = root.to_owned() + &"/usr/share/man/man1/".to_owned() + &*manual.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(manual, destination).unwrap();
        }
    }

    if Path::new(&etcpath).is_dir() {
        for cfg in fs::read_dir(&etcpath).unwrap() {
            let cfg = cfg.unwrap().path();
            let cfg = cfg.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/CONFIGS" + "/etc";

            let installed_path = root.to_owned() + &"/etc/".to_owned() + &*cfg.replace(&real_path, "");

            files = files + &installed_path + " ";

            if Path::new(&installed_path).exists() {
                if resolve_conflict(&installed_path, cli) == 2 {
                    red_ln!("Error: File Conflict: Cannot continue");
                    exit(128);
                } else {
                    fs::copy(cfg, installed_path).unwrap();
                }
            }
        }
    }

    if Path::new(&usr_sharepath).is_dir() {
        for cfg in fs::read_dir(&etcpath).unwrap() {
            let cfg = cfg.unwrap().path();
            let cfg = cfg.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/CONFIGS" + "/usr-share";

            let destination = root.to_owned() + &"/usr/share/".to_owned() + &*cfg.replace(&real_path, "");

            files = files + &destination + " ";

            if Path::new(&destination).exists() {
                if resolve_conflict(&destination, cli) == 2 {
                    red_ln!("Error: File Conflict: Cannot continue");
                    exit(128);
                } else {
                    fs::copy(cfg, destination).unwrap();
                }
            }
        }
    }

    if Path::new(&bootpath).is_dir() {
        for file in fs::read_dir(&bootpath).unwrap() {
            let file = file.unwrap().path();
            let file = file.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/BOOT";

            let destination = root.to_owned() + &"/boot/".to_owned() + &*file.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(file, destination).unwrap();
        }
    }

    if Path::new(&libpath).is_dir() {
        for lib in fs::read_dir(&libpath).unwrap() {
            let lib = lib.unwrap().path();
            let lib = lib.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/LIBRARIES" + "/lib";

            let destination = root.to_owned() + &"/usr/lib/".to_owned() + &*lib.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(lib, destination).unwrap();
        }
    }

    if Path::new(&lib64path).is_dir() {
        for lib64 in fs::read_dir(&lib64path).unwrap() {
            let lib64 = lib64.unwrap().path();
            let lib64 = lib64.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/LIBRARIES" + "/lib64";

            let destination = root.to_owned() + &"/usr/lib64/".to_owned() + &*lib64.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(lib64, destination).unwrap();
        }
    }

    if Path::new(&varlibpath).is_dir() {
        for lib in fs::read_dir(&varlibpath).unwrap() {
            let lib = lib.unwrap().path();
            let lib = lib.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/LIBRARIES/" + "var";

            let destination = root.to_owned() + &"/var/lib/".to_owned() + &*lib.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(lib, destination).unwrap();
        }
    }

    if !upgrade {
        if add_pkg_to_db(pkg, files) == 16 {
            return 16; // return 16 if database is locked
        }
    } else {
        let pkgdb = sqlite::open("/var/lib/vpt/local/packages.db").unwrap();
        pkgdb
            .execute("
            CREATE TABLE if not exists packages (name TEXT, version TEXT, description TEXT, files TEXT);
            ", ).expect("Couldn't create table.");

        let ver = get_pkg_version(pkg);

        let desc = get_pkg_desc(pkg);


        let cmd = "UPDATE packages".to_owned()
            + " SET version = '"
            + &*ver.to_string()
            + "', description = '"
            + &desc
            + "' WHERE name = '"
            + pkg
            + "';";

        if !db_lock() {
            return 16; // return 16 if database is locked
        }

        pkgdb.execute(cmd).unwrap();
    }


    return 0;
}

// Generate a random name.
fn assign_random_name() -> String {
    // The charset we want to use for the name.
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    // Initialize the random number generator.
    let mut rng = rand::thread_rng();

    // Create the name.
    let name: String = (0..10).map(|_| {
        // Choose a random index into the charset.
        let idx = rng.gen_range(0..CHARSET.len());
        // Convert the byte to a character.
        CHARSET[idx] as char
    }).collect();

    // Return the name.
    return name;
}

pub fn remove_tar(pkg: &str) -> i32 {
    let db = sqlite::open("/var/lib/vpt/local/packages.db").unwrap();

    db.execute(
        "CREATE TABLE if not exists packages (name TEXT, version TEXT, description TEXT, files TEXT);"
    ).unwrap();

    let mut files = String::new();

    db.iterate("SELECT name, files FROM packages", |pairs| {
        let mut check = false;

        for &(column, value) in pairs.iter() {
            if column == "name" && value.unwrap() == pkg {
                check = true;
            } else if column == "files" && check {
                files = value.unwrap().to_string();
                check = false;
            }
        }

        true
    }).unwrap();

    let tmp = files.split(' ');

    let mut all_files: Vec<_> = tmp.collect();

    for i in all_files.iter() {
        fs::remove_file(i);
    }

    if db_lock() {
        let cmd = "DELETE FROM packages WHERE name = '".to_owned() + pkg + "';";
        db.execute(cmd).unwrap();
        fs::remove_file("/var/lib/vpt/local/packages.db.lock");
    } else {
        println!("Couldn't lock database.");
        return 1;
    }

    return 0;
}

pub fn get_repos() -> Vec<i32> {
    let repos = fs::read_to_string("/etc/vpt/vpt.conf").unwrap();
    let repos = repos.split("repos = {").collect::<Vec<&str>>()[1];
    let repos = repos.split("}").collect::<Vec<&str>>()[0];
    let mut repos = repos.split('\n').collect::<Vec<&str>>();

    repos.remove(0);
    repos.remove(repos.len() - 1);

    return repos;
}

pub fn count_repos() -> i32 {
    let repos = fs::read_to_string("/etc/vpt/vpt.conf").unwrap();
    let repos = repos.split("repos = {").collect::<Vec<&str>>()[1];
    let repos = repos.split("}").collect::<Vec<&str>>()[0];
    let mut repos = repos.split('\n').collect::<Vec<&str>>();

    repos.remove(0);
    repos.remove(repos.len() - 1);

    return repos.len() as i32;
}

pub fn download_pkglist() -> i32 { // print error code
    // get all repos after that is after "repos =" and in between {} and split them into links within a vector
    let repos = fs::read_to_string("/etc/vpt/vpt.conf").unwrap();
    let repos = repos.split("repos = {").collect::<Vec<&str>>()[1];
    let repos = repos.split("}").collect::<Vec<&str>>()[0];
    let mut repos = repos.split('\n').collect::<Vec<&str>>();

    repos.remove(0);
    repos.remove(repos.len() - 1);

    for i in 0..repos.len() {
        repos[i] = match repos[i].find("https") {
            Some(index) => &repos[i][index..],
            None => repos[i],
        };
    }


    for i in 0..repos.len() {
        println!("{0}: {1}", i, repos[i]);
    }


    for i in 0..repos.len() {
        println!("Downloading package list from {}", repos[i]);

        let output_path = format!("/var/lib/vpt/packages{}.db", i);

        let status = Command::new("/usr/bin/curl")
            .arg(repos[i].to_owned().replace('"', "") + "/pkglist")
            .arg("-o")
            .arg(&output_path)
            .status()
            .expect("Failed to execute command");
    }


    // TODO: add error checking
    return 0;
}

pub fn list_packages() -> String {
    for i in 0..count_repos() {
        let path = format!("/var/lib/vpt/packages{}.db", i);

        if Path::new(&path).exists() {
            let mut db = sqlite::open(&path).unwrap();

            db.execute(
                "CREATE TABLE if not exists pkglist (name TEXT, version TEXT, description TEXT, files TEXT);"
            ).unwrap();

            let mut packages = String::new();

            db.iterate("SELECT name FROM pkglist", |pairs| {
                for &(column, value) in pairs.iter() {
                    if column == "name" {
                        packages = packages.to_owned() + " " + &value.unwrap().to_string();
                    }
                }
                true
            }).unwrap();

            return packages;
        }
    }
}

fn resolve_conflict(conflict: &str, cli: bool) -> i32 {
    if cli {
        fs::remove_file(conflict).unwrap();
        return 0;
    }

    println!("File {} already exists", conflict);
    println!("1) Overwrite file");
    println!("2) Skip file");
    println!("3) Do nothing and abort");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let choice: i32 = choice.trim().parse().unwrap();

    return if choice == 1 {
        fs::remove_file(conflict).unwrap();
        0
    } else if choice == 2 {
        1
    } else if choice == 3 {
        2
    } else {
        println!("Invalid input");
        3
    };
}