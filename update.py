from colorama import Fore
import sys
import os
import helppage


def refresh():
    # Remove current repo to make space for new repo
    os.system("rm -rf /etc/elements/repos/nitrogen")

    # Reclone and Warning to not exit
    print("Recloning repository, do not exit. Exiting may break your repository or even make system unbootable.")
    os.system("git clone https://github.com/tekq/elements-repo.git /etc/elements/repos/nitrogen")

    # Read installed version and newest version
    currentver = os.popen('cat /etc/elements/repos/nitrogen/.current-ver').read()
    ver = helppage.ver

    # Print the versions
    print("Currently installed Elements version: Elements-" + ver)
    print("Elements version in repository: Elements-" + currentver)
    print("Note: In case the Repository has a newer version, it is most recommended to to a 'lmt --update'")


def update():
    ## TODO: remake current updating system to be better
    # backup current executable
    os.system("mv -fv /etc/elements/lmt /etc/elements/lmt.bak")

    # download the new executable
    os.system("wget https://raw.githubusercontent.com/NitrogenLinux/elements/main/dist/lmt")
    os.system("mv -fv lmt /etc/elements/lmt")

    # refresh repositories
    refresh()   
    print(Fore.GREEN + "Elements Update Complete!")

def cfgregen():
    # Warning just in case
    print("Doing this will remove your old cfg.py, and replace it with a fresh one, are you sure?")

    # Create a Y/N prompt
    def prompt():
        x = str(input(Fore.GREEN + "Y" + Fore.WHITE + "/" + Fore.RED + "n" + ' ' + Fore.WHITE))
        # Check if user wants to regen config
        if x in ['y']:
            # If yes it will regenerate
            print("Regenerating Config...")
            os.system("curl https://raw.githubusercontent.com/NitrogenLinux/elements/main/cfg.py > "
                      "/etc/elements/cfg.py")
        elif x in ['n']:
            # If not it will exit
            sys.exit()
        else:
            # If input meaning is unsure, throw an error at the user, they should figure it out
            print(Fore.RED + '"' + x + '"' + " is not understood." + Fore.WHITE)
            prompt()
    # Start the new prompt
    prompt()
