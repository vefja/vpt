import os
import install
from colorama import Fore

# TODO: change prompt of found, and for not found

def search_pkg():
    searched_item = int(os.system("ls ~/.lmt-repo | grep " + install.pkg + " > /dev/null"))
    if searched_item == 256:
        print(Fore.RED + install.pkg + ' is not in the local repository, if the package shows on GitHub I would advise to run a lmt --refresh' + Fore.WHITE)
    else:
        print(install.pkg + " found.")
