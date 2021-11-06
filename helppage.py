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
    if args in ["['add']", "['install']"]:
        print("Elements --add")
        print("--------------")
        print("Installs a package")
        print("--------------")
        print("Syntax: 'lmt --add package'")
    elif args in ["['del']", "['delete']", "['remove']", "['rm']"]:
        print("Elements --del")
        print("--------------")
        print("Removes a package")
        print("--------------")
        print("Syntax: 'lmt --delete package'")
    elif args in ["['ref']", "['refresh']"]:
        print("Elements --refresh")
        print("------------------")
        print("Refreshes/Re-Downloads Elements repository")
        print("------------------")
        print("Syntax: 'lmt --refresh'")
    elif args in ["['update']", "['U']"]:
        print("Elements --update")
        print("-----------------")
        print("Fully updates Elements and re-downloads it, after this, the repository will be refreshed")
        print("-----------------")
        print("Syntax: 'lmt --update'")
    elif args in ["['sr']", "['s']", "['search']"]:
        print("Elements --search")
        print("-----------------")
        print("Search a package, pretty simple huh?")
        print("-----------------")
        print("Syntax: 'lmt --search package'")
    elif args in ["['cfg-regen']"]:
        print("Elements --cfg-regen")
        print("--------------------")
        print("Regenerate the Elements configuration file")
        print("--------------------")
        print("Syntax: 'lmt --cfg-regen'")
    elif args in ["['list']", "['l']"]:
        print("Elements --list")
        print("---------------")
        print("Lists all installed packages")
        print("---------------")
        print("Syntax: 'lmt --list'")
    elif args in ["['configure']"]:
        print("Elements --configure")
        print("--------------------")
        print("Configure Elements and Nitrogen")
        print("--------------------")
        print("Syntax: 'lmt --configure'")
    elif args in ["['version']"]:
        print("Elements --ver")
        print("--------------")
        print("Prints current Elements version")
        print("--------------")
        print("Syntax: 'lmt --ver'")
        print("Elements-" + ver)
    elif args in ["['help']", "['h']"]:
        print("Elements --help")
        print("---------------")
        print("Shows general Help Page or ManPage of a specific command")
        print("---------------")
        print("Syntax: 'lmt --help'")
        print("or")
        print("Syntax: 'lmt --help command'")

def version():
    print("Elements " + ver)
