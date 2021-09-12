import os
import install
from colorama import Fore


def search_pkg():
    searched_item = int(os.system("ls ~/.lmt-repo | grep " + install.pkg + " > /dev/null"))
    if searched_item == 256:
        print(Fore.RED + install.pkg + " not in current repo or not in repo. If you know this package is in the repo "
                                       "then I would "
                                       "recommend doing a lmt --refresh" + Fore.WHITE)
    else:
        print(install.pkg + " found.")
