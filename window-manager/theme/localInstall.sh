rm -rf ~/.icewm/themes/dotLight 
mkdir -p ~/.icewm/themes/dotLight 
cp -a ./theme/. ~/.icewm/themes/dotLight/ 
icewmbg -p &
echo reload && icewm -r