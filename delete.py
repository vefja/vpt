import install, os, sys, protect
from colorama import Fore

pkgs = open('/etc/elements/pkgs', 'r')
packages = pkgs.read()


def delete_pkg():
    protected = install.pkg_args[0] in protect.protected_package
    # Check if deleting package is protected
    if protected is True:
        print(Fore.RED + "The package you are trying to remove is protected" + Fore.WHITE)
        print(Fore.RED + "Continuing with your removal is not recommended" + Fore.WHITE)
        print("If you know what you are doing, type 'Yes, I want to proceed.' to remove this package")
        protected_confirm = input()
        if protected_confirm == "Yes, I want to proceed.":
            print(Fore.GREEN + "Continuing..." + Fore.WHITE)
        elif protected_confirm == "exit":
            sys.exit()
        else:
            print(Fore.RED + "Error: Trying to remove protected package, cannot continue.")
            sys.exit()
    # valid package check
    pkgvalid = os.system("ls /etc/elements/repos/Nitrogen/ | grep " + install.pkg_args + "> /dev/null")
    pkgvalid = os.system("ls /etc/elements/repos/" + install.ntgrepo + "/ | grep " + install.pkg_args + " " + "> /dev/null")
    inrepo = 'Nitrogen'
    origin = 'Nitrogen'
    if pkgvalid != 0:
        pkgvalid = os.system(
            "ls /etc/elements/repos/" + install.customrepo1 + "/ | grep " + install.pkg_args + " " + "> /dev/null")
        inrepo = install.customrepo1
        origin = 'Custom'
    if pkgvalid != 0:
        pkgvalid = os.system(
            "ls /etc/elements/repos/" + install.customrepo2 + "/ | grep " + install.pkg_args + " " + "> /dev/null")
        inrepo = install.customrepo2
        origin = 'Custom'
    if pkgvalid != 0:
        print(Fore.RED + "Couldn't find in Nitrogen Repository." + ' ' + "Defaulting to pacman." + Fore.WHITE)
        invalid_pkg = os.system("pacman -Rns " + install.pkg_args)
        if invalid_pkg != 0:
            print(Fore.RED + "Couldn't default to pacman" + Fore.WHITE)
            print("Program Terminated. Invalid/Nonexistent Package.")
    if pkgvalid == 0:
        print(Fore.WHITE + "Are you sure you want to uninstall " + install.pkg_args + "?")

        def prompt():
            x = str(input(Fore.GREEN + "Y" + Fore.WHITE + "/" + Fore.RED + "n" + ' ' + Fore.WHITE))
            if x in ['y']:
                print("Removing: " + install.pkg_args)
                os.system("cd /usr/src/" + install.pkg_args)
                os.system("make uninstall")
                os.system("rm -v /usr/bin/" + install.pkg_args)
                os.system("rm -rf -v /usr/src/" + install.pkg_args)
                afterrmpkgs = packages.replace(install.pkg_args, "")
                open('/etc/elements/pkgs', 'w').close()
                pkgs = open('/etc/elements/pkgs', 'a')
                pkgs.write(afterrmpkgs)
                print("Removed " + install.pkg_args + " successfully")
            elif x in ['n']:
                sys.exit()
            else:
                print(Fore.RED + '"' + x + '"' + " is not understood." + Fore.WHITE)
                prompt()

        prompt()
