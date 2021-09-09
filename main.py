import urllib.request
import delete
import sys
import install
import update
import search
import helppage
import os

full_cmd_arguments = sys.argv
args1 = full_cmd_arguments[1:]
full_cmd_arguments = sys.argv
args2 = full_cmd_arguments[2:]
full_cmd_arguments = sys.argv

debugging = os.system("ls /usr/share/elements | grep debug")

os.system("bash /usr/share/elements/lmt.cfg")

debug = "false"
helper = "false"
invalid = "false"
updating = "false"
package_validity = ""
args = args1[0]


def connect():
    try:
        urllib.request.urlopen('http://google.com')
        return True

    except:
        return False


# debug
def debugger():
    print("Debugger:")
    print("Debug: " + debug)
    print("Argument:" + args)
    print("Pkg: " + install.pkg)
    print("Valid: " + package_validity)
    print("Updating: " + updating)
    print("C Flags: " + str(os.system("echo $CFLAGS")))
    print("C++ Flags: " + str(os.system("echo $CXXFLAGS")))
    print("Helppage: " + helper)
    print("Version: " + helppage.ver)


if connect():
    if args in ['--up', '-U', '--update', '--ref', '-R', '--refresh', '--cfg-regen']:
        updating = "true"
    elif args in ['--help', '-h', '?']:
        helper = "true"
        helppage.helppage()
    else:
        os.system("bash /usr/share/elements/lmt.cfg")
        if debugging == 0:
            debug = "true"
            debugger()
        if not args2:
            print("Error: you must specify what package to add/remove.")
        else:
            package_validity = "valid"

        if package_validity in ['valid']:
            install.pkg = args2[0]
        else:
            sys.exit()

    if args in ['--add', '-a']:
        install.install_pkg()
    elif args in ['--del', '-d', '--delete']:
        delete.delete_pkg()
    elif args in ['--sr', '--search', '-s']:
        search.search_pkg()
    elif args in ['--ref', '-R', '--refresh']:
        pkg = ""
        update.refresh()
    elif args in ['--cfg-regen']:
        pkg = ""
        update.cfgregen()
    else:
        if helper in ['true']:
            print("")
        elif debug in ['true']:
            print("")

    if invalid in ['true']:
        debugger()

else:
    print("No internet. Cannot do " + args + " at the moment.")
