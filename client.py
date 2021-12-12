import requests

PORT = '3001'

OBJECT = 'org.HCPanel'
PATH = '/org/HCPanel/Sensors/BME280'
INTERFACE = 'org.HCPanel.EnvSensor'
REFRESH = '/RefreshSensorData'
GET_DATA = '/GetSensorDataAsString'

URL = 'http://localhost:' + PORT + '/Session/' + OBJECT + PATH + '/' + INTERFACE

def call_method(method):
    return requests.get(url = URL + method)

def refresh_sensor_data():
    call_method(REFRESH)

def get_data():
    return call_method(GET_DATA).json()

def get_sensor_data():
    return get_data()['output']

def get_sensor_data_as_dict():
    keys = ['Temperature', 'Humidity', 'Pressure']
    output = get_sensor_data()
    dict = {}
    for key in keys:
        key_idx = output.find(key)
        colon_idx = output.find(':', key_idx)
        comma_idx = output.find(',', key_idx)
        value = float(output[colon_idx + 2 : comma_idx])
        dict.update({key: value})
    return dict

print(get_sensor_data_as_dict())