import os
import install
from colorama import Fore


def search_pkg():
    searched_item = int(os.system("ls ~/.lmt-repo | grep " + install.pkg + " > /dev/null"))
    if searched_item != 0:
        print(Fore.RED + install.pkg + ' not found. You may try to refresh the repository by doing "lmt --refresh"' + Fore.WHITE)
    else:
        print(install.pkg + " found in current repository.")
        print("Do a " + '"lmt --add ' + install.pkg + '"' + " to install it.")
