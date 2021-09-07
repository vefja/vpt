import install, os, sys

def delete_pkg():
    print("Are you sure you want to uninstall " + install.pkg + "?")
    print("1) Yes")
    print("2) No")
    x = input("#? ")
    x = int(x)
    if x == 1:
        print("Removing: " + install.pkg)
        os.system("cd /usr/src/" + install.pkg)
        os.system("make uninstall")
        os.system("rm -v /usr/bin/" + install.pkg)
        os.system("rm -rf -v /usr/src/" + install.pkg)
        print("Removed " + install.pkg + " successfully")
    elif x == 2:
        sys.exit()
    else:
        print(x + " not understood. Valid commands: 1/2")
