import delete
import sys
import time
import install as add
import update as upd
import search as sr
import helppage as help
import os
import urllib.request
import ntgcfg
from colorama import Fore

# Get arguments
full_cmd_arguments = sys.argv
# Declare some things before
packages = ''
ok_post = False
debug = False
cc = 'none'
cxx = 'none'

# look for P.O.S.T. disabling arguments
DebugArgs = full_cmd_arguments[3:]

# Elements P.O.S.T.
# -----------------
# P.O.S.T., is the Power On Self Test of Elements
# It tests for every important feature
# But to make debugging easy on other platforms there is a '--debug' flag at the end
# '--debug' can also remove the need for the 'cfg.py' file, which is not recommended


if str(DebugArgs) not in ["['--debug']"]:
    print(Fore.GREEN + "Post Enabled" + Fore.WHITE)

    # P O S T, do all checks to be sure Elements can go ahead
    if os.geteuid() != 0:
        print(Fore.RED + "Fatal Error: You must run Elements as root." + Fore.WHITE)
        sys.exit()
    else:
        ok_post = True


else:
    debug = True
if ok_post is True:
    pkgs = open('/etc/elements/pkgs', 'r')
    packages = pkgs.read()
    add.current_pkgs = packages
    print(Fore.GREEN + "Pkgs Loaded")

pkg_num = len(packages.split())
ntgcfg.Debug = DebugArgs

# Check for Configuration File
if debug is False:
    cfg_load = os.system("ls /etc/elements | grep cfg.py > /dev/null")
else:
    cfg_load = 1
# In case config file isn't found(command returns an answer other an 0), throw an error
if cfg_load != 0:
    print(Fore.RED + "Fatal Error: Config File Not Found." + Fore.WHITE)
    if DebugArgs:
        print(Fore.RED + "Continuing with Errors" + Fore.WHITE)
    else:
        sys.exit()
else:
    # If successful import the config file
    sys.path.insert(0, '/etc/elements/cfg.py')
    import cfg

    if cfg.custom_repos is True:
        print(Fore.GREEN + "Setting Repositories", end='')
        sys.stdout.flush()
        time.sleep(0.3)
        print(".", end='')
        sys.stdout.flush()
        time.sleep(0.2)
        print(".", end='')
        sys.stdout.flush()
        time.sleep(0.3)
        print("." + Fore.WHITE)
        repos = cfg.repos
        add.ntgrepo = repos[0]
        add.customrepo1 = repos[1]
        add.customrepo2 = repos[2]

    print(Fore.GREEN + "Success: Config File Loaded" + Fore.WHITE)

if str(DebugArgs) not in ["['--debug']"]:
    print(Fore.GREEN + "Current C Compiler: " + cfg.cc + Fore.WHITE)
    print(Fore.GREEN + "Current C++ Compiler: " + cfg.cxx + Fore.WHITE)

# take first argument, used for commands
args1 = full_cmd_arguments[1:]
# take second argument from arguments, mostly used from packages, can be ignored for some functions
args2 = full_cmd_arguments[2:]

# If first args aren't found, tell the user how to use Elements
if not args1:
    print(Fore.RED + "Usage: 'lmt --option package'")
    help.helppage()
    sys.exit()
else:
    args = args1[0]


# Check for internet, since --update could do massive damage without internet
def connect():
    try:
        # Try find internet
        urllib.request.urlopen('https://google.com')
        return True
    except:
        return False


if connect():
    # Several CLI Arguments
    if args in ['--up', '-U', '--update']:
        upd.update()
    elif args in ['--ref', '-R', '--refresh']:
        upd.refresh()
    elif args in ['--cfg-regen']:
        upd.cfgregen()
    elif args in ['--help', '-h', '?']:
        if not args2:
            help.helppage()
        else:
            help.args = str(full_cmd_arguments[2:])
            help.man_command()
    elif args in ['--ver', '-v', "--version"]:
        help.version()
    elif args in ['--list', '-l']:
        print("Packages", " (", pkg_num, ") ", ": ", packages)
    elif args in ['--configure', '--cfg']:
        os.system("clear")
        ntgcfg.tui_interface()
    elif args in ['--gui']:
        print(Fore.RED + "Running Elements in Debug Mode." + Fore.WHITE)
        import lmtgui
    else:
        # Check for second argument
        if not args2:
            print(Fore.RED + "Error: you must specify what package to add/remove." + Fore.WHITE)
        else:
            # Make pkg str in install.py to be the second argument taken before
            add.pkg = args2[0]
            if args in ['--add', '-a']:
                add.install_pkg()
            elif args in ['--del', '-d', '--delete']:
                delete.delete_pkg()
            elif args in ['--sr', '--search', '-s']:
                sr.search_pkg()


else:
    # Average Internet Error
    print(Fore.RED + "No internet. Cannot do " + args + " at the moment." + Fore.WHITE)
