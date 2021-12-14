#!/usr/bin/env python3


from pydbus import SessionBus
import subprocess

from time import sleep




def get_sensors():
    proc = subprocess.Popen("busctl --user tree org.HCPanel | grep -Po '/Sensors/\K.*'", shell=True, stdout=subprocess.PIPE)
    sensors_list = []
    for line in proc.stdout:
        sensors_list.append(line.rstrip().decode("utf-8"))
    return sensors_list

def get_devices():
    proc = subprocess.Popen("busctl --user tree org.HCPanel | grep -Po '/Control/\K.*'", shell=True, stdout=subprocess.PIPE)
    devices_list = []
    for line in proc.stdout:
        devices_list.append(line.rstrip().decode("utf-8"))
    return devices_list


def BME280_heating(thermostat, sensor, heating, cooling):
    if thermostat.Temperature_auto:
        thermostat.Cooling = False
        thermostat.Heating = False
        if sensor.Temperature < thermostat.Min_temp:
            heating.State_on = True
        elif sensor.Temperature > thermostat.Min_temp:
            heating.State_on = False
        if sensor.Temperature > thermostat.Max_temp:
            cooling.State_on = True
        else:
            cooling.State_on = False
    else:
        heating.State_on = thermostat.Heating
        cooling.State_on = thermostat.Cooling


def BME280_ventilation(thermostat, sensor, ventilation):
    if thermostat.Ventilation_auto:
        if sensor.Humidity > thermostat.Max_humidity:
            ventilation.State_on = True
        else:
            ventilation.State_on = False
    else:
        ventilation.State_on = thermostat.Ventilation
        

def sensor_refresh(sensor):
    sensor.RefreshSensorData()

def main():
    dbus_sesion = SessionBus()
    thermostat = dbus_sesion.get('org.HCPanel', f"/org/HCPanel/Thermostat")
    BME280 = dbus_sesion.get('org.HCPanel', f"/org/HCPanel/Sensors/BME280")
    heating = dbus_sesion.get('org.HCPanel', f"/org/HCPanel/Control/Heating")
    cooling = dbus_sesion.get('org.HCPanel', f"/org/HCPanel/Control/Cooling")
    ventilation = dbus_sesion.get('org.HCPanel', f"/org/HCPanel/Control/Ventilation")

    while(True):
        sleep(1)
        sensor_refresh(BME280)
        BME280_heating(thermostat, BME280, heating, cooling)
        BME280_ventilation(thermostat, BME280, ventilation)

if __name__ == '__main__':
	main()