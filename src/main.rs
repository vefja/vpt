use std::{env, fs, io, mem};
use std::io::{stdout, Write};
use std::process::exit;
use nix::unistd::getuid;
use colour::{red_ln, white};
use indicatif::{ProgressBar, ProgressStyle};
use vmod;
use immutability;

mod debug; // for cargo test 

fn main() {
    if (vmod::self_test() != 0)
    {
        red_ln!("One or several necessary files are missing. Cannot continue.");
        red_ln!("Run 'vpt --repair' to repair the installation. (online)");
        red_ln!("Run 'vpt --repair-offline' to repair the installation. (offline)");
    }

    let mut args: Vec<String> = env::args().collect(); // args_mod that can be modified
    let imut_args: Vec<String> = env::args().collect(); // immutable args_mod for other things

    let mut debug_mode: bool = false;


    if imut_args.len() >= 2 {
        let command = &imut_args[1].to_lowercase();

        if command == "--repair" {
            println!("repairing online");
        }
        else if command == "--repair-offline" {
            println!("repairing offline");
        }

        if command == "install"
            || command == "in"
            || command == "remove"
            || command == "rm"
            || command == "upgrade"
            || command == "up"
            || command == "search"
            || command == "se"
        {
            #[cfg(not(debug_assertions))]
            if vmod::check_option("snapshots") {
                vmod::new_snapshot("pre", &imut_args[1]);
            }
        
            #[cfg(not(debug_assertions))]
            if !getuid().to_string().eq("0") {
                red_ln!("You must be root to use this command!");
                std::process::exit(1);
            }

            #[cfg(not(debug_assertions))]
            if vmod::self_test() == 999 {
                red_ln!("One or more necessary files are missing. Cannot continue.")
            }

            #[cfg(debug_assertions)]
            println!("Running in debug mode. You're free to do whatever")

        } else if command.eq("help") || command.eq( "help") {
            help(0);
        } else {
            red_ln!("Invalid operation: {}", command);
            std::process::exit(1);
        }
    } else {
        red_ln!("Error: At least one 2 arguments are required(0 found)");
        std::process::exit(0);
    }

    let command = &imut_args[1].to_lowercase(); // redeclare in main

    let mut inst_path: String = "".to_owned(); 

    if imut_args.len() >= 3 {
        for i in 2..args.len() {
            if args[i].contains("--root=") {
                inst_path = args[i].replace("--root=", "");
                break;
            }

            if args[i].is_empty() {
                // Throw error if "" is passed as argument
                red_ln!("Error: Argument cannot be equal to nothing");
                std::process::exit(512); // Error 512 for invalid arguments
            }

            if args[i].contains(' ') {
                // Throw error if package name contains space
                red_ln!("Error: Argument cannot contain space");
                std::process::exit(512);
            }

            if args[i].contains('.') || args[i].contains('/') {
                red_ln!("Error: Package name cannot contain '{}'", args[i]);
                std::process::exit(512);
            }

            if vmod::get_pkg_version(args[i].as_str()).is_empty() {
                red_ln!("Error: Package '{}' not found in repository", args[i]);
                std::process::exit(1);
            }

            if !vmod::check_option("remove_protected")
                && command.eq("remove")
                && [
                "vpt",
                "gnome-core",
                "gnome",
                "linux",
                "xbps",
                "mutter",
                "kernel",
            ] // kernel - Vefjiaw OS's kernel
                .contains(&&*args[i])
            {
                red_ln!(
                    "Cannot remove '{0}': Package is required by system.",
                    args[i]
                );
                std::process::exit(128);
            }

            if command.eq("remove") {
                let pkglist = vmod::list_packages();

                let tmp = pkglist.split(' ');

                let all_packages: Vec<&str> = tmp.collect();

                let mut package_exists = false;

                for j in all_packages {
                    if j.eq(args[i].as_str()) {
                        red_ln!("Error: Package {} not installed.", args[i].as_str());
                        exit(120);
                    }
                }
            }
        }
    } else if command == "upgrade" || command == "up" {
        let mut prompt = true;
        while prompt {
            print!("[Y/n] ");
            stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            if input == "y" || input == "yes" || input.is_empty() {
                prompt = false;
            } else if input == "n" || input == "no" {
                std::process::exit(0);
            } else {
                red_ln!("Invalid input: {}", input);
            }
        }

        upgrade_system();
    } else {
        red_ln!("At least 3 arguments are required(2 found)");
        white!();
        exit(1);
    }

    args.remove(0);
    args.remove(0); // remove unneeded args in order to change args to pkg_args

    let mut pkg_args = args.clone();
    drop(args); // drop the old args

    if imut_args[2].eq("search") {
        if vmod::search_package(&pkg_args[0]) {
            println!(
                "{0}-{1}",
                &pkg_args[0],
                vmod::get_pkg_version(&pkg_args[0])
            );
            println!("Use 'vpt install {0}' to install it.", &pkg_args[0])
        } else {
            red_ln!("Error: '{0}': No package found.", &pkg_args[0])
        }
    }

    pkg_args.dedup(); // remove duplicates

    if command.eq("install") || command.eq("in") && pkg_args.len() == 1 {
        println!("Installing: {0:?}", pkg_args.join(" "));
    } else if command.eq("remove") || command.eq("rm") && pkg_args.len() == 1 {
        println!("Removing: {0:?}", pkg_args.join(" "));
    } else if command.eq("upgrade") || command.eq("up") && pkg_args.len() == 1 {
        println!("Upgrading: {0:?}", pkg_args.join(" "));
    } else if command.eq("install") || command.eq("in") && pkg_args.len() != 1 {
        println!(
            "Installing {0} packages: {1:?}",
            pkg_args.len(),
            pkg_args.join("\n")
        );
    } else if command.eq("remove") || command.eq("rm") && pkg_args.len() != 1 {
        println!(
            "Removing {0} packages: {1:?}",
            pkg_args.len(),
            pkg_args.join("\n")
        );
    } else if command.eq("upgrade") || command.eq("up") && pkg_args.len() != 1 {
        println!(
            "Upgrading {0} packages: {1:?}",
            pkg_args.len(),
            pkg_args.join("\n")
        );
    }

    let mut in_prompt = true;

    while in_prompt {
        print!("(Y/n) ");
        stdout().flush().unwrap(); // flush stdout

        let mut input = String::new(); // answer to the y/n prompt
        io::stdin().read_line(&mut input).unwrap(); // take input
        input = input.to_lowercase();

        if input.eq("n\n") || input.eq("no\n") {
            // if answer is "n" or "no"
            println!("Aborting...");
            std::process::exit(0);
        } else if !input.eq("y\n") && !input.eq("yes\n") && !input.eq("\n") {
            // if answer is neither "y" nor "yes" nor nothing
            red_ln!("Input Error: Invalid answer.")
        } else { // if answer if "y", "yes" or nothing
            in_prompt = false;
        }
    }

    let mut pkgs_done = 0;

	let orig_mode = immutability::getmode(); // save orig mode (so it doesn't constantly check)
  
    if orig_mode {
        immutability::enterrw();
    }

    let progress = ProgressBar::new(pkg_args.len() as u64);

    while pkgs_done < pkg_args.len() {
        progress.set_position(pkgs_done as u64);
        if command.eq("install") || command.eq("in") {
            vmod::install_tar(&pkg_args[pkgs_done], &inst_path, inst_path.is_empty(), false);
        } else if command.eq("remove") || command.eq("rm") {
            println!(
                "Removing package: {0} {1}/{2}",
                &pkg_args[pkgs_done],
                pkgs_done + 1,
                pkg_args.len()
            );
            vmod::remove_tar(&pkg_args[pkgs_done]);

        } else if command.eq("upgrade") || command.eq("up") {
            println!(
                "Updating package: {0} {1}/{2}",
                &pkg_args[pkgs_done],
                pkgs_done + 1,
                pkg_args.len()
            );
            vmod::install_tar(&pkg_args[pkgs_done], "", false, true);
            }

        pkgs_done += 1;
    }

    progress.set_position(pkg_args.len() as u64);

	if orig_mode {
        immutability::enterro();
    }

    mem::drop(orig_mode);
}

fn help(exit_code: i32) {
    println!("usage: vpt <action> <package>");
    println!("Use 'man vpt' to check all available commands");
    std::process::exit(exit_code);
}

fn upgrade_system() -> i32 {
    vmod::download_pkglist();

    let pkglist = vmod::list_packages();

    let tmp = pkglist.split(' ');

    let mut all_pkgs: Vec<_> = tmp.collect();

    all_pkgs.remove(0);

    for i in all_pkgs.iter() {
        if !vmod::compare_old_to_new(i) {
            vmod::install_tar(i, "", false, true);
        }
    }

    return 0;
}

fn resolve_conflict(conflict: &str) -> i32 {
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
    }
}
