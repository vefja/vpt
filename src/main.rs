use nix::unistd::getuid;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::{exit, Command};
use std::{env, io, str};

fn upgr_sys() { // system update
    let _p1_log = Command::new("/bin/sh")
        .output()
        .expect("How is this error even possible?");

    if test_xbps() && check_option("use_xbps") {
        // extra step for Nitrogen Linux
        println!("Updating xbps packages 1/5");
        let _p1_log = Command::new("xbps-install")
            .arg("-Suy")
            .output()
            .expect("Couldn't execute xbps");
    }

    if test_xbps() && check_option("use_xbps") {
        // Change numbering for non-xbps systems(so everything but Void)
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

    while pkg_left > 0 {
        let mut version_path = File::open(
            "/etc/elements/repos/nitrogen/".to_owned()
                + pkg_db_vec[packages_done]
                + "/version",
        )
            .unwrap();

        let mut version = String::new();
        version_path.read_to_string(&mut version).unwrap();

        let mut version_old_path = File::open(
            "/etc/elements/repos/.old_nitrogen/".to_owned()
                + pkg_db_vec[packages_done]
                + "/version",
        )
            .unwrap();
        let mut version_old = String::new();
        version_old_path.read_to_string(&mut version_old).unwrap();

        if !version.eq(&version_old) {
            println!(
                "Updating: {0} {1} => {2}",
                pkg_db_vec[packages_done], version_old, version
            );

            Command::new("bash")
                .arg(
                    "/etc/elements/repos/nitrogen/".to_owned()
                        + pkg_db_vec[packages_done]
                        + "/build",
                )
                .output()
                .expect("Couldn't execute bash");
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

    println!(
        "Update complete. A restart may be needed to use new libraries and/or kernels."
    );

    if check_option("snapshots") {
        take_snapshot("post", "system upgrade");
    }


    exit(0);
}

fn write_to_package_db(package: String) -> io::Result<()> {
    let mut package_db = File::create("/etc/elements/.sys_files/.pkg.db").unwrap();
    package_db
        .write_all(package.as_bytes())
        .expect("write failed");

    let mut input = File::open("/etc/elements/.sys_files/.pkg.db")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;
    Ok(())
}

fn check_option(option: &str) -> bool {
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

fn take_snapshot(snapshot_type: &str, snapshot_reason: &str) {
    // TODO: reactivate snapshots
    // let reason = snapshot_type.to_lowercase() + " " + snapshot_reason;
    // Command::new("snapper")
    //     .arg("-c")
    //     .arg("root")
    //     .arg("create")
    //     .arg("--description")
    //     .arg(reason)
    //     .output()
    //     .expect("Couldn't take snapshot.");
}

fn test_xbps() -> bool {
    // check if xbps can be found on the OS
    // (mostly for usage on NTG OS)
    return Path::new("/usr/bin/xbps-install").exists();
}


fn main() {
    let mut args_mod: Vec<String> = env::args().collect(); // args_mod that can be modified
    let imut_args: Vec<String> = env::args().collect(); // immutable args_mod for other things

    if imut_args.len() >= 2 {
        let command = &imut_args[1].to_lowercase();

        if command.eq("install") || command.eq("in")
            || command.eq("remove") || command.eq("rm")
            || command.eq("upgrade") || command.eq("up")
            || command.eq("search") || command.eq("se") {
            if check_option("snapshots") {
                take_snapshot("pre", &imut_args[1]);
            }

            if !getuid().to_string().eq("0") {
                println!("You must be root to use command: {0}", args_mod[1]);
                exit(512);
            }
        } else if command.eq("help") || command.eq("h") {
            println!("usage: lmt <action> <package>");
            println!("List of Main Commands:");
            println!("  install: Install a package");
            println!("  remove: Remove a package");
            println!("  update: Update all packages");
            println!("  search: Search for a package");
            println!("  help: Show this help message");
            exit(0);
        } else {
            println!("No command called '{0}' found.", command);
            exit(256);
        }

        if imut_args[1].eq("upgrade")&& imut_args.len() == 2 {
            upgr_sys(); // put system update in a func to clean up main
        }

        if imut_args.len() >= 3 {
            for i in 2..args_mod.len() {
                if args_mod[i].is_empty() {
                    // Throw error if "" is passed as argument
                    println!("Error: Unknown error.");
                    exit(512);
                }


                if args_mod[i].contains(' ') {
                    // Throw error if package name contains space
                    println!("Error: Package name cannot be empty.");
                    exit(512);
                }


                if args_mod[i].contains('.') || args_mod[i].contains('/') {
                    println!("Error: Package name cannot contain '{}'", args_mod[i]);
                    exit(512);
                }

                if !check_option("remove_protected") && command.eq("remove") && [
                    "elements",
                    "gnome-core",
                    "gnome",
                    "linux",
                    "xbps",
                    "mutter",
                    "kernel",
                ] // kernel - nitrogen os's kernel
                    .contains(&&*args_mod[i]) {
                    println!("Cannot remove '{0}': Package is required by system.", args_mod[i]);
                    exit(128);
                }

                let path = "/etc/elements/repos/nitrogen/".to_owned() + &args_mod[i];
                if !Path::new(&path).exists() {
                    println!("Couldn't find '{0}' in the repository.", args_mod[i]);
                    exit(0);
                }
            }
        } else {
            println!("3 arguments expected(2 given).");
            exit(512);
        }
    } else {
        println!("usage: lmt <action> <package>");
        println!("List of Main Commands:");
        println!("  install: Install a package");
        println!("  remove: Remove a package");
        println!("  update: Update all packages");
        println!("  search: Search for a package");
        println!("  help: Show this help message");
        exit(256);
    }

    // code below executes after the necessary checks above

    args_mod.remove(0);
    args_mod.remove(0); // remove non-important arguments(will be saved in imut args_mod)

    if imut_args[2].eq("search") {
        if Path::new(&("/etc/elements/repos/nitrogen/".to_owned() + &args_mod[0])).exists() {
            println!("Package: {0} was found in Elements' repository.", &args_mod[0]);
            println!("Use 'lmt install {0}' to install it.", &args_mod[0])
        } else { println!("Couldn't find '{0}' .", &args_mod[0]) }
    }

    let command = &imut_args[1].to_lowercase(); // redeclare "command"

    args_mod.dedup(); // remove duplicates

    if command.eq("install") && args_mod.len() == 1 {
        println!("Installing {0:?}", args_mod.join(" "));
    } else if command.eq("remove") && args_mod.len() == 1 {
        println!("Removing: {0:?}", args_mod.join(" "));
    } else if command.eq("upgrade") && args_mod.len() == 1 {
        println!("Upgrading: {0:?}", args_mod.join(" "));
    } else if command.eq("install") && args_mod.len() != 1 {
        println!("Installing {0} packages: {1:?}", args_mod.len(), args_mod.join(" "));
    } else if command.eq("remove") && args_mod.len() != 1 {
        println!("Removing {0} packages: {1:?}", args_mod.len(), args_mod.join(" "));
    } else if command.eq("upgrade") && args_mod.len() != 1 {
        println!("Upgrading {0} packages: {1:?}", args_mod.len(), args_mod.join(" "));
    }

    let mut in_prompt = true;

    while in_prompt {
        print!("Continue? [Y/n] ");
        io::stdout().flush().unwrap(); // flush stdout

        let mut input = String::new(); // answer to the "Continue?" prompt
        io::stdin().read_line(&mut input).unwrap(); // take input
        input = input.to_lowercase();

        println!("{0} {1}", !input.eq("y\n"), !input.eq("yes\n"));

        if input.eq("n\n") || input.eq("no\n") { // if answer is "n" or "no"
            println!("Aborting.");
            exit(0);
        } else if !input.eq("y\n") && !input.eq("yes\n") { // if answer is neither "y" nor "yes"
            println!("Input Error: Unknown answer.")
        } else {
            in_prompt = false;
        }
    }

    let mut pkgs_done = 0;
    let mut pkg_db_path = File::open("/etc/elements/.sys_files/.pkg.db").unwrap();
    let mut pkg_db = String::new();

    while pkgs_done < args_mod.len() {
        pkg_db_path.read_to_string(&mut pkg_db).unwrap();

        let path = "/etc/elements/repos/nitrogen/".to_owned() + &args_mod[pkgs_done];

        let updated_pkg_db = "";


        if command.eq("install") {
            if pkg_db.contains(&args_mod[pkgs_done]) {
                println!("'{}' is already installed. Skipping.", args_mod[pkgs_done]);
            } else {
                println!("Installing package: {0} {1}/{2}", &args_mod[pkgs_done], pkgs_done + 1, args_mod.len());
                let updated_pkg_db = pkg_db.to_owned() + &*args_mod[pkgs_done] + " ";
                write_to_package_db(updated_pkg_db)
                    .expect("Couldn't write to package database.");

                let build_log = Command::new("bash")
                    .arg(path.to_owned() + "/build")
                    .output()
                    .expect("Didn't work.");

                let mut build_log_file = File::create("/tmp/build.log").unwrap();
                build_log_file.write_all(&build_log.stdout).unwrap();
            }
        } else if command.eq("remove") {
            if pkg_db.contains(&args_mod[pkgs_done]) {
                println!("Removing package: {0} {1}/{2}", &args_mod[pkgs_done], pkgs_done + 1, args_mod.len());

                let mut updated_pkg_db = pkg_db.replace(&args_mod[pkgs_done], "");
                write_to_package_db(updated_pkg_db);

                let remove_log = Command::new("bash")
                    .arg(path.to_owned() + "/remove")
                    .output()
                    .expect("Didn't work.");
                let mut remove_log_file = File::create("/tmp/remove.log").unwrap();
                remove_log_file.write_all(&remove_log.stdout).unwrap();
            } else {
                println!("Couldn't remove: {0}: Package not installed", args_mod[pkgs_done])
            }
        } else if command.eq("update") {
            if !pkg_db.contains(&args_mod[pkgs_done]) {
                println!("Couldn't update: {0}: Package not installed. Skipping.", args_mod[pkgs_done])
            } else {
                println!("Updating package: {0} {1}/{2}", &args_mod[pkgs_done], pkgs_done + 1, args_mod.len());
                let update_log = Command::new("bash")
                    .arg(path.to_owned() + "/build")
                    .output()
                    .expect("Didn't work.");

                let mut update_log_file = File::create("/tmp/update.log").unwrap();
                update_log_file.write_all(&update_log.stdout).unwrap();
            }
        }

        pkgs_done += 1;
    }
}
