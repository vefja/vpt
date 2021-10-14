from colorama import Fore
ver = "Next Branch"

args = ''

def helppage():
    print(Fore.CYAN + "Elements " + ver + Fore.WHITE)
    print("--------------")
    print("Commands: ")
    print("--add/-a: " + "install packages")
    print("--del/-d: " + "remove packages")
    print("--ref/-r: " + "refresh the repository")
    print("--up/-U: " + "update")
    print("--sr/-s: " + "search a package")
    print("--cfg-regen: " + "regenerate cfg.py file")
    print("--list/-l: " + " list all installed packages")
    print("--configure/--cfg: " + " configure Elements & Nitrogen")
    print('--ver/-v: ' + 'show version')
    print("--help/-h: " + "show this menu")
    print("------------")
    print("You can always check what every command does in detail by doing 'lmt --help command'")


def man_command():
    # TODO: add more options in here, for now its quite useless but will have more options
    if args in ["['add']"]:
        print("Elements --add")
        print("--------------")
        print("Installs a package")
        print("--------------")
        print("Example Command: 'lmt --add neofetch'")
    elif args in ["['del']", "['delete']"]:
        print("Elements --del")
        print("--------------")
        print("Removes a package")
        print("--------------")
        print("Example Command: 'lmt --delete neofetch'")
    elif args in ["['ref']", "['refresh']"]:
        print("Elements --refresh")
        print("------------------")
        print("Refreshes/Re-Downloads Elements repository")
        print("------------------")
        print("Example Command: 'lmt --refresh'")


def version():
    print("Elements " + ver)
