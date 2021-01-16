# This shell file install all themes to the users theme directory
# This is intended for development as the themes should be stored
# in the system directory (/etc/icewm/themes/) in production.

rm -rf ~/.icewm/themes/dotOS\ Light/
rm -rf ~/.icewm/themes/dotOS\ Dark/
mkdir -p ~/.icewm/themes/dotOS\ Light/
mkdir -p ~/.icewm/themes/dotOS\ Dark/
cp -a ./light/. ~/.icewm/themes/dotOS\ Light/
cp -a ./dark/. ~/.icewm/themes/dotOS\ Dark/

icewmbg -p &
echo reload && icewm -r