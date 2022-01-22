# About Elements
Elements is a package manager that is trying to be as User Friendly as possible
# Installing Elements
You can either grab a precompiled binary, or you could compile it yourself with pyinstaller.

Usint a precompiled binary:

Download the binary from GitHub
move the binary to /usr/bin/ and make it runable
```
sudo mv -v /path/to/lmt /usr/bin/
sudo chmod a+x /usr/bin/lmt
```
Compiling it yourself:
```
git clone https://github.com/NitrogenLinux/elements.git
cd elements
pyinstaller --onefile Elements.py
chmod a+x dist/lmt
sudo mv -v dist/lmt /usr/bin/
```
# Contributing
Contributions are and always will be welcome!
