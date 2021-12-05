import os
import sys
from colorama import Fore

ntgrepo = ''
customrepo1 = ''
customrepo2 = ''
current_pkgs = ''
invalid_pkg = 0
pkg = ""
ver = 0
if os.geteuid() != 0:
    print(Fore.RED + "Fatal Error: You must run Elements as root.")
    sys.exit()
else:
    pkgs = open("/etc/elements/pkgs", 'a')


def pkgscheck():
    if os.path.exists("/etc/elements/pkgs"):
        pass
    else:
        file = open("/etc/elements/pkgs", 'w')
        file.close()


def install_pkg():
    # Package Check 2
    global invalid_pkg
    pkgvalid = os.system("ls /etc/elements/repos/" + ntgrepo + "/ | grep " + pkg + " " + "> /dev/null")
    inrepo = 'Nitrogen'
    origin = 'Nitrogen'
    if pkgvalid != 0:
        pkgvalid = os.system("ls /etc/elements/repos/" + customrepo1 + "/ | grep " + pkg + " " + "> /dev/null")
        inrepo = customrepo1
        origin = 'Custom'
    if pkgvalid != 0:
        pkgvalid = os.system("ls /etc/elements/repos/" + customrepo2 + "/ | grep " + pkg + " " + "> /dev/null")
        inrepo = customrepo2
        origin = 'Custom'
    if pkgvalid != 0:
        # As a backup use pacman
        print(Fore.RED + "Couldn't find in Nitrogen Repository." + ' ' + "Defaulting to pacman." + Fore.WHITE)
        invalid_pkg = os.system('pacman -S ' + pkg)
        if invalid_pkg != 0:
            print(Fore.RED + "Program Terminated. Invalid/Nonexistent Package." + Fore.WHITE)
            sys.exit()

    if pkg in current_pkgs:
        print(pkg + " already installed.")
    elif invalid_pkg == 0:
        print("Installing: " + pkg)
        print("Repository: " + origin + '/' + inrepo)

        def prompt():
            x = str(input(Fore.GREEN + "Y" + Fore.WHITE + "/" + Fore.RED + "n" + ' ' + Fore.WHITE))
            if x in ['y']:
                os.system("bash /etc/elements/repos/" + inrepo + '/' + pkg + "/build")
                print("----------------------------")
                print("Installed " + pkg + " successfully")
                pkgs.write(" " + pkg)
            elif x in ['n']:
                sys.exit()
            else:
                print(Fore.RED + '"' + x + '"' + " is not understood." + Fore.WHITE)
                prompt()

        prompt()
