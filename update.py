from colorama import Fore
import sys
import os
import helppage


def refresh():
    print("Remove repo")
    os.system("rm -rf ~/.lmt-repo")
    print("Reclone it")
    os.system("git clone https://github.com/tekq/elements-repo.git ~/.lmt-repo")
    print("Checking for updates")
    currentver = os.popen('cat ~/.lmt-repo/.current-ver').read()
    ver = helppage.ver
    print("Local Elements version: " + ver)
    print("Newest Elements version: " + currentver)


def update():
    # delete current elements files
    os.system("rm -rvf /usr/share/elements")
    # replace them with the latest and greatest
    os.system("git clone https://github.com/NitrogenLinux/elements.git")
    os.system("mv -v elements /usr/share/")
    # refresh repositories
    refresh()   
    print("Elements Update Complete!")


def cfgregen():
    print("Doing this will remove your old cc.cfg, are you sure?")

    def prompt():
        x = str(input(Fore.GREEN + "Y" + Fore.WHITE + "/" + Fore.RED + "n" + ' ' + Fore.WHITE))
        if x in ['y']:
            print("Regenerating Config...")
            os.system("curl https://raw.githubusercontent.com/NitrogenLinux/elements/main/cc.cfg > "
                      "/usr/share/elements/cc.cfg")
        elif x in ['n']:
            sys.exit()
        else:
            print(Fore.RED + '"' + x + '"' + " is not a valid command." + Fore.WHITE)
            prompt()
    prompt()
