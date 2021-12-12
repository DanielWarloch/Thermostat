#!/usr/bin/env python3

from flask import Flask, request, jsonify
from pydbus import SessionBus
from gi.repository import GLib
import subprocess
import distutils.util

class HCPanel:

	bus = SessionBus()

	def set_thrmostat_property(self, name, value):
		termostat = self.bus.get('org.HCPanel', f"/org/HCPanel/Thermostat")
		if name=="Cooling":
			termostat.Cooling = bool((distutils.util.strtobool(value)))
		elif name=="Heating":
			termostat.Heating = bool((distutils.util.strtobool(value)))
		elif name=="Max_humidity":
			termostat.Max_humidity = int(value)
		elif name=="Max_temp":
			termostat.Max_temp = int(value)
		elif name=="Min_humidity":
			termostat.Min_humidity = int(value)
		elif name=="Min_temp":
			termostat.Min_temp = int(value)
		elif name=="Temperature_auto":
			termostat.Temperature_auto = bool((distutils.util.strtobool(value)))
		elif name=="Ventilation_auto":
			termostat.Ventilation_auto = bool((distutils.util.strtobool(value)))
		return 200

	def get_thrmostat_property(self, name):
		return getattr(self.bus.get('org.HCPanel', f"/org/HCPanel/Thermostat"), name)

	def refreshSensorData(self, sensor_name):
		return self.bus.get('org.HCPanel', f"/org/HCPanel/Sensors/{sensor_name}").RefreshSensorData

	def getEnvSensorData(self, sensor_name):
		# temperature = self.bus.get('org.HCPanel', f"/org/HCPanel/Sensors/{sensor_name}").Temperature
		# humidity = self.bus.get('org.HCPanel', f"/org/HCPanel/Sensors/{sensor_name}").Humidity
		# pressure = self.bus.get('org.HCPanel', f"/org/HCPanel/Sensors/{sensor_name}").Pressure
		# return {"Temperature": temperature, "Humidity": humidity, "Pressure": pressure}
		a = self.bus.get('org.HCPanel', f"/org/HCPanel/Sensors/{sensor_name}").GetSensorDataAsJson()
		return f"{a}"

	def get_list_of_available_sensors(self):
		proc = subprocess.Popen("busctl --user tree org.HCPanel | grep -Po '/Sensors/\K.*'", shell=True, stdout=subprocess.PIPE)
		a = []
		for line in proc.stdout:
			a.append(line.rstrip().decode("utf-8"))
		return a





api = Flask(__name__)
hcpanel = HCPanel()






@api.route('/sensors', methods = ['GET', 'POST'])
def get_list_of_available_sensors():
	return jsonify(hcpanel.get_list_of_available_sensors()) # list of sensors under "/org/HCPanel/Sensors"

@api.route('/sensors/RefreshSensorData', methods=['GET', 'POST'])
def refreshSensorData():
	content = request.json
	hcpanel.refreshSensorData(content['name'])
	return "", 200

@api.route('/sensors/getSensorData', methods=['GET', 'POST'])
def getSensorData():
	content = request.json
	return jsonify(hcpanel.getEnvSensorData(content['name']))

@api.route('/thermostat/getProperty', methods=['GET', 'POST'])
def thermostatGetProperty():
	content = request.json
	return jsonify(hcpanel.get_thrmostat_property(content['name']))

@api.route('/thermostat/setProperty', methods=['GET', 'POST'])
def thermostatSetProperty():
	content = request.json
	return jsonify(hcpanel.set_thrmostat_property(content['name'], content['value']))


def main():
	api.run(host='0.0.0.0', port=3500)
	

if __name__ == '__main__':
	main()