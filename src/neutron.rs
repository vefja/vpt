use rand::Rng;
use sqlite;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::{exit, Command, ExitStatus};
use std::{env, fs, io, str};
use version_compare::{Cmp, Version};

pub(crate) fn check_option(option: &str) -> bool {
    let output = Command::new("bash")
        .arg("/etc/elements/tools/find_opt.sh") // run find_opt.sh tool
        .arg(option) // add option to script
        .output() // take output of find_opt.sh
        .expect("Couldn't execute find_opt.sh"); // error

    let mut output_buffer = String::new(); // create buffer for output

    output_buffer.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from git"),
    }); // push output to buffer

    output_buffer.contains("true")
}

pub(crate) fn compare_old_to_new(package: &str) -> bool {
    // restore databases from backup using sqlite
    let mut db1 = sqlite::open("/var/lib/elements/local/packages.db").unwrap();
    let mut db2 = sqlite::open("/var/lib/elements/packages.db").unwrap();

    // create table packages if it doesn't exist
    db1.execute(
        "
        CREATE TABLE if not exists packages (name TEXT, version INTEGER, description TEXT);
        ",
    )
    .unwrap(); // make sure table exists

    // create other table pkglist if it doesn't exist
    db2.execute(
        "
        CREATE TABLE if not exists pkglist (name TEXT, version INTEGER, description TEXT);
        ",
    )
    .unwrap(); // make sure table exists

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
    let mut db = sqlite::open("/var/lib/elements/packages.db").unwrap();

    db.execute(
        "
        CREATE TABLE if not exists pkglist (name TEXT, version INTEGER, description TEXT);
        ",
    )
    .unwrap();

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
    let mut db = sqlite::open("/var/lib/elements/packages.db").unwrap();

    db.execute(
        "
        CREATE TABLE if not exists pkglist (name TEXT, version INTEGER, description TEXT);
        ",
    )
    .unwrap();

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

pub(crate) fn add_pkg_to_db(package: &str) {
    if !db_lock() {
        return;
    }; // check if database is locked

    let pkgdb = sqlite::open("/var/lib/elements/local/packages.db").unwrap();

    pkgdb.execute(
        "
        CREATE TABLE if not exists packages (name TEXT, version TEXT, description TEXT);
        ",
    );

    let ver = get_pkg_version(package); // TODO: remove placeholder

    let desc = get_pkg_desc(package);

    let cmd = "INSERT INTO packages VALUES ('".to_owned()
        + package
        + "', '"
        + &*ver.to_string()
        + "', '"
        + &desc
        + "');";

    println!("{}", cmd);

    pkgdb.execute(cmd).unwrap();

    // package = the function argument
    // version can be found
    // Description is found like version

    fs::remove_file("/var/lib/elements/local/packages.db.lock").unwrap();
}

pub(crate) fn debug_add_pkg_to_pkglist(package: &str) {
    let pkgdb = sqlite::open("/var/lib/elements/packages.db").unwrap();

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

    pkgdb
        .execute("CREATE TABLE if not exists pkglist (name TEXT, version TEXT, description TEXT);")
        .unwrap();
    pkgdb.execute(cmd).unwrap();

    // package = the function argument
    // version can be found
    // Description is found like version
}

fn db_lock() -> bool {
    if Path::new("/var/lib/elements/local/packages.db.lock").exists() {
        println!("Couldn't acquire lock on database");
        return false; // return false if you can't lock database
    }

    let mut file = File::create("/var/lib/elements/local/packages.db.lock").unwrap();
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
    // (mostly for usage on NTG OS or
    // non Nitrogen Linux distributions)
    return Path::new("/usr/bin/xbps-install").exists();
}

pub(crate) fn search_package(pkg_name: &str) -> bool {
    return !get_pkg_version(pkg_name).is_empty(); // return if package is found (true if found, false if not
}

pub(crate) fn get_package(pkg: &str, cache: bool, location: &str, tarName: &str) -> ExitStatus {
    let link = ""; // add link searching

    let status = if cache {
        let download_cmd = "curl ".to_owned() + link + " >> " + "/tmp/lmnt/" + tarName;

        Command::new(download_cmd)
            .status()
            .expect("Error: Couldn't download package.")
    } else {
        let download_cmd = "curl".to_owned() + link + &" >> ".to_owned() + location;

        Command::new(download_cmd)
            .status()
            .expect("Error: Couldn't download package.")
    };
    status
}

pub(crate) fn install_tar(pkg: &str, root: &str, offline: bool) -> i32 {
    // return i32 for error codes; 0 - good
    if !root.is_empty() && !Path::new(root).exists() {
        println!("Error: Cannot install to: {}: No such directory.", root);
    } else if !root.is_empty() {
        // TODO: add ability to install to a different root directory
    }

    let tarName = assign_random_name();

    if !offline {
        // offline install tries to install the package off the disk
        //get_package(pkg, true, "", &tarName);
    }

    let temp_dir = env::temp_dir();

    let mut tar_file = "/tmp/lmnt/".to_owned() + &tarName;

    println!("{}", tar_file);

    Command::new("tar")
        .arg("xzf")
        .arg(tar_file) // TODO: Change this to a random name
        .arg("-C")
        .arg(temp_dir)
        .output()
        .expect("Couldn't extract tar.gz file.");

    add_pkg_to_db(pkg);

    return 0;
}

fn assign_random_name() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let name: String = (0..10)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    return name;
}

pub(crate) fn inst_package(pkg: &str, root: &str) -> i32 {
    // status code
    if !root.is_empty() && !Path::new(root).exists() {
        println!("Error: Cannot install to: {}", root);
    } else if !root.is_empty() {
    }

    let mut pkg_db_path = File::open("/etc/elements/.sys_files/.pkg.db").unwrap();
    let mut pkg_db = String::new();

    pkg_db_path.read_to_string(&mut pkg_db).unwrap();

    if pkg_db.contains(pkg) {
        return 128; // package already installed
    }

    let path = "/etc/elements/repos/nitrogen/".to_owned() + pkg; // for elements pass args into pkg

    let build_log = Command::new("bash")
        .arg(path + "/build")
        .output()
        .expect("Didn't work.");

    let mut build_log_file = File::create("/tmp/build.log").unwrap();
    build_log_file.write_all(&build_log.stdout).unwrap();

    let updated_pkg_db = pkg_db.to_owned() + pkg + " ";
    // write_to_package_db(updated_pkg_db).expect("Couldn't write to package database.");

    0
}

pub(crate) fn rm_package(pkg: &str) -> i32 {
    let mut pkg_db_path = File::open("/etc/elements/.sys_files/.pkg.db").unwrap();
    let mut pkg_db = String::new();

    pkg_db_path.read_to_string(&mut pkg_db).unwrap();

    if !pkg_db.contains(pkg) {
        return 128; // package not installed
    }

    let path = "/etc/elements/repos/nitrogen/".to_owned() + pkg; // for elements pass args into pkg

    let build_log = Command::new("bash")
        .arg(path + "/remove")
        .output()
        .expect("Didn't work.");

    let mut build_log_file = File::create("/tmp/build.log").unwrap();
    build_log_file.write_all(&build_log.stdout).unwrap();

    let updated_pkg_db = pkg_db.replace(pkg, "");
    // write_to_package_db(updated_pkg_db).expect("Couldn't write to package database.");

    0
}

pub(crate) fn up_package(pkg: &str) -> i32 {
    let mut pkg_db_path = File::open("/etc/elements/.sys_files/.pkg.db").unwrap();
    let mut pkg_db = String::new();

    pkg_db_path.read_to_string(&mut pkg_db).unwrap();

    if !pkg_db.contains(pkg) {
        return 128; // package not installed
    }

    let path = "/etc/elements/repos/nitrogen/".to_owned() + pkg; // for elements pass args into pkg

    let update_log = Command::new("bash")
        .arg(path + "/build")
        .output()
        .expect("Didn't work.");

    let mut update_log_file = File::create("/tmp/update.log").unwrap();
    update_log_file.write_all(&update_log.stdout).unwrap();

    0
}

//noinspection ALL,RsTypeCheck
pub(crate) fn upgr_sys() {
    // system update
    let _p1_log = Command::new("/bin/sh") // set _p1_log so compiler doesn't scream, only used for xbps' log
        .output()
        .expect("How is this error even possible?");

    if test_xbps() && check_option("use_xbps") {
        // extra step for Nitrogen Linux(with xbps)
        println!("Updating xbps packages 1/5");
        let _p1_log = Command::new("xbps-install")
            .arg("-Suy")
            .output()
            .expect("Couldn't execute xbps");
    }

    if test_xbps() && check_option("use_xbps") {
        // Change numbering for non-xbps systems(so everything but Void and NL)
        println!("Removing old repository 2/5");
    } else {
        println!("Removing old repository 1/4")
    }
    Command::new("rm") // remove unnecessary files
        .arg("-rf")
        .arg("/etc/elements/repos/.old_nitrogen")
        .output()
        .expect("Couldn't execute rm");

    Command::new("cp") // copy outdated repository to .old_nitrogen
        .arg("-rf") // copy the whole directory
        .arg("/etc/elements/repos/nitrogen") // copy from
        .arg("/etc/elements/repos/.old_nitrogen") // copy to
        .output()
        .expect("Couldn't backup repository.");

    let p2_log = Command::new("rm")
        .arg("-rf") // forced recursively remove
        .arg("/etc/elements/repos/nitrogen") // path to remove
        .output()
        .expect("Couldn't remove repository.");

    if test_xbps() && check_option("use_xbps") {
        println!("Re-clone Repository 3/5");
    } else {
        println!("Re-clone Repository 2/4")
    }
    let p3_log = Command::new("git")
        .arg("clone")
        .arg("https://github.com/NitrogenLinux/elements-repo.git") // Nitrogen Linux's main repository
        .arg("/etc/elements/repos/nitrogen") // path to clone to
        .output()
        .expect("Couldn't clone the repository.");

    let pkg_perm_log = Command::new("chmod")
        .arg("a+x")
        .arg("-R")
        .arg("/etc/elements/repos/nitrogen") // path to chmod
        .output()
        .expect("Couldn't set permissions for the repository.");

    if test_xbps() && check_option("use_xbps") {
        println!("Reinstall Elements 4/5");
    } else {
        println!("Reinstall Elements 3/4")
    }

    let p4_log = Command::new("curl")
        .arg("-s")
        .arg("https://api.github.com/repos/NitrogenLinux/Elements/releases/latest | grep 'browser_download_url.*lmt' | cut -d : -f 2,3 | tr -d \" | wget -qi -")// get the latest release
        .output()
        .expect("Couldn't execute curl");

    let mv_log = Command::new("mv")
        .arg("-v") // verbose for logging
        .arg("lmt")
        .arg("/usr/bin/lmt") // move the file to /usr/bin/lmt
        .output()
        .expect("Couldn't move the file.");

    let chmod_log = Command::new("chmod")
        .arg("a+x")
        .arg("-v") // verbose for logging
        .arg("/usr/bin/lmt") // make the file executable
        .output()
        .expect("Couldn't make the file executable.");

    let mut pkg_db_path = File::open("/etc/elements/.sys_files/.pkg.db").unwrap();
    let mut pkg_db = String::new();
    pkg_db_path.read_to_string(&mut pkg_db).unwrap();

    let packages_to_update = words_count::count_separately(&pkg_db);

    let mut pkg_left = packages_to_update.len();
    let mut packages_done = 0;

    if test_xbps() && check_option("use_xbps") {
        println!("Updating Rest of Packages 5/5");
    } else {
        println!("Updating Packages 4/4")
    }

    let tmp = pkg_db.split(' ');

    let mut pkg_db_vec: Vec<_> = tmp.collect();

    let mut no_blank_spaces = false;

    let mut verified_slot = 0;

    while !no_blank_spaces {
        if pkg_db_vec[verified_slot].is_empty() {
            pkg_db_vec.remove(verified_slot);
        } else {
            verified_slot += 1;
        }
        if verified_slot == pkg_db_vec.len() {
            no_blank_spaces = true;
        }
    }

    let mut kernel_change = false;

    while pkg_left > 0 {
        if compare_old_to_new(pkg_db_vec[packages_done]) {
            println!(
                "Updating: {0} {1}/{2}",
                pkg_db_vec[packages_done],
                packages_done + 1,
                packages_to_update.len()
            );

            if pkg_db_vec[packages_done] == "kernel" || pkg_db_vec[packages_done] == "linux" {
                kernel_change = true;
            }

            install_tar(pkg_db_vec[packages_done], "", false);
        }

        pkg_left -= 1;
        packages_done += 1;
    }

    let mut update_log_file = File::create("/tmp/update.log").unwrap();

    if test_xbps() && check_option("use_xbps") {
        // add log for xbps
        update_log_file.write_all(&_p1_log.stdout).unwrap();
    }

    update_log_file.write_all(&p2_log.stdout).unwrap();
    update_log_file.write_all(&p3_log.stdout).unwrap();
    update_log_file.write_all(&pkg_perm_log.stdout).unwrap();
    update_log_file.write_all(&p4_log.stdout).unwrap();
    update_log_file.write_all(&mv_log.stdout).unwrap();
    update_log_file.write_all(&chmod_log.stdout).unwrap();

    Command::new("rm")
        .arg("-rf") // forced recursively remove
        .arg("/etc/elements/repos/.old_nitrogen") // path to remove
        .output()
        .expect("Couldn't remove repository.");

    Command::new("cp")
        .arg("-rv") // verbose for logging
        .arg("/etc/elements/repos/nitrogen")
        .arg("/etc/elements/repos/.old_nitrogen") // Copy new repository to old repository
        .output()
        .expect("Couldn't remove the old repository.");

    if kernel_change {
        println!("A restart is required in order to complete the update");
    }

    if check_option("snapshots") {
        new_snapshot("post", "system upgrade");
    }

    exit(0);
}
