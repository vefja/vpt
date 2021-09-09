import os
import helppage


def refresh():
    print("Remove repo")
    os.system("rm -rf ~/.lmt-repo")
    print("Reclone it")
    os.system("git clone https://github.com/tekq/elements-repo.git ~/.lmt-repo")
    os.system("chmod a+x ~/.lmt-repo/*")
    print("Checking for updates")
    currentver = os.popen('cat ~/.lmt-repo/.current-ver').read()
    ver = helppage.ver
    print("Local Elements version: " + ver)
    print("Newest Elements version: " + currentver)


def update():
    # delete current elements files
    os.system("rm -rvf /usr/share/elements")
    # replace them with the latest and greatest
    os.system("git clone https://github.com/NitrogenLinux/elements.git")
    os.system("mv -v elements /usr/share/")
    print("Elements Update Complete!")

def cfgregen():
    print("Regenerating Config...")
    os.system("curl https://github.com/NitrogenLinux/lmt.cfg > /usr/share/elements/lmt.cfg")
