#!/bin/bash

function start(){
    /home/pi/thermostat/target/release/thermostat &
    /home/pi/thermostat/refresher.py &
    /home/pi/thermostat/rest-server2.py &
}

function start(){
    killall /home/pi/thermostat/target/release/thermostat
    killall /home/pi/thermostat/refresher.py
    killall /home/pi/thermostat/rest-server2.py
}
