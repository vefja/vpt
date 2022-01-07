import os
import sys
from colorama import Fore

ver = "one α"
## TODO: change next to stable when Elements One gets released
branch = "next"

def search_repository():
    local_repo_contains = os.system("/etc/elements/search-repo " + sys.argv[2])
    if local_repo_contains != 0:
        if os.system("pacman -S " + " ".join(sys.argv[2:])) != 0:
            print("BRUH")


def chk_root():
    if os.geteuid() != 0:
        print("Root is required to run " + sys.argv[1])
        sys.exit()


if not sys.argv[1:]:
    print("Elements " + ver)
    print("Usage: lmt command")
    print()
    print("Commands: ")
    print("install - install a package")
    print("remove - remove a package")
    print("update - update Nitrogen")
    print("search - search a package")
    sys.exit()
else:
    if sys.argv[1] in ["install", "remove", "search", "show"]:
        if not sys.argv[2:]:
            print("Must Specify what package to " + sys.argv[1] + ".")
            sys.exit()
if sys.argv[1] in "install":
    chk_root()
    print("The following packages will be installed:")
    print(" " + ", ".join(sys.argv[2:]))
    prompt = input(
        "Do you wish to continue? " + "[" + Fore.GREEN + "Y" + Fore.RESET + "/" + Fore.RED + "n" + Fore.RESET + "] ")
    if prompt in ["y", "yes"]:
        print()
        ## TODO: Add install scripts
    elif prompt in ["n", "no"]:
        print("Exit.")
        sys.exit()

elif sys.argv[1] in "remove":
    chk_root()
    print("The following packages will be removed:")
    print(" " + ", ".join(sys.argv[2:]))
    prompt = input(
        "Do you wish to continue? " + "[" + Fore.GREEN + "Y" + Fore.RESET + "/" + Fore.RED + "n" + Fore.RESET + "] ")
    if prompt in ["y", "yes"]:
        print()
        ## TODO: Add removal scripts
    elif prompt in ["n", "no"]:
        sys.exit()

elif sys.argv[1] in "search":
    os.system("/etc/elements/search " + sys.argv[2])
    search_repository()
elif sys.argv[1] in "update":
    os.system("wget https://raw.githubusercontent.com/NitrogenLinux/elements/" + branch + "/Elements.py")
elif sys.argv[1] in "show":
    print()
    ## TODO: Add Show Package Mechanism
else:
    print(sys.argv[1] + ": Command Not found.")
