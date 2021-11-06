import os # OS Lib for clear command
import sys # Sys is needed for exit
from colorama import Fore # Colorama for coloured text

Debug = ''

# Start Configuration Tool for Nitrogen Configuration
def ntg_configuration():
    x = int(input(Fore.GREEN + "1" + Fore.WHITE + "/" + Fore.CYAN + "2" + " " + Fore.WHITE))
    if x == 1:
        print("This will Regenerate your GRUB Configuration. Are you sure?")
        def prompt():
            x = str(input(Fore.GREEN + "Y" + Fore.WHITE + "/" + Fore.RED + "n" + ' ' + Fore.WHITE))
            if x in ['y']:
                os.system("update-grub")
            elif x in ['n']:
                sys.exit()
            else:
                print(Fore.RED + '"' + x + '"' + " is not understood." + Fore.WHITE)
                prompt()
        prompt()

    elif x == 2:
        os.system('nano /etc/default/grub')
    else:
        print(Fore.RED + "Command not understood." + Fore.WHITE)
        ntg_configuration()

# Start configuration tool for Elements Configuration
def lmt_configuration():
    # TODO: Make Elements configuration possible from the application itself, using a tui
    print("Currently Configuring: Elements")
    print("Under Construction.")

# Interface for choosing between LMT and NTG Config
def tui_interface():
    # Print a lovely welcome
    print("Welcome to Nitrogen Config!")
    print("---------------------------")
    print("What would you like to do today?")
    # TODO: Some more options on each would be welcome
    print("1) Configure " + Fore.CYAN + "Nitrogen" + Fore.WHITE)
    print("2) Configure " + Fore.LIGHTGREEN_EX + "Elements" + Fore.WHITE)

    # Create a 1/2 Prompt
    def prompt1():
        # Get input of keyboard and translate it to int
        x = int(input(Fore.CYAN + "1" + Fore.WHITE + "/" + Fore.LIGHTGREEN_EX + "2" + " " + Fore.WHITE))
        if x == 1:
            # Start Configuring Nitrogen
            print("Currently Configuring: Nitrogen")
            print("1) Regenerate GRUB Configuration")
            print("2) Open GRUB Configuration file")
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