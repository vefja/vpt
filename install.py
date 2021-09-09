import os
import sys

pkg = ""


def install_pkg():
    print("Are you sure you want to install " + pkg + "?")
    x = str(input("Y/n "))
    if x in ['y']:
        os.system("~/.lmt-repo/" + pkg + "-package")
        print("----------------------------")
        print("Installed " + pkg + " successfully")
    elif x in ['n']:
        sys.exit()
    else:
        print('"' + x + '"' + " is not a valid command.")
        install_pkg()
