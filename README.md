# Elements
Elements is a package manager coded in Python and GoLang. And as packages it has tiny shell scripts.

## Installing Elements

### Dependencies
In order to compile and use Elements, the following packages are needed `python 3.10`(may work with 3.9 but it is untested), `pip`, `colorama`, `fuzzywuzzy`, `python-Levenshtein`.

### Precompiled binary
This is actually the method that Elements itself uses
```
wget https://raw.githubusercontent.com/NitrogenLinux/elements/stable/lmt
chmod a+x lmt
sudo mv lmt /usr/bin/
git clone https://github.com/tekq/elements-search.git
sudo mkdir /etc/elements
sudo elements-search/* /etc/elements
rm -rf elements-search
sudo chmod a+x /etc/elements/{search,search-repo}
```

### Compile it yourself
This is a bit tougher
Use the instructions for the Precompiled binary and instead of downloading lmt do:
```
git clone https://github.com/NitrogenLinux/elements.git
cd elements
pyinstaller --onefile Elements.py
sudo mv dist/Elements /usr/bin/lmt
```
And for Elements Search
```
git clone https://github.com/tekq/elements-search.git
cd elements-search
go build search.go
go build search-repo.go
sudo mv search /etc/elements/
sudo mv search-repo /etc/elements/
```

And now you successfully compiled Elements yourself!

## Contributing to Elements
Everyone is welcome to contributing to Elements
