#!/usr/bin/python3
import os, sys
from colorama import Fore
import requests

ver = "One"
pkg_number = 2

def protect_packages():
    if sys.argv[pkg_number] in ["gnome", "linux-lts", "xbps", "elements"]:
        print(Fore.RED + "You are trying to remove a protected package." + Fore.RESET)
        print("Doing so may damage your system.")
        print('Type "I understand the possible consequences of this action." if you wish to continue.')
        removal_ok = input(": ")
        if removal_ok == "I understand the possible consequences of this action.":
            pass
        else:
            print(Fore.RED + "Error: Could not remove package: Removing " + sys.argv[pkg_number] + " is forbidden." + Fore.RESET)
            sys.exit()

def search_repository():
    global in_repository
    global use_xbps
    global find_success
    global pkgs_left
    global pkg_number
    use_xbps = False # declare xbps as false for the next package(can change)
    in_repository = os.system("/etc/elements/search-repo " + sys.argv[pkg_number] + " >> /dev/null") # search for the package using search-repo
    find_success = in_repository # define success
    if in_repository != 0: # if package not found in the repository, use xbps
        find = requests.get('https://github.com/void-linux/void-packages/tree/master/srcpkgs/' + sys.argv[pkg_number])
        if find.status_code != 200: # search in the Void repositories for the package
           sys.exit()
        use_xbps = True
        in_repository = "Void Linux"
    else:
        in_repository = os.popen("/etc/elements/search-repo " + sys.argv[pkg_number]).read() # if package is found in the Nitrogen repositories, find the path
        in_repository = in_repository.replace('\n', ' ') # replace ln with nothing
        in_repository = in_repository.split(' ', 1)
        in_repository = in_repository[0]
        if os.system("ls /etc/elements/repos/" + in_repository + '/' + sys.argv[pkg_number] + " >> /dev/null") != 0:
            print(sys.argv[pkg_number] + " does not exist.") # package not found error
            sys.exit()


def chk_root():
    if os.geteuid() != 0:
        print("Root is required to run " + sys.argv[1])
        sys.exit()



if len(sys.argv[1:]) == 0:
    print("Elements " + ver)
    print("Usage: lmt command")
    print()
    print("Commands: ")
    print("install - install a package")
    print("remove - remove a package")
    print("update - update Nitrogen")
    print("refresh - refresh the Nitrogen Repository")
    print("search - search a package")
    sys.exit()
else:
    if sys.argv[1] in ["install", "remove", "search", "show"]:
        if sys.argv[2:]:
            pass
        else:
            print("Must Specify what package to " + sys.argv[1] + ".")
            sys.exit()

if sys.argv[1] == "install":
    chk_root() # check for root permissions
    search_repository() # search if package is available
    if len(sys.argv[2:]) != len(set(sys.argv[2:])):
        sys.argv[2:] = list(dict.fromkeys(sys.argv[2:]))
    print("The following packages will be installed:")
    print(" " + ", ".join(sys.argv[2:]))
    prompt = str(input("Do you wish to continue? " + "[" + Fore.GREEN + "Y" + Fore.RESET + "/" + Fore.RED + "n" + Fore.RESET + "] "))
    if prompt in ["y", "yes", ""]:
        pkgs_left = len(sys.argv[2:])
        while pkgs_left > 0:
            search_repository()
            print(Fore.GREEN + "Installing package " + Fore.YELLOW + str(pkg_number - 1) + Fore.WHITE + "/" + Fore.YELLOW + str(len(sys.argv[2:])) + Fore.WHITE)
            if use_xbps is True:
                os.system("xbps-install -Sy " + sys.argv[pkg_number])
                pkgs_left = pkgs_left - 1
            else:
                os.system("/etc/elements/repos/" + in_repository + "/" + sys.argv[pkg_number] + "/build")
                pkgs_left = pkgs_left - 1
                pkg_number = pkg_number + 1

    elif prompt in ["n", "no"]:
        print("Exit.")
        sys.exit()

elif sys.argv[1] == "remove":
    chk_root()
    protect_packages()
    search_repository()
    if len(sys.argv[2:]) != len(set(sys.argv[2:])):
        sys.argv[2:] = list(dict.fromkeys(sys.argv[2:]))
    print("The following packages will be removed:")
    print(" " + ", ".join(sys.argv[2:]))
    prompt = str(input("Do you wish to continue? " + "[" + Fore.GREEN + "Y" + Fore.RESET + "/" + Fore.RED + "n" + Fore.RESET + "] "))
    if prompt in ["y", "yes", ""]:
        pkgs_left = len(sys.argv[2:])
        while pkgs_left > 0:
            search_repository()
            print(Fore.GREEN + "Removing package " + Fore.YELLOW + str(pkg_number - 1) + Fore.WHITE + "/" + Fore.YELLOW + str(len(sys.argv[2:])) + Fore.WHITE)
            if use_xbps is True:
                os.system("xbps-remove -y " + sys.argv[pkg_number])
                pkgs_left = pkgs_left - 1
            else:
                os.system("/etc/elements/repos/" + in_repository + "/" + sys.argv[pkg_number] + "/remove")
                pkgs_left = pkgs_left - 1
                pkg_number = pkg_number + 1

    elif prompt in ["n", "no"]:
        print("Exit.")
        sys.exit()


elif sys.argv[1] == "search":
    search_repository()
    if os.system("/etc/elements/search " + sys.argv[2] + " >> /dev/null") != 0 and use_xbps == False:
        print(sys.argv[2] + " not found.")
    else:
        searched_item = os.popen("/etc/elements/search " + sys.argv[2]).read()
        searched_item = searched_item.replace('\n', ' ')
        searched_item = searched_item.split(' ', 1)
        searched_item = searched_item[0]
        print(searched_item + " found.")


elif sys.argv[1] == "update":
    chk_root()
    os.system("curl https://raw.githubusercontent.com/NitrogenLinux/elements/stable/lmt > lmt.src")
    os.system("mv lmt.src /usr/bin/")
    os.system("git clone https://github.com/tekq/elements-search.git")
    os.system("mv -vf elements-search/search-repo /etc/elements/")
    os.system("mv -vf elements-search/search /etc/elements/")
    os.system("rm -rvf elements-search")
    os.system("chmod a+x /etc/elements/search")
    os.system("chmod a+x /etc/elements/search-repo")
    os.system("chmod a+x /usr/bin/*")
    os.system("chmod -R a+x /etc/elements/repos/")
    os.system("xbps-install -Suy")

elif sys.argv[1] == "refresh":
    chk_root()
    if os.path.exists("/etc/elements/repos/Nitrogen"):
        os.system("cd /etc/elements/repos/Nitrogen/")
        os.system("git pull; cd - >> /dev/null")
    else:
        os.system("git clone https://github.com/NitrogenLinux/elements-repo /etc/elements/repos/Nitrogen")
    os.system("chmod -R a+x /etc/elements/repos/")

elif sys.argv[1] == "show":
    search_repository()
    if os.system("/etc/elements/search " + sys.argv[2] + " >> /dev/null") != 0 and use_xbps == False:
        print(sys.argv[2] + " not found.")
    else:
        print("Package: " + sys.argv[2])
        print("Repository: " + in_repository)
else:
    print(sys.argv[1] + ": Command Not found.")
