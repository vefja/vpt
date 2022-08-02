use nix::unistd::getuid;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::{exit, Command};
use std::{env, io};

fn main() {
    let mut args: Vec<String> = env::args().collect(); // take args in a vector
    let clone_args: Vec<String> = env::args().collect(); // have an immutable version of args

    if args.len() >= 2 {
        // detect action
        let action = &clone_args[1];

        if action.to_lowercase().eq("install")
            || action.to_lowercase().eq("remove")
            || action.to_lowercase().eq("update")
            || action.to_lowercase().eq("search")
        { // detect action
             // pass if action is install, remove, update or search
        } else if action.to_lowercase().eq("help") {
            println!("usage: lmt <action> <package>");
            println!("List of Main Commands:");
            println!("  install: Install a package");
            println!("  remove: Remove a package");
            println!("  update: Update all packages");
            println!("  search: Search for a package");
            println!("  help: Show this help message");
            exit(0);
        } else {
            println!("No command called '{0}' found.", action);
            exit(1);
        }

        if args.len() >= 3 {
            for i in 2..args.len() {
                if args[i].is_empty() {
                    // Throw error if "" is passed as argument
                    println!("Error: Unknown error.");
                    exit(512);
                }

                if args[i].contains(' ') {
                    // Throw error if package name contains space
                    println!("Error: Package name cannot be empty.");
                    exit(512);
                }

                if args[i].contains('.') || args[i].contains('/') {
                    // Cannot believe the things I have to do to make elements not install nothing
                    println!("Error: Package name cannot contain '{}'", args[i]);
                    exit(512);
                }
            }

            if !getuid().to_string().eq("0") {
                // Error and exit if executed without root privileges
                println!("You must be root to execute command: '{}'", args[1]);
                exit(128);
            }

            // detect if package is specified and install
            args.remove(0); // remove exec name
            args.remove(0); // remove argument

            if args.len() == 1 {
                if action.to_lowercase().eq("search") {
                    // search
                    if Path::new(&("/etc/elements/repos/Nitrogen/".to_owned() + &args[0])).exists()
                    {
                        println!(
                            "Package: '{}' was found in Nitrogen Linux's repositories.",
                            args[0]
                        );

                        println!("Use 'lmt install {}' to install it.", args[0]);
                    } else {
                        println!("No package called '{0}' found.", args[0]);
                        exit(1);
                    }
                    exit(0);
                }

                if action.to_lowercase().eq("install") || action.to_lowercase().eq("in") {
                    println!("Installing {0:?}", args.join(" "));
                } else if action.to_lowercase().eq("remove") || action.to_lowercase().eq("rm") {
                    println!("Removing: {0:?}", args.join(" "));
                } else if action.to_lowercase().eq("update") || action.to_lowercase().eq("up") {
                    println!("Updating: {0:?}", args.join(" "));
                }
            } else if action.to_lowercase().eq("install") || action.to_lowercase().eq("in") {
                println!("Installing {0} packages: {1:?}", args.len(), args.join(" "));
            } else if action.to_lowercase().eq("remove") || action.to_lowercase().eq("rm") {
                println!("Removing {0} packages: {1:?}", args.len(), args.join(" "));
            } else if action.to_lowercase().eq("update") || action.to_lowercase().eq("up") {
                println!("Updating {0} packages: {1:?}", args.len(), args.join(" "));
            }

            if ["install", "remove", "update", "in", "rm", "up"].contains(&&*action.to_lowercase())
            {
                print!("Continue? [y/n] "); // ask for confirmation
                io::stdout().flush().unwrap(); // flush stdout
                let mut input = String::new(); // create a string to store input

                io::stdin().read_line(&mut input).unwrap(); // take input

                if input.to_lowercase().contains('y') || input.len() == 1 {
                    // pass
                } else {
                    // if input is not empty, nor yes
                    println!("Aborting."); // print abort message
                    exit(0); // exit
                }
            } else {
                // In case of an error, No idea what triggers this error but it happens.
                println!("Couldn't execute: '{}': Unknown error.", action);
                println!("Do not report this error."); // just don't dare to report this error.
                exit(512); // exit
            }

            let mut package_to_install = 0; // create a variable to store the number of packages to install

            while package_to_install < args.len() {
                if ["elements", "gnome-core", "gnome", "linux", "xbps"]
                    .contains(&&*args[package_to_install])
                {
                    println!(
                        "Cannot remove {}: Package is required by the system.",
                        &args[package_to_install]
                    ); // print error message
                    exit(256);
                }

                let mut pkg_db_path = File::open("/etc/elements/.sys_files/.pkg.db").unwrap();
                let mut updated_pkg_db = String::new();
                pkg_db_path.read_to_string(&mut updated_pkg_db).unwrap();

                let path = "/etc/elements/repos/Nitrogen/".to_owned() + &args[package_to_install];

                if Path::new(&path).exists() {
                    if action.to_string().eq("install") {
                        if updated_pkg_db.contains(&args[package_to_install]) {
                            println!(
                                "{} already installed. Reinstalling.",
                                &args[package_to_install]
                            );
                        } else {
                            println!(
                                "Installing package: {0} [{1}/{2}]",
                                &args[package_to_install],
                                package_to_install + 1,
                                args.len()
                            );
                            let updated_pkg_db = updated_pkg_db + &*args[package_to_install] + " ";
                            write_to_package_db(updated_pkg_db)
                                .expect("Couldn't write to package database.");
                        }

                        let build_log = Command::new("bash")
                            .arg(path.to_owned() + "/build")
                            .output()
                            .expect("Didn't work.");

                        let mut build_log_file = File::create("/tmp/build.log").unwrap();
                        build_log_file.write_all(&build_log.stdout).unwrap();
                    } else if action.to_string().eq("remove") {
                        if updated_pkg_db.contains(&args[package_to_install]) {
                            let updated_pkg_db =
                                updated_pkg_db.replace(&args[package_to_install], "");

                            write_to_package_db(updated_pkg_db)
                                .expect("Couldn't write to package database.");
                        } else {
                            println!(
                                "Cannot remove {}: Package not installed.",
                                &args[package_to_install]
                            );
                            exit(256);
                        }
                        println!(
                            "Removing package: {0} [{1}/{2}]",
                            &args[package_to_install],
                            package_to_install + 1,
                            args.len()
                        ); // print action and the number of packages remaining
                        let remove_log = Command::new("bash")
                            .arg(path.to_owned() + "/remove")
                            .output()
                            .expect("Didn't work.");
                        let mut remove_log_file = File::create("/tmp/remove.log").unwrap();
                        remove_log_file.write_all(&remove_log.stdout).unwrap();
                    }
                } else if action.to_string().eq("update") {
                    if !updated_pkg_db.contains(&args[package_to_install]) {
                        println!(
                            "Cannot update {}: Package not installed.",
                            &args[package_to_install]
                        );
                        exit(256);
                    }
                    println!(
                        "Updating package {0} {1}/{2}",
                        &args[package_to_install],
                        package_to_install + 1,
                        args.len()
                    ); // print action and the number of packages remaining
                    let update_log = Command::new("bash")
                        .arg(path.to_owned() + "/build")
                        .output()
                        .expect("Didn't work.");

                    let mut update_log_file = File::create("/tmp/update.log").unwrap();
                    update_log_file.write_all(&update_log.stdout).unwrap();
                } else {
                    if action.eq("install") {
                        let build_log = Command::new("xbps-install")
                            .arg("-Sy")
                            .arg(&args[package_to_install])
                            .output()
                            .expect("Couldn't execute xbps");

                        let mut build_log_file = File::create("/tmp/build.log").unwrap();
                        build_log_file.write_all(&build_log.stdout).unwrap();
                    } else if action.eq("remove") {
                        let removal_log = Command::new("xbps-remove")
                            .arg("-y")
                            .arg(&args[package_to_install])
                            .output()
                            .expect("Couldn't execute xbps");

                        let mut removal_log_file = File::create("/tmp/build.log").unwrap();
                        removal_log_file.write_all(&removal_log.stdout).unwrap();
                    } else if action.eq("update") {
                        let update_log = Command::new("xbps-install")
                            .arg("-Sy")
                            .arg(&args[package_to_install])
                            .output()
                            .expect("Couldn't execute xbps");

                        let mut update_log_file = File::create("/tmp/build.log").unwrap();
                        update_log_file.write_all(&update_log.stdout).unwrap();
                    } else {
                        println!("{} is not a valid action.", action);
                        exit(1);
                    }
                    exit(0);
                }
                package_to_install += 1;
                if package_to_install == args.len() {
                    exit(0);
                }
            }
        } else if action.to_lowercase().eq("update") {
            println!("Updating Void packages 1/5");
            let p1_log = Command::new("xbps-install")
                .arg("-Suy")
                .output()
                .expect("Couldn't execute xbps");

            println!("Removing old repository 2/5");
            Command::new("rm") // remove unnecessary files
                .arg("-rf")
                .arg("/etc/elements/repos/.old_Nitrogen")
                .output()
                .expect("Couldn't execute rm");

            Command::new("cp") // copy outdated repository to .old_Nitrogen
                .arg("-rf") // copy the whole directory
                .arg("/etc/elements/repos/Nitrogen") // copy from
                .arg("/etc/elements/repos/.old_Nitrogen") // copy to
                .output()
                .expect("Couldn't backup repository.");

            let p2_log = Command::new("rm")
                .arg("-rf") // forced recursively remove
                .arg("/etc/elements/repos/Nitrogen") // path to remove
                .output()
                .expect("Couldn't remove repository.");

            println!("Re-clone Repository 3/5");
            let p3_log = Command::new("git")
                .arg("clone")
                .arg("https://github.com/NitrogenLinux/elements-repo.git") // Nitrogen Linux's main repository
                .arg("/etc/elements/repos/Nitrogen") // path to clone to
                .output()
                .expect("Couldn't clone the repository.");

            let pkg_perm_log = Command::new("chmod")
                .arg("a+x")
                .arg("-R")
                .arg("/etc/elements/repos/Nitrogen") // path to chmod
                .output()
                .expect("Couldn't set permissions for the repository.");

            println!("Reinstall elements 4/5");
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

            println!("Updating Rest of Packages 5/5");

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
                    "/etc/elements/repos/Nitrogen/".to_owned()
                        + pkg_db_vec[packages_done]
                        + "/version",
                )
                .unwrap();

                let mut version = String::new();
                version_path.read_to_string(&mut version).unwrap();

                let mut version_old_path = File::open(
                    "/etc/elements/repos/.old_Nitrogen/".to_owned()
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
                            "/etc/elements/repos/Nitrogen/".to_owned()
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
            update_log_file.write_all(&p1_log.stdout).unwrap();
            update_log_file.write_all(&p2_log.stdout).unwrap();
            update_log_file.write_all(&p3_log.stdout).unwrap();
            update_log_file.write_all(&pkg_perm_log.stdout).unwrap();
            update_log_file.write_all(&p4_log.stdout).unwrap();
            update_log_file.write_all(&mv_log.stdout).unwrap();
            update_log_file.write_all(&chmod_log.stdout).unwrap();

            Command::new("rm")
                .arg("-rf") // forced recursively remove
                .arg("/etc/elements/repos/.old_Nitrogen") // path to remove
                .output()
                .expect("Couldn't remove repository.");

            Command::new("cp")
                .arg("-rv") // verbose for logging
                .arg("/etc/elements/repos/Nitrogen")
                .arg("/etc/elements/repos/.old_Nitrogen") // Copy new repository to old repository
                .output()
                .expect("Couldn't remove the old repository.");

            println!(
                "Update complete. A restart may be needed to use new libraries and/or kernels."
            );
            exit(0);
        } else {
            println!("No package specified to {0}.", action.to_lowercase());
            exit(2);
        }
    } else {
        println!("usage: lmt <action> <package>");
        println!("List of Main Commands:");
        println!("  install: Install a package");
        println!("  remove: Remove a package");
        println!("  update: Update all packages");
        println!("  search: Search for a package");
        println!("  help: Show this help message");
        exit(1);
    }
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
