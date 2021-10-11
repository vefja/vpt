from colorama import Fore # Colorama for coloured text

# Start Configuration Tool for Nitrogen Configuration
def ntg_configuration():
    print("Currently Configuring: Nitrogen")

# Start configuration tool for Elements Configuration
def lmt_configuration():
    print("Currently Configuring: Elements")

# Interface for choosing between LMT and NTG Config
def tui_interface():
    # Print a lovely welcome
    print("Welcome to Nitrogen Config!")
    print("---------------------------")
    print("What would you like to do today?")
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
            print(Fore.RED + '"' + str(x) + '"' + " " + "is not a valid command." + Fore.WHITE)
            prompt1()
    # Run Prompt
    prompt1()