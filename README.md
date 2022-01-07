# Elements Next
New experimental updates for Elements
IMPORTANT: If you decide to use Elements Next(not recommended), your system working is not a guarranteed due to it being really unstable and experimental.

## To Upgrade To Next
First of all, this is not recommended, Next is very experimental and usually has a ton of changes
anyways, if you want to continue...
Remove your current elements installation
``` $ sudo rm /usr/bin/lmt ```
Get the Experimental version
``` $ wget https://raw.githubusercontent.com/NitrogenLinux/elements/next/lmt ```
Move your new shiny experimental version to /usr/bin
``` $ sudo mv lmt /usr/bin ```
Enjoy the newest Elements has to offer!

## To Downgrade Back to stable:
Remove Elements Next
```
$ sudo rm /usr/bin/lmt
```
Get the Stable version
```
$ wget https://raw.githubusercontent.com/NitrogenLinux/elements/stable/lmt
```
Move stable version to /usr/bin
```
$ sudo mv lmt /usr/bin
```
You can now use Elements Stable again!
