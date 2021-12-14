#!/bin/bash

function start_apps(){
    /home/pi/thermostat/target/release/thermostat &
    sleep 1
    /home/pi/thermostat/refresher.py &
    sleep 1
    /home/pi/thermostat/rest-server2.py &
}

function stop_apps(){
    killall /home/pi/thermostat/rest-server2.py
    sleep 1
    killall /home/pi/thermostat/refresher.py
    sleep 1
    killall /home/pi/thermostat/target/release/thermostat
}

case "$1" in 
    start)   start_apps ;;
    stop)    stop_apps ;;
    restart) stop_apps; start_apps ;;
    *) echo "usage: $0 start|stop|restart" >&2
       exit 1
       ;;
esac
