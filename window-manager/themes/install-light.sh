rm -rf ~/.icewm/themes/light
mkdir -p ~/.icewm/themes/light 
cp -a ./light/. ~/.icewm/themes/light/ 
icewmbg -p &
echo reload && icewm -r