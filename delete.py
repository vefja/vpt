import install
import os
import sys
from colorama import Fore

pkgs = open('/etc/elements/pkgs', 'r')
packages = pkgs.read()


def delete_pkg():
    # valid package check
    pkgvalid = os.system("ls /etc/elements/repos/Nitrogen/ | grep " + install.pkg + "> /dev/null")
    pkgvalid = os.system("ls /etc/elements/repos/" + install.ntgrepo + "/ | grep " + install.pkg + " " + "> /dev/null")
    inrepo = 'Nitrogen'
    origin = 'Nitrogen'
    if pkgvalid != 0:
        pkgvalid = os.system(
            "ls /etc/elements/repos/" + install.customrepo1 + "/ | grep " + install.pkg + " " + "> /dev/null")
        inrepo = install.customrepo1
        origin = 'Custom'
    if pkgvalid != 0:
        pkgvalid = os.system(
            "ls /etc/elements/repos/" + install.customrepo2 + "/ | grep " + install.pkg + " " + "> /dev/null")
        inrepo = install.customrepo2
        origin = 'Custom'
    if pkgvalid != 0:
        print(Fore.RED + "Couldn't find in Nitrogen Repository." + ' ' + "Defaulting to pacman." + Fore.WHITE)
        invalid_pkg = os.system("pacman -Rns " + install.pkg)
        if invalid_pkg != 0:
            print(Fore.RED + "Couldn't default to pacman" + Fore.WHITE)
            print("Program Terminated. Invalid/Nonexistent Package.")
    if pkgvalid == 0:
        print(Fore.WHITE + "Are you sure you want to uninstall " + install.pkg + "?")

        def prompt():
            x = str(input(Fore.GREEN + "Y" + Fore.WHITE + "/" + Fore.RED + "n" + ' ' + Fore.WHITE))
            if x in ['y']:
                print("Removing: " + install.pkg)
                os.system("cd /usr/src/" + install.pkg)
                os.system("make uninstall")
                os.system("rm -v /usr/bin/" + install.pkg)
                os.system("rm -rf -v /usr/src/" + install.pkg)
                afterrmpkgs = packages.replace(install.pkg, "")
                open('/etc/elements/pkgs', 'w').close()
                pkgs = open('/etc/elements/pkgs', 'a')
                pkgs.write(afterrmpkgs)
                print("Removed " + install.pkg + " successfully")
            elif x in ['n']:
                sys.exit()
            else:
                print(Fore.RED + '"' + x + '"' + " is not understood." + Fore.WHITE)
                prompt()

        prompt()
