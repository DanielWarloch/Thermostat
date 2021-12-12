#!/usr/bin/env python3

import distutils.util
import sys
from flask import Flask, json, request
from pydbus import SessionBus
from gi.repository import GLib

bus = SessionBus()
sensor = bus.get('org.HCPanel', '/org/HCPanel/Sensors/BME280')
thermostat = bus.get('org.HCPanel', '/org/HCPanel/Thermostat')
api = Flask(__name__)


@api.route('/bme280/methods/GetSensorDataAsString', methods = ['GET'])
def get_sensor_data_as_string():
    return sensor.GetSensorDataAsString()

@api.route('/bme280/methods/RefreshSensorData', methods = ['GET'])
def refresh_sensor_data():
    return sensor.RefreshSensorData()

@api.route('/bme280/methods/Quit', methods = ['POST'])
def quit_sensor():
    return sensor.Quit()

@api.route('/bme280/properties/Address', methods = ['GET'])
def address():
    return str(sensor.ADDRESS)

@api.route('/bme280/properties/Humidity', methods = ['GET'])
def humidity():
    return str(sensor.Humidity)

@api.route('/bme280/properties/Pressure', methods = ['GET'])
def pressure():
    return str(sensor.Pressure)

@api.route('/bme280/properties/Temperature', methods = ['GET'])
def temperature():
    return str(sensor.Temperature)

@api.route('/thermostat/methods/TurnOn', methods = ['POST'])
def turn_on():
    return thermostat.TurnOn

@api.route('thermostat/methods/Quit', methods = ['POST'])
def quit_thermostat():
    return thermostat.Quit

@api.route('/thermostat/properties/Cooling', methods = ['GET', 'POST'])
def cooling():
    if request.method == 'POST':
        cooling = request.args.get('cooling')

        if cooling:
            thermostat.Cooling = bool((distutils.util.strtobool(cooling)))
    
    return str(thermostat.Cooling)

@api.route('/thermostat/properties/Heating', methods = ['GET', 'POST'])
def heating():
    if request.method == 'POST':
        heating = request.args.get('heating')

        if heating:
            thermostat.Heating = bool((distutils.util.strtobool(heating)))
    
    return str(thermostat.Heating)

@api.route('/thermostat/properties/Max_humidity', methods = ['GET', 'POST'])
def max_humidity():
    if request.method == 'POST':
        max_humidity = request.args.get('max_humidity')

        if max_humidity:
            thermostat.Max_humidity = int(max_humidity)
    
    return str(thermostat.Max_humidity)

@api.route('/thermostat/properties/Max_temp', methods = ['GET', 'POST'])
def max_temp():
    if request.method == 'POST':
        max_temp = request.args.get('max_temp')

        if max_temp:
            thermostat.Max_temp = int(max_temp)
    
    return str(thermostat.Max_temp)

@api.route('/thermostat/properties/Min_humidity', methods = ['GET', 'POST'])
def min_humidity():
    if request.method == 'POST':
        min_humidity = request.args.get('min_humidity')

        if min_humidity:
            thermostat.Min_humidity = int(min_humidity)
    
    return str(thermostat.Min_humidity)

@api.route('/thermostat/properties/Min_temp', methods = ['GET', 'POST'])
def min_temp():
    if request.method == 'POST':
        min_temp = request.args.get('min_temp')

        if min_temp:
            thermostat.Min_temp = int(min_temp)
    
    return str(thermostat.Min_temp)

@api.route('/thermostat/properties/Temperature_auto', methods = ['GET', 'POST'])
def temperature_auto():
    if request.method == 'POST':
        temperature_auto = request.args.get('temperature_auto')

        if temperature_auto:
            thermostat.Temperature_auto = bool((distutils.util.strtobool(temperature_auto)))
    
    return str(thermostat.Temperature_auto)

@api.route('/thermostat/properties/Ventilation_auto', methods = ['GET', 'POST'])
def ventilation_auto():
    if request.method == 'POST':
        ventilation_auto = request.args.get('ventilation_auto')
        print(ventilation_auto)
        if ventilation_auto:
            thermostat.Ventilation_auto = bool((distutils.util.strtobool(ventilation_auto)))
    
    return str(thermostat.Ventilation_auto)

@api.route('/thermostat/properties/Temperature_output', methods = ['GET', 'POST'])
def temperature_output():
    if request.method == 'POST':
        temperature_output = request.args.get('temperature_output')

        if temperature_output:
            thermostat.Temperature_output = bool((distutils.util.strtobool(temperature_output)))
    
    return str(thermostat.Temperature_output)

def main():
    api.run()

if __name__ == '__main__':
    main()