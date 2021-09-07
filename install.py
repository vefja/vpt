import os

pkg=""
def install_pkg():
    os.system("~/.lmt-repo/" + pkg + "-package")
    print("----------------------------")
    print("Installed " + pkg + " successfully")