import delete, sys, time, os, urllib.request
import install as add
import update as upd
import helppage as help
from colorama import Fore

# Get arguments
arguments = sys.argv
# Declare some things before
packages = ''
ok_post = False

# look for P.O.S.T. disabling arguments
DebugArgs = arguments[3:]

# Elements P.O.S.T.
# -----------------
# P.O.S.T., is the Power On Self Test of Elements
# It tests for every important feature
# But to make debugging easy on other platforms there is a '--debug' flag at the end
# '--debug' can also remove the need for the 'cfg.py' file

# Elements Debug
# --------------
# The included Debugging feature disables the P.O.S.T.
# But it also disables Configuration file, and with mode,
# also in general Debug Mode is for debugging on other
# Operating Systems, so do not use it for another reason.


if DebugArgs:
    DebugArgs = DebugArgs[0]
    if DebugArgs not in '--debug':
        print(Fore.RED + 'Debug Arguments Invalid')
        sys.exit()

else:
    ok_post = True
    print(Fore.GREEN + "Post Enabled" + Fore.WHITE)

# P O S T, do all checks to be sure Elements can go ahead
if os.geteuid() != 0:
    print(Fore.RED + "Fatal Error: You must run Elements as root." + Fore.WHITE)
    sys.exit()
else:
    ok_post = True

if ok_post is True:
    pkgs = open('/etc/elements/pkgs', 'r')
    packages = pkgs.read()
    add.current_pkgs = packages
    print(Fore.GREEN + "Pkgs Loaded")

pkg_num = len(packages.split())

if DebugArgs:
    if DebugArgs in '--debug':
        ok_post = False

# Check for Configuration File
if ok_post is True:
    cfg_load = os.system("ls /etc/elements | grep cfg.py > /dev/null")
elif ok_post is False:
    cfg_load = 0
    print(Fore.RED + "Running in Debug Mode. Configuration File will not be used.")
    add.disable = True
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

    if cfg.repos_enabled is True:
        print(Fore.GREEN + "Setting Repositories", end='')
        sys.stdout.flush()
        print(".", end='')
        sys.stdout.flush()
        print(".", end='')
        sys.stdout.flush()
        print("." + Fore.WHITE)
        repos = cfg.repos
        add.ntgrepo = repos[0]
        add.customrepo1 = repos[1]
        add.customrepo2 = repos[2]

    print(Fore.GREEN + "Success: Config File Loaded" + Fore.WHITE)

if not DebugArgs:
    ok_post = True
    if cfg.plugins_enabled is True:
        plugins = [cfg.plugins[0:], ]
        for plugin in plugins:
            sys.path.append('/etc/elements/plugins')
            import_plugin1 = "import {0}".format(plugin[0], ', '.join(str(i) for i in plugin[1:]))
            exec(import_plugin1)
            import_plugin_stage2 = "import {1}".format(plugin[0], ', '.join(str(i) for i in plugin[1:]))
            exec(import_plugin_stage2)

arguments = arguments[1:]
pkg_args = arguments[1:]
# search for argument 1 and 2
if not arguments:
    print(Fore.RED + "Usage: 'lmt --option package'")
    help.helppage()
    sys.exit()

# take first argument, used for commands
args = arguments[0]


# Check for internet, since --update could remove Elements without internet
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
        if not pkg_args:
            help.helppage()
        else:
            help.args = str(arguments[2:])
            help.man_command()
    elif args in ['--ver', '-v', "--version"]:
        help.version()
    elif args in ['--list', '-l']:
        print("Packages", " (", pkg_num, ") ", ": ", packages)
    else:
        # Make pkg str in install.py to be the second argument taken before
        if args in ['--add', '-a']:
            if not pkg_args:
                print(Fore.RED + "You must specify what package to add/remove.")
                sys.exit()
            add.install_pkg()
        elif args in ['--del', '-d', '--delete']:
            if not pkg_args:
                print(Fore.RED + "You must specify what package to add/remove.")
                sys.exit()
            add.pkg = pkg_args
            delete.delete_pkg()
        elif args in ['--sr', '--search', '-s']:
            if not pkg_args:
                print(Fore.RED + "You must specify what package to add/remove.")
                sys.exit()
            os.system("/etc/elements/binaries/search " + pkg_args[0])
            print("Error code: ", end='')
            print(os.system("/etc/elements/binaries/search " + pkg_args[0] + " > /dev/null"))

        else:
            print(Fore.RED + "Unknown command.")

else:
    # Average Internet Error
    print(Fore.RED + "No internet. Cannot do " + args + " at the moment." + Fore.WHITE)
