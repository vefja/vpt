# /etc/elements/tools/find_opt.sh
cat /etc/elements/lmt.conf | grep ${1} | sed 's/'${1}'=//'
