use std::{env, io};
use std::path::Path;
use std::io::{stdout, Write};
use nix::unistd::getuid;
use crate::imut_api::enterrw;
use crate::neutron::{add_pkg_to_db, compare_old_to_new, debug_add_pkg_to_pkglist, install_tar, search_package};

mod neutron; // import Neutron API
mod imut_api; // Immutability API

fn main() {
    let mut args_mod: Vec<String> = env::args().collect(); // args_mod that can be modified
    let imut_args: Vec<String> = env::args().collect(); // immutable args_mod for other things

    if imut_args.len() >= 2 {
        let command = &imut_args[1].to_lowercase();

        if command == "install"
            || command == "in"
            || command == "remove"
            || command == "rm"
            || command == "upgrade"
            || command == "up"
            || command == "search"
            || command == "se"
        {
            if neutron::check_option("snapshots") {
                neutron::new_snapshot("pre", &imut_args[1]);
            }

            if !getuid().to_string().eq("0") {
                println!("You must be root to use this command!");
                std::process::exit(1);
            }

        } else if command.eq("help") || command.eq( "help") {
            help(0);
        } else {
            println!("Invalid operation: {}", command);
            std::process::exit(1);
        }
    } else {
        println!("At least one 2 arguments are required(1 found)");
        std::process::exit(1);
    }

    let command = &imut_args[1].to_lowercase(); // redeclare in main

    if imut_args.len() >= 3 {
        for i in 2..args_mod.len() {
            if args_mod[i].is_empty() {
                // Throw error if "" is passed as argument
                println!("Error: Unknown error.");
                std::process::exit(512); // Error 512 for invalid arguments
            }

            if args_mod[i].contains(' ') {
                // Throw error if package name contains space
                println!("Error: Package name cannot be empty.");
                std::process::exit(512);
            }

            if args_mod[i].contains('.') || args_mod[i].contains('/') {
                println!("Error: Package name cannot contain '{}'", args_mod[i]);
                std::process::exit(512);
            }

            if !neutron::check_option("remove_protected")
                && command.eq("remove")
                && [
                "elements",
                "gnome-core",
                "gnome",
                "linux",
                "xbps",
                "mutter",
                "kernel",
            ] // kernel - nitrogen os's kernel
                .contains(&&*args_mod[i])
            {
                println!(
                    "Cannot remove '{0}': Package is required by system.",
                    args_mod[i]
                );
                std::process::exit(128);
            }

            let path = "/etc/elements/repos/nitrogen/".to_owned() + &args_mod[i];
            if !Path::new(&path).exists() {
                println!("Couldn't find '{0}' in the repository.", args_mod[i]);
                std::process::exit(256); // Error 256 for package not found
            }
        }
    } else if command == "upgrade" || command == "up" {
        let mut prompt = true;
        while prompt {
            print!("Are you sure you want to upgrade all packages? [Y/n] ");
            stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            if input == "y" || input == "yes" || input.is_empty() {
                prompt = false;
            } else if input == "n" || input == "no" {
                std::process::exit(0);
            } else {
                println!("Invalid input: {}", input);
            }
        }

        neutron::upgr_sys();
    } else {
        println!("At least one 3 arguments are required(2 found)");
        std::process::exit(1);
    }

    args_mod.remove(0);
    args_mod.remove(0); // remove non-important arguments(will be saved in imut_args)

    if imut_args[2].eq("search") {
        if neutron::search_package(&args_mod[1]) {
            println!(
                "Package: {0} was found in Elements' repository.",
                &args_mod[0]
            );
            println!("Use 'lmt install {0}' to install it.", &args_mod[0])
        } else {
            println!("Couldn't find '{0}' .", &args_mod[0])
        }
    }

    args_mod.dedup(); // remove duplicates

    if command.eq("install") || command.eq("in") && args_mod.len() == 1 {
        println!("Installing {0:?}", args_mod.join(" "));
    } else if command.eq("remove") || command.eq("rm") && args_mod.len() == 1 {
        println!("Removing: {0:?}", args_mod.join(" "));
    } else if command.eq("upgrade") || command.eq("up") && args_mod.len() == 1 {
        println!("Upgrading: {0:?}", args_mod.join(" "));
    } else if command.eq("install") || command.eq("in") && args_mod.len() != 1 {
        println!(
            "Installing {0} packages: {1:?}",
            args_mod.len(),
            args_mod.join(" ")
        );
    } else if command.eq("remove") || command.eq("rm") && args_mod.len() != 1 {
        println!(
            "Removing {0} packages: {1:?}",
            args_mod.len(),
            args_mod.join(" ")
        );
    } else if command.eq("upgrade") || command.eq("up") && args_mod.len() != 1 {
        println!(
            "Upgrading {0} packages: {1:?}",
            args_mod.len(),
            args_mod.join(" ")
        );
    }

    let mut in_prompt = true;

    while in_prompt {
        print!("Continue? [Y/n] ");
        io::stdout().flush().unwrap(); // flush stdout

        let mut input = String::new(); // answer to the "Continue?" prompt
        io::stdin().read_line(&mut input).unwrap(); // take input
        input = input.to_lowercase();

        if input.eq("n\n") || input.eq("no\n") {
            // if answer is "n" or "no"
            println!("Aborting.");
            std::process::exit(0);
        } else if !input.eq("y\n") && !input.eq("yes\n") && !input.eq("\n") {
            // if answer is neither "y" nor "yes" nor nothing
            println!("Input Error: Unknown answer.")
        } else {
            in_prompt = false;
        }
    }

    let mut pkgs_done = 0;
    
    if imut_api::getmode() {
        imut_api::enterrw();
    }
    
    while pkgs_done < args_mod.len() {
        if command.eq("install") || command.eq("in") {
            println!(
                "Install package: {0} {1}/{2}",
                &args_mod[pkgs_done],
                pkgs_done + 1,
                args_mod.len()
            );
            if neutron::inst_package(&args_mod[pkgs_done], "") == 128 {
                println!("Package already installed. Skipping...");
            };
        } else if command.eq("remove") || command.eq("rm") {
            println!(
                "Removing package: {0} {1}/{2}",
                &args_mod[pkgs_done],
                pkgs_done + 1,
                args_mod.len()
            );
            if neutron::rm_package(&args_mod[pkgs_done]) == 128 {
                println!("Package not installed. Skipping...");
            };
        } else if command.eq("upgrade") || command.eq("up") {
            println!(
                "Updating package: {0} {1}/{2}",
                &args_mod[pkgs_done],
                pkgs_done + 1,
                args_mod.len()
            );
            if neutron::up_package(&args_mod[pkgs_done]) == 128 {
                println!("Package not installed. Skipping...");
            };
        }

        pkgs_done += 1;

    }

    // println!("{}", imut_api::enterro());
	if imut_api::getmode() {
        imut_api::enterro();
    }
}

fn help(exit_code: i32) {
    println!("usage: lmt <action> <package>");
    println!("List of Main Commands:");
    println!("  install: Install a package");
    println!("  remove: Remove a package");
    println!("  update: Update all packages");
    println!("  search: Search for a package");
    println!("  help: Show this help message");
    std::process::exit(exit_code);
}
