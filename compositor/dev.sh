trap break INT
find $(dirname $BASH_SOURCE) | entr $(dirname $BASH_SOURCE)/localRun.sh