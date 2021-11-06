import install
import os
import sys
from colorama import Fore

pkgs = open('/etc/elements/pkgs', 'r')
packages = pkgs.read()


def delete_pkg():
    # valid package check
    pkgvalid = os.system("ls /etc/elements/repos/nitrogen/ | grep " + install.pkg + "> /dev/null")
    if pkgvalid == 256:
        print(Fore.RED + install.pkg + " is not a valid package." + Fore.WHITE)
    else:
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
