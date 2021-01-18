chmod +x ./run.sh

# Rip people with triple screen setups
DISPLAY=:0 Xephyr -br -ac -screen 1920x1080 -noreset :2 &

find | entr ./run.sh
