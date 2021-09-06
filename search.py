import os,install,re

def search_pkg():
    searched_item=int(os.system("ls ~/.lmt-repo | grep " + install.pkg + " > /dev/null"))
    if searched_item == 256:
        print(install.pkg + " not in current repo or not in repo. If you know this package is in the repo then i would recommend doing a lmt --refresh")
    else:
        print(install.pkg + " found.")