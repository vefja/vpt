import delete,sys,install,update,search
import helppage

full_cmd_arguments = sys.argv
args1 = full_cmd_arguments[1:]
full_cmd_arguments = sys.argv
args2 = full_cmd_arguments[2:]
full_cmd_arguments = sys.argv



debug="false"
help="false"
invalid="false"
updating="false"
package_validity=""
args = args1[0]

import urllib.request
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
    print("Helppage: " + help)
    print("Version: " + helppage.ver)


if connect():
    if args in ['--up', '-U', '--update', '--ref', '-R', '--refresh']:
        updating = "true"
    elif args in ['--help', '-h', '?']:
        help = "true"
        helppage.helppage()
    else:
        if args2 == []:
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
    elif args in ['--sr', '--search', 's']:
        search.search_pkg()
    elif args in ['--ref', '-R', '--refresh']:
        pkg = ""
        update.refresh()
    else:
        if help in ['true']:
            print("")
        elif debug in ['true']:
            print("")

    if invalid in ['true']:
        debugger()

else:
    print("No internet. Using Elements with no internet might be dangerous to the well being of your system, try again later.")