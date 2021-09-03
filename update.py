import os

def update():
    print("Remove repo")
    os.system("rm -rf ~/.lmt-repo")
    print("Reclone it")
    os.system("git clone https://github.com/tekq/elements-repo.git ~/.lmt-repo")
    os.system("chmod a+x ~/.lmt-repo/*")
    print("Update Complete")