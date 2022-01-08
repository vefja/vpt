import os, sys
from colorama import Fore

ver = "one α"
## TODO: change next to stable when Elements One gets released
branch = "next"

def search_repository():
    global local_repo_contains
    global pacman
    pacman = False
    local_repo_contains = os.system("/etc/elements/search-repo " + sys.argv[2] + " >> /dev/null")
    if local_repo_contains != 0:
        if os.system("pacman -Ss " + " ".join(sys.argv[2:]) + " >> /dev/null") != 0:
            sys.exit()
        else:
            pacman = True
    else:
        local_repo_contains = os.popen("/etc/elements/search-repo " + sys.argv[2]).read()
        local_repo_contains = local_repo_contains.replace('\n', ' ')
        local_repo_contains = local_repo_contains.split(' ', 1)
        local_repo_contains = local_repo_contains[0]
        print(local_repo_contains)


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
    search_repository()
    print("The following packages will be installed:")
    print(" " + ", ".join(sys.argv[2:]))
    prompt = input(
        "Do you wish to continue? " + "[" + Fore.GREEN + "Y" + Fore.RESET + "/" + Fore.RED + "n" + Fore.RESET + "] ")
    if prompt in ["y", "yes", ""]:
        if pacman is True:
            os.system("pacman -S --noconfirm " + sys.argv[2])
            sys.exit()
        os.system("/etc/elements/repos/" + local_repo_contains + "/" + sys.argv[2] + "/build")

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
    if prompt in ["y", "yes", ""]:
        print()
        ## TODO: Add removal scripts
    elif prompt in ["n", "no"]:
        sys.exit()

elif sys.argv[1] in "search":
    if os.system("/etc/elements/search-repo " + sys.argv[2] + " >> /dev/null") == 0:
        print(sys.argv[2] + " found.")
    else:
        print("Could not find " + sys.argv[2])

elif sys.argv[1] in "update":
    os.system("wget https://raw.githubusercontent.com/NitrogenLinux/elements/" + branch + "/Elements.py")
## TODO: Complete update script
## TODO: Make update script use the binary file instead of the source code

elif sys.argv[1] in "show":
    if os.system("./search " + sys.argv[2] + " >> /dev/null") != 0:
        print(sys.argv[2] + " not found.")
    else:
        print("Package: " + sys.argv[2])
        print("Repository: " + os.popen("./search-repo " + sys.argv[2]).read())
else:
    print(sys.argv[1] + ": Command Not found.")
