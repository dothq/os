echo "Note: this script requeres Xephyr and parallel to run along with all of the other dependancies"

./runX.sh &
./runWM.sh &
DISPLAY=:100 ./compositor/dev.sh &
DISPLAY=:100  ./window-manager/themes/dev.sh &
