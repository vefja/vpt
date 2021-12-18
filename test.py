import cfg, install, helppage, delete, update, protect, os, sys, time


# Test search.go
## WARNING: THIS TEST REQUIRES THE PACKAGE GOLANG
print("Runnning search:")
os.system("go run $HOME/Nitrogen/Elements/elements-search/search.go test")

# Test install
install.pkg = "test"
install.test = True
install.install_pkg()

time.sleep(1)

# Test remove
install.pkg_args = install.pkg
install.pkg = "gnome"
delete.delete_pkg()

# Print config
print("PacMan - " + str(cfg.pm_compat))
print("Repos - " + str(cfg.repos_enabled))
print("Plugins - " + str(cfg.plugins_enabled))
print("Repo List: " + str(cfg.repos[0:]))
print("Plugins List: " + str(cfg.plugins[0:]))
print("")
print("Test End.")