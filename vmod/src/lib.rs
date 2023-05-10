use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::{fs, io, str};
use version_compare::{Cmp, Version};
use colour::*;
use rand::Rng;
use std::process::{exit, Command, ExitStatus};
use tar::Archive;
use xz2::read::XzDecoder;

fn repair_vpt() {

}

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
    // restore databases from backup using sqlite
    let mut db1 = sqlite::open("/var/lib/vpt/local/packages.db").unwrap();
    let mut db2 = sqlite::open("/var/lib/vpt/packages.db").unwrap();

    // create table packages if it doesn't exist
    db1.execute(
        "
        CREATE TABLE if not exists packages (name TEXT, version INTEGER, description TEXT, files TEXT);
        ",
    ).unwrap(); // make sure table exists

    // create other table pkglist if it doesn't exist
    db2.execute(
        "
        CREATE TABLE if not exists pkglist (name TEXT, version INTEGER, description TEXT, files TEXT);
        ",
    ).unwrap(); // make sure table exists

    let mut oldver = String::new();
    let mut newver = String::new();


    db1.iterate("SELECT name, version FROM packages", |pairs| {
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
    })
        .unwrap();

    db2.iterate("SELECT name, version FROM pkglist", |pairs| {
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
    })
        .unwrap();

    let oldver = Version::from(&oldver).unwrap();

    let newver = Version::from(&newver).unwrap();

    let mut is_newer = false;

    if oldver.compare(newver) == Cmp::Gt {
        is_newer = true;
    }

    return !is_newer; // return if db2's version is newer than db1's
}

pub fn get_pkg_version(package: &str) -> String {
    let mut db = sqlite::open("/var/lib/vpt/packages.db").unwrap();

    db.execute(
        "
        CREATE TABLE if not exists pkglist (name TEXT, version INTEGER, description TEXT, files TEXT);
        ",
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
    })
        .unwrap();

    return ver;
}

fn get_pkg_desc(package: &str) -> String {
    let mut db = sqlite::open("/var/lib/vpt/packages.db").unwrap();

    db.execute(
        "
        CREATE TABLE if not exists pkglist (name TEXT, version INTEGER, description TEXT, files TEXT);
        ",
    ).unwrap();

    let mut description = String::new();

    db.iterate("SELECT name, description FROM pkglist", |pairs| {
        let mut get_desc = false;

        for &(column, value) in pairs.iter() {
            if column == "name" && value.unwrap() == package {
                get_desc = true;
            } else if column == "description" && get_desc {
                description = value.unwrap().to_string();
                get_desc = false;
            }
        }
        true
    })
        .unwrap();

    description
}

pub fn add_pkg_to_db(package: &str, files: String) {
    if !db_lock() { return; }; // check if database is locked

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
}

pub fn debug_add_pkg_to_pkglist(package: &str) {
    let pkgdb = sqlite::open("/var/lib/vpt/packages.db").unwrap();

    let ver = 99999;

    let desc = "This is a test description, and this is not a real package, instead it is a fake package for testing purposes.";

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
        println!("Couldn't acquire lock on database");
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
    // check if xbps can be found on the OS
    // (mostly for usage on VF OS or
    // non Vefjiaw Linux distributions)
    return Path::new("/usr/bin/xbps-install").exists();
}

pub fn search_package(pkg_name: &str) -> bool {
    return !get_pkg_version(pkg_name).is_empty(); // return if package is found (true if found, false if not
}

pub fn get_package(pkg: &str, cache: bool, location: &str, tarName: &str) -> ExitStatus {
    let link = "https://raw.githubusercontent.com/vefjiaw/repo/main/".to_owned() + pkg + ".tar.xz"; // add link searching
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

pub fn install_tar(pkg: &str, root: &str, offline: bool, upgrade: bool) -> i32 {
    let pkglist = list_packages();

    let tmp = pkglist.split(' ');

    let all_packages: Vec<&str> = tmp.collect();

    let mut upgr_ok = false;

    for i in all_packages {
        if i == pkg && !upgrade {
            red_ln!("Error: Package {} is already installed.", pkg);
            return 1;
        } else if i == pkg && upgrade {
            upgr_ok = true;
        }
    }

    if !upgr_ok && upgrade {
        red_ln!("Error: Package {} is not installed.", pkg);
        return 1;
    }

    // return i32 for error codes; 0 - good
    if !root.is_empty() && !Path::new(root).exists() {
        red_ln!("Error: Cannot install to: {}: No such directory.", root);
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

            let destination = "/usr/bin/".to_owned() + &*binary.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(binary, destination).unwrap();
        }
    }

    if Path::new(&bin_binpath).is_dir() {
        for binary in fs::read_dir(&bin_binpath).unwrap() {
            let binary = binary.unwrap().path();
            let binary = binary.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/BINARIES" + "/bin/";

            let destination = "/bin/".to_owned() + &*binary.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(binary, destination).unwrap();
        }
    }

    if Path::new(&manpath).is_dir() {
        for manual in fs::read_dir(&manpath).unwrap() {
            let manual = manual.unwrap().path();
            let manual = manual.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/MANUALS";

            let destination = "/usr/share/man/man1/".to_owned() + &*manual.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(manual, destination).unwrap();
        }
    }

    if Path::new(&etcpath).is_dir() {
        for cfg in fs::read_dir(&etcpath).unwrap() {
            let cfg = cfg.unwrap().path();
            let cfg = cfg.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/CONFIGS" + "/etc";

            let installed_path = "/etc/".to_owned() + &*cfg.replace(&real_path, "");

            files = files + &installed_path + " ";

            if Path::new(&installed_path).exists() {
                if resolve_conflict(&installed_path) == 2 {
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

            let destination = "/usr/share/".to_owned() + &*cfg.replace(&real_path, "");

            files = files + &destination + " ";

            if Path::new(&destination).exists() {
                if resolve_conflict(&destination) == 2 {
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

            let destination = "/boot/".to_owned() + &*file.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(file, destination).unwrap();
        }
    }

    if Path::new(&libpath).is_dir() {
        for lib in fs::read_dir(&libpath).unwrap() {
            let lib = lib.unwrap().path();
            let lib = lib.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/LIBRARIES" + "/lib";

            let destination = "/usr/lib/".to_owned() + &*lib.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(lib, destination).unwrap();
        }
    }

    if Path::new(&lib64path).is_dir() {
        for lib64 in fs::read_dir(&lib64path).unwrap() {
            let lib64 = lib64.unwrap().path();
            let lib64 = lib64.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/LIBRARIES" + "/lib64";

            let destination = "/usr/lib64/".to_owned() + &*lib64.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(lib64, destination).unwrap();
        }
    }

    if Path::new(&varlibpath).is_dir() {
        for lib in fs::read_dir(&varlibpath).unwrap() {
            let lib = lib.unwrap().path();
            let lib = lib.to_str().unwrap();

            let real_path = "/tmp/vpt/".to_owned() + &dir_name + "/LIBRARIES/" + "var";

            let destination = "/var/lib/".to_owned() + &*lib.replace(&real_path, "");

            files = files + &destination + " ";

            fs::copy(lib, destination).unwrap();
        }
    }

    if !upgrade {
        add_pkg_to_db(pkg, files);
    } else {
        let pkgdb = sqlite::open("/var/lib/vpt/local/packages.db").unwrap();
        pkgdb
            .execute("
            CREATE TABLE if not exists packages (name TEXT, version TEXT, description TEXT, files TEXT);
            ", );

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

        pkgdb.execute(cmd).unwrap();
    }

    return 0;
}

fn unattended_conflict_solver(conflict: &str, solution: &str) -> i32 { // for use with alternative VPT frontends
    if solution == "overwrite" {
        fs::remove_file(conflict).unwrap();
        return 0; // conflict solved
    } else if solution == "skip" {
        return 1; // conflict solved (might cause issues)
    } else if solution == "abort" {
        return 2; // conflict not solved and abort
    } else {
        return 999; // programmer error
    }
}

fn resolve_conflict(conflict: &str) -> i32 {
    println!("File {} already exists", conflict);
    println!("1) Overwrite file");
    println!("2) Skip file");
    println!("3) Do nothing and abort");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let choice: i32 = choice.trim().parse().unwrap();

    if choice == 1 {
        fs::remove_file(conflict).unwrap();
        return 0;
    } else if choice == 2 {
        return 1;
    } else if choice == 3 {
        return 2;
    } else {
        println!("Invalid input");
        return 3;
    }
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

pub fn download_pkglist() {
    Command::new("curl")
        .arg("--output")
        .arg("/var/lib/vpt/packages.db")
        .arg("https://raw.githubusercontent.com/Vefjiaw/repo/main/pkglist")
        .output()
        .expect("Couldn't download package list.");
}

pub fn list_packages() -> String {
    // Open the database
    let db = sqlite::open("/var/lib/vpt/local/packages.db").unwrap();

    // Create the packages table if it doesn't exist
    db.execute(
        "CREATE TABLE if not exists packages (name TEXT, version TEXT, description TEXT, files TEXT);"
    ).unwrap();

    // Create a string to store the list of packages
    let mut packages = String::new();

    // Iterate over the rows in the packages table
    db.iterate("SELECT name FROM packages", |pairs| {
        // For each column in this row
        for &(column, value) in pairs.iter() {
            // If the column is "name"
            if column == "name" {
                // Add the value to the packages string
                packages = packages.to_owned() + " " + &value.unwrap().to_string();
            }
        }
        // Continue iterating
        true
    }).unwrap();

    // Return the packages string
    return packages;
}
