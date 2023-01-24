use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::{env, fs, io, str};
use version_compare::{Cmp, Version};
use sqlite;
use rand::Rng;
use std::process::{exit, Command, ExitStatus};


pub(crate) fn check_option(option: &str) -> bool {
    let output = Command::new("bash")
        .arg("/etc/vpt/tools/find_opt.sh") // run find_opt.sh tool
        .arg(option) // add option to script
        .output() // take output of find_opt.sh
        .expect("Couldn't execute find_opt.sh"); // error

    let mut output_buffer = String::new(); // create buffer for output

    output_buffer.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from find_opt.sh"),
    }); // push output to buffer

    output_buffer.contains("true")
}

pub(crate) fn compare_old_to_new(package: &str) -> bool {
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

    println!("{0} {1}", oldver, newver);

    let oldver = Version::from(&oldver).unwrap();

    let newver = Version::from(&newver).unwrap();

    let mut is_newer = false;

    if oldver.compare(newver) == Cmp::Gt {
        is_newer = true;
    }

    // println!("Current: {0} \n Newest: {1:?} \n Newer? {2}", db1_ver, db2_ver, isnewer);

    return !is_newer; // return if db2's version is newer than db1's
}

fn get_pkg_version(package: &str) -> String {
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

pub(crate) fn add_pkg_to_db(package: &str, files: &str) {
    if !db_lock() { return; }; // check if database is locked

    let pkgdb = sqlite::open("/var/lib/vpt/local/packages.db").unwrap();

    pkgdb
        .execute("
        CREATE TABLE if not exists packages (name TEXT, version TEXT, description TEXT, files TEXT);
        ", ).unwrap();

    let ver = get_pkg_version(package);

    let desc = get_pkg_desc(package);

    println!("{} {} {}", package, ver, desc);

    let cmd = "INSERT INTO packages VALUES ('".to_owned()
        + package
        + "', '"
        + &*ver.to_string()
        + "', '"
        + &desc
        + "', '"
        + files
        + "');";

    println!("{}", cmd);

    pkgdb.execute(cmd).unwrap();

    // package = the function argument
    // version can be found
    // Description is found like version

    fs::remove_file("/var/lib/vpt/local/packages.db.lock").unwrap();
}

pub(crate) fn debug_add_pkg_to_pkglist(package: &str) {
    let pkgdb = sqlite::open("/var/lib/vpt/packages.db").unwrap();

    let ver = 999999;

    let desc = "Foo Bar Baz";

    let cmd = "INSERT INTO pkglist VALUES ('".to_owned()
        + package
        + "', '"
        + &*ver.to_string()
        + "', '"
        + desc
        + "');";

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

pub(crate) fn new_snapshot(snapshot_type: &str, snapshot_reason: &str) {
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

pub(crate) fn test_xbps() -> bool {
    // check if xbps can be found on the OS
    // (mostly for usage on VF OS or
    // non Vefjiaw Linux distributions)
    return Path::new("/usr/bin/xbps-install").exists();
}

pub(crate) fn search_package(pkg_name: &str) -> bool {
    return !get_pkg_version(pkg_name).is_empty(); // return if package is found (true if found, false if not
}

pub(crate) fn get_package(pkg: &str, cache: bool, location: &str, tarName: &str) -> ExitStatus {
    let link = "https://raw.githubusercontent.com/vefjiaw/vpt-repo/main/".to_owned() + pkg + ".tar.gz"; // add link searching

    let status = {
        Command::new("curl")
            .arg(&link)// the link that is figured out previously
            .arg("-o")
            .arg("/tmp/vpt/".to_owned() + tarName + ".tar.gz")// name of the tar file
            .status()
            .expect("Error: Couldn't download package.")
    };

    status
}

pub(crate) fn install_tar(pkg: &str, root: &str, offline: bool, upgrade: bool) -> i32 {
    // return i32 for error codes; 0 - good
    if !root.is_empty() && !Path::new(root).exists() {
        println!("Error: Cannot install to: {}: No such directory.", root);
    } else if !root.is_empty() {
        // TODO: add ability to install to a different root directory
    }

    let tarName = assign_random_name();

    println!("{}", tarName);

    let dir_name = assign_random_name();

    let temp_dir = "/tmp/vpt/".to_owned() + &dir_name;

    if !offline {
        // offline install tries to install the package off the disk
        get_package(pkg, true, "", &tarName);
    }

    let tar_file = "/tmp/vpt/".to_owned() + &tarName + ".tar.gz";

    fs::create_dir_all(&temp_dir).expect("Couldn't create temp directory.");

    println!("{}", tar_file);
    Command::new("tar")
        .arg("xzf")
        .arg(tar_file)
        .arg("-C")
        .arg(&temp_dir)
        .output()
        .expect("Couldn't extract tar.gz file.");

    println!("Files in {}:", temp_dir);

    let paths = fs::read_dir(temp_dir).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    let usr_binpath = "/tmp/vpt".to_owned() + &dir_name + "/BINARIES" + "/usr-bin";

    let bin_binpath = "/tmp/vpt".to_owned() + &dir_name + "/BINARIES" + "/bin";

    if Path::new(&usr_binpath).is_dir() {
        for patch in fs::read_dir(&usr_binpath).unwrap() {
            let patch = patch.unwrap().path();
            let patch = patch.to_str().unwrap();
            println!("Installing: {}", patch);
        }
    } else if Path::new(&bin_binpath).is_dir() {
        for patch in fs::read_dir(&bin_binpath).unwrap() {
            let patch = patch.unwrap().path();
            let patch = patch.to_str().unwrap();
            println!("Installing: {}", patch);
        }
    } // TODO: do the same for CONFIGS, BOOT, MANUALS, and LIBRARIES

    if !upgrade {
        add_pkg_to_db(pkg, "");
    } else {
        let pkgdb = sqlite::open("/var/lib/vpt/local/packages.db").unwrap();
        pkgdb
            .execute("
            CREATE TABLE if not exists packages (name TEXT, version TEXT, description TEXT, files TEXT);
            ", );

        let ver = get_pkg_version(pkg); // TODO: remove placeholder

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

fn assign_random_name() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();

    let name: String = (0..10).map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    }).collect();

    return name;
}

pub(crate) fn remove_tar(pkg: &str) -> i32 {
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

    println!("{:?}", all_files);

    return 0;
}

pub(crate) fn download_pkglist() {
    Command::new("curl")
        .arg("--output")
        .arg("/var/lib/vpt/packages.db")
        .arg("https://raw.githubusercontent.com/Vefjiaw/vpt-repo/main/pkglist")
        .output()
        .expect("Couldn't download package list.");
}

pub(crate) fn list_packages() {
    let db = sqlite::open("/var/lib/vpt/local/packages.db").unwrap();
    let mut packages = String::new();

    db.execute(
        "CREATE TABLE if not exists packages (name TEXT, version TEXT, description TEXT, files TEXT);"
    ).unwrap();

    db.iterate("SELECT name FROM packages", |pairs| {
        for &(column, value) in pairs.iter() {
            if column == "name" {
                packages = packages.to_owned() + &value.unwrap().to_string();
            }
        }
        true
    }).unwrap();
}

pub(crate) fn upgrade_system() -> i32 {
    let mut p1_log = Command::new("sh")
        .arg("-c")
        .arg("echo 'Updating package list...' && curl --output /var/lib/vpt/pkglist https://raw.githubusercontent.com/NitrogenLinux/vpt-repo/main/pkglist && echo 'Done.'")
        .output()
        .expect("Couldn't update package list.");

    // TODO: finish upgrade
    return 0;
}