import os # OS Lib for clear command
import sys # Sys is needed for exit
from colorama import Fore # Colorama for coloured text

# Start Configuration Tool for Nitrogen Configuration
def ntg_configuration():
    print("Currently Configuring: Nitrogen")
    print("1) Regenerate .bashrc")
    print("2) Change Default Kernel")
    x = int(input(Fore.GREEN + "1" + Fore.WHITE + "/" + Fore.CYAN + "2" + " " + Fore.WHITE))
    if x == 1: # https://raw.githubusercontent.com/NitrogenLinux/Nitrogen/main/.bashrc
        print("This will remove your current .bashrc configuration")
        print("Are you sure?")
        def prompt():
            x = str(input(Fore.GREEN + "Y" + Fore.WHITE + "/" + Fore.RED + "n" + ' ' + Fore.WHITE))
            if x in ['y']:
                # TODO: Finish regeneration of .bashrc
                print("Unfinished")
            elif x in ['n']:
                sys.exit()
            else:
                print(Fore.RED + '"' + x + '"' + " is not understood." + Fore.WHITE)
                prompt()
        prompt()

    elif x == 2:
        # TODO: Make option to change kernel
        print("Coming soon")
    else:
        print(Fore.RED + "Command not understood." + Fore.WHITE)

# Start configuration tool for Elements Configuration
def lmt_configuration():
    # TODO: Make Elements configuration possible from the application itself, using a tui
    print("Currently Configuring: Elements")
    print("Not Functional Yet. Configure it manually in cfg.py.")

# Interface for choosing between LMT and NTG Config
def tui_interface():
    # Print a lovely welcome
    print("Welcome to Nitrogen Config!")
    print("---------------------------")
    print("What would you like to do today?")
    # TODO: Some more options on each would be welcome
    print("1) Configure " + Fore.GREEN + "Nitrogen" + Fore.WHITE)
    print("2) Configure " + Fore.CYAN + "Elements" + Fore.WHITE)

    # Create a 1/2 Prompt
    def prompt1():
        # Get input of keyboard and translate it to int
        x = int(input(Fore.GREEN + "1" + Fore.WHITE + "/" + Fore.CYAN + "2" + " " + Fore.WHITE))
        if x == 1:
            # Start Configuring Nitrogen
            ntg_configuration()
        elif x == 2:
            # Start configuring Elements
            lmt_configuration()
        else:
            # Error in case of stroke
            print(Fore.RED + '"' + x + '"' + " is not understood." + Fore.WHITE)
            prompt1()
    # Run Prompt
    prompt1()