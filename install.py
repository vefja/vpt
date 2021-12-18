import os
import sys
from colorama import Fore

import cfg

disable = False
test = False
ntgrepo = ''
customrepo1 = ''
customrepo2 = ''
current_pkgs = ''
invalid_pkg = 0
pkg_args = ""
ver = 0


def pkgscheck():
    if os.path.exists("/etc/elements/pkgs"):
        pass
    else:
        file = open("/etc/elements/pkgs", 'w')
        file.close()
    pkgs = open("/etc/elements/pkgs", 'a')

def install_pkg():
    if disable is True:
        print(Fore.RED + "Debug: Enabled" + Fore.WHITE)
        print(Fore.RED + "Using Elements in debug mode is not recommended." + Fore.WHITE)
    else:
        print(pkg_args)
        # Package Check 2
        global invalid_pkg
        pkgvalid = os.system("ls /etc/elements/repos/" + ntgrepo + "/ | grep " + pkg_args + " " + "> /dev/null")
        inrepo = 'Nitrogen'
        origin = 'Nitrogen'
        if cfg.repos[1]:
            if pkgvalid != 0:
                pkgvalid = os.system(
                    "ls /etc/elements/repos/" + customrepo1 + "/ | grep " + pkg_args + " " + "> /dev/null")
                inrepo = customrepo1
                origin = 'Custom'
        if cfg.repos[2]:
            if pkgvalid != 0:
                pkgvalid = os.system(
                    "ls /etc/elements/repos/" + customrepo2 + "/ | grep " + pkg_args + " " + "> /dev/null")
                inrepo = customrepo2
                origin = 'Custom'

        if pkgvalid != 0:
            # As a backup use pacman
            print(pkg_args)
            if cfg.pm_compat is True:
                print(Fore.RED + "Couldn't find in Nitrogen Repository." + ' ' + "Defaulting to pacman." + Fore.WHITE)
                invalid_pkg = os.system('pacman -S ' + pkg_args)
            else:
                print("Couldn't default to pacman.'")
            if invalid_pkg != 0:
                print(Fore.RED + "Program Terminated. Invalid/Nonexistent Package." + Fore.WHITE)
                if test is False:
                    sys.exit()

        if test is False:
            if pkg_args in current_pkgs:
                print(pkg_args + " already installed.")
            elif invalid_pkg == 0:
                print("Installing: " + pkg_args)
                print("Repository: " + origin + '/' + inrepo)

                def prompt():
                    x = str(input(Fore.GREEN + "Y" + Fore.WHITE + "/" + Fore.RED + "n" + ' ' + Fore.WHITE))
                    if x in ['y']:
                        os.system("bash /etc/elements/repos/" + inrepo + '/' + pkg_args + "/build")
                        print("----------------------------")
                        print("Installed " + pkg_args + " successfully")
                        pkgs.write(" " + pkg_args)
                    elif x in ['n']:
                        sys.exit()
                    else:
                        print(Fore.RED + '"' + x + '"' + " is not understood." + Fore.WHITE)
                        prompt()

                prompt()
        else:
            print("Running in Test mode.")
