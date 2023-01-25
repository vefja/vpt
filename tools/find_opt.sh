# /etc/vpt/tools/find_opt.sh
cat /etc/vpt/vpt.conf | grep ${1} | sed 's/'${1}'=//'
