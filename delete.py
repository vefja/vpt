import install
import os
import sys


def delete_pkg():
    print("Are you sure you want to uninstall " + install.pkg + "?")
    x = str(input("Y/n "))
    if x in ['y']:
        print("Removing: " + install.pkg)
        os.system("cd /usr/src/" + install.pkg)
        os.system("make uninstall")
        os.system("rm -v /usr/bin/" + install.pkg)
        os.system("rm -rf -v /usr/src/" + install.pkg)
        print("Removed " + install.pkg + " successfully")
    elif x in ['n']:
        sys.exit()
    else:
        print('"' + x + '"' + " is not a valid command.")
        delete_pkg()
