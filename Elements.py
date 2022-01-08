import os, sys
from colorama import Fore

ver = "one β"
## TODO: change next to stable when Elements One gets released
branch = "next"

package_install = 2

def search_repository():
    global local_repo_contains
    global pacman
    global success
    global packages_to_install
    global package_install
    pacman = False
    local_repo_contains = os.system("/etc/elements/search-repo " + sys.argv[package_install] + " >> /dev/null")
    success = local_repo_contains
    if local_repo_contains != 0:
        ##if os.system("pacman -Ss " + " ".join(sys.argv[package_install]) + " >> /dev/null") != 0:
         ##   sys.exit()
        pacman = True
    else:
        local_repo_contains = os.popen("/etc/elements/search-repo " + sys.argv[package_install]).read()
        local_repo_contains = local_repo_contains.replace('\n', ' ')
        local_repo_contains = local_repo_contains.split(' ', 1)
        local_repo_contains = local_repo_contains[0]


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
        if sys.argv[2:]:
            pass
        else:
            print("Must Specify what package to " + sys.argv[1] + ".")
            sys.exit()

if sys.argv[1] in "install":
    chk_root()
    search_repository()
    if len(sys.argv[2:]) != len(set(sys.argv[2:])):
        sys.argv[2:] = list(dict.fromkeys(sys.argv[2:]))
    print("The following packages will be installed:")
    print(" " + ", ".join(sys.argv[2:]))
    prompt: str = input(
        "Do you wish to continue? " + "[" + Fore.GREEN + "Y" + Fore.RESET + "/" + Fore.RED + "n" + Fore.RESET + "] ")
    if prompt in ["y", "yes", ""]:
        packages_to_install = len(sys.argv[2:])
        while packages_to_install > 0:
            search_repository()
            if pacman is True:
                os.system("pacman -S --noconfirm " + sys.argv[package_install])
                packages_to_install = packages_to_install - 1
            else:
                os.system("/etc/elements/repos/" + local_repo_contains + "/" + sys.argv[package_install] + "/build")
                packages_to_install = packages_to_install - 1
                package_install = package_install + 1
                print(packages_to_install)

    elif prompt in ["n", "no"]:
        print("Exit.")
        sys.exit()

elif sys.argv[1] in "remove":
    chk_root()
    search_repository()
    if len(sys.argv[2:]) != len(set(sys.argv[2:])):
        sys.argv[2:] = list(dict.fromkeys(sys.argv[2:]))
    print("The following packages will be removed:")
    print(" " + ", ".join(sys.argv[2:]))
    prompt: str = input(
        "Do you wish to continue? " + "[" + Fore.GREEN + "Y" + Fore.RESET + "/" + Fore.RED + "n" + Fore.RESET + "] ")
    if prompt in ["y", "yes", ""]:
        packages_to_install = len(sys.argv[2:])
        while packages_to_install > 0:
            search_repository()
            if pacman is True:
                os.system("pacman -Rns --noconfirm " + sys.argv[package_install])
                packages_to_install = packages_to_install - 1
            else:
                os.system("/etc/elements/repos/" + local_repo_contains + "/" + sys.argv[package_install] + "/remove")
                packages_to_install = packages_to_install - 1
                package_install = package_install + 1
                print(packages_to_install)

    elif prompt in ["n", "no"]:
        print("Exit.")
        sys.exit()


elif sys.argv[1] in "search":
    search_repository()
    if success != 0:
        print("Couldn't find " + sys.argv[2])
    else:
        searched = os.popen("/etc/elements/search " + sys.argv[2]).read()
        searched = searched.replace('\n', ' ')
        searched = searched.split(' ', 1)
        searched = searched[0]
        print(searched + " found.")


elif sys.argv[1] in "update":
    chk_root()
    os.system("wget https://raw.githubusercontent.com/NitrogenLinux/elements/" + branch + "/lmt")
    os.system("mv lmt /usr/bin/")
    os.system("git clone https://github.com/tekq/elements-search.git")
    os.system("mv -vf elements-search/search-repo /etc/elements/")
    os.system("mv -vf elements-search/search /etc/elements/")
    os.system("rm -rvf elements-search")
    os.system("chmod +x /etc/elements/{search,search-repo}")
    os.system("chmod +x /usr/bin/*")
    os.system("pacman -Syu")
## TODO: Complete update script
## TODO: Make update script use the binary file instead of the source code

elif sys.argv[1] in "show":
    if os.system("./search " + sys.argv[2] + " >> /dev/null") != 0:
        print(sys.argv[2] + " not found.")
    else:
        search_repository()
        print("Package: " + sys.argv[2])
        print("Repository: " + local_repo_contains)
else:
    print(sys.argv[1] + ": Command Not found.")
