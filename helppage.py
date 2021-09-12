from colorama import Fore
ver = "0.0.5"


def helppage():
    print(Fore.CYAN + "Elements " + ver + Fore.WHITE)
    print("--------------")
    print("Commands: ")
    print("--add/-a: " + "install packages")
    print("--del/-d: " + "remove packages")
    print("--ref/-r: " + "refresh the repository")
    print("--up/-U: " + "update")
    print("--sr/-s: " + "search a package")
    print("--cfg-regen: " + "regenerate CFLAGS and CXXFLAGS")
    print("--list/-l: " + " list all installed packages")
    print('--ver/-v: ' + 'show version')
    print("--help/-h: " + "show this menu")


def version():
    print("Elements " + ver)
