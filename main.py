import delete,sys,install,update
import helppage
version="0.0.1"


full_cmd_arguments = sys.argv
args1 = full_cmd_arguments[1:]
full_cmd_arguments = sys.argv
args2 = full_cmd_arguments[2:]

debug="false"
help="false"
invalid="false"
updating="false"
package_validity=""
args = args1[0]

# debug
def debugger():
    print("Debugger:")
    print("Debug: " + debug)
    print("Argument:" + args)
    print("Pkg: " + install.pkg)
    print("Valid: " + package_validity)
    print("Updating: " + updating)
    print("Helppage: " + help)
    print("Version: " + version)


if args in ['--up', '-U', '--update']:
    updating = "true"
elif args in ['--help', '-h', '?']:
    help = "true"
    helppage.helppage()
elif args in ['debug', '--debug', '-dbg', '--i_broke_elements_again,-_-']:
    debug = "true"
    debugger()
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
elif args in ['--up', '-U', '--update']:
    pkg = ""
    update.update()
else:
    if help in ['true']:
        print("")
    elif debug in ['true']:
        print("")

if invalid in ['true']:
    debugger()