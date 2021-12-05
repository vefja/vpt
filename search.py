import os
import install
from colorama import Fore


def search_pkg():
    searched_item = int(os.system("ls /etc/elements/repos/nitrogen/ | grep " + install.pkg + " > /dev/null"))
    if searched_item != 0:
        print(Fore.RED + install.pkg + ' not found in Nitrogen Repository."' + Fore.WHITE)
    else:
        print(install.pkg + " found in Nitrogen repository.")
        print("Do a " + '"lmt --add ' + install.pkg + '"' + " to install it.")
