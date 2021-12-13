use std::error::Error;
use std::convert::TryInto;
use zbus::fdo;
use zbus_macros::dbus_interface;
use zbus::MessageHeader;
use regex::Regex;
use linux_embedded_hal::{Delay, I2cdev};
use bme280::BME280;
use std::cell::RefCell;
use std::process::Command;



struct Thermostat {
    min_temp: u16,
    max_temp: u16,
    temperature_auto: bool,
    heating: bool,
    cooling: bool,
    min_humidity: u16,
    max_humidity: u16,
    ventilation_auto: bool,
    ventilation: bool,
}


#[dbus_interface(name = "org.HCPanel.Thermostat")]
impl Thermostat {
    // "Quit" method. A method may throw errors.
    fn quit(&self, #[zbus(header)] hdr: MessageHeader<'_>) -> zbus::fdo::Result<()> {
        let path = hdr.path()?.unwrap();
        let msg = format!("You are leaving me on the {} path?", path);

        Err(zbus::fdo::Error::Failed(msg))
    }

    #[dbus_interface(property, name = "Min_temp")]
    fn get_min_temp(&self) -> &u16 {
        &self.min_temp
    }

    #[dbus_interface(property, name = "Min_temp")]
    fn set_min_temp(&mut self, value: u16) {
        if(value <= self.max_temp){
            self.min_temp = value;
        } else {
            self.min_temp = self.max_temp;
        }
        
    }

    #[dbus_interface(property, name = "Max_temp")]
    fn get_max_temp(&self) -> &u16 {
        &self.max_temp
    }

    #[dbus_interface(property, name = "Max_temp")]
    fn set_max_temp(&mut self, value: u16) {
        if(value >= self.min_temp){
            self.max_temp = value;
        } else {
            self.max_temp = self.min_temp;
        }
    }

    #[dbus_interface(property, name = "Temperature_auto")]
    fn get_temp_auto(&self) -> &bool {
        &self.temperature_auto
    }

    #[dbus_interface(property, name = "Temperature_auto")]
    fn set_temp_auto(&mut self, value: bool) {
        self.temperature_auto = value;
    }

    #[dbus_interface(property, name = "Heating")]
    fn get_heating(&self) -> &bool {
        &self.heating
    }

    #[dbus_interface(property, name = "Heating")]
    fn set_heating(&mut self, value: bool){
        self.cooling = !value;
        self.heating = value;
    }

    #[dbus_interface(property, name = "Cooling")]
    fn get_cooling(&self) -> &bool {
        &self.cooling
    }

    #[dbus_interface(property, name = "Cooling")]
    fn set_cooling(&mut self, value: bool) {
        self.heating = !value;
        self.cooling = value;
    }

    #[dbus_interface(property, name = "Min_humidity")]
    fn get_min_humidity(&self) -> &u16 {
        &self.min_humidity
    }

    #[dbus_interface(property, name = "Min_humidity")]
    fn set_min_humidity(&mut self, value: u16) {
        self.min_humidity = value;
    }

    #[dbus_interface(property, name = "Max_humidity")]
    fn get_max_humidity(&self) -> &u16 {
        &self.max_humidity
    }

    #[dbus_interface(property, name = "Max_humidity")]
    fn set_max_humidity(&mut self, value: u16) {
        self.max_humidity = value
    }

    #[dbus_interface(property, name = "Ventilation_auto")]
    fn get_ventilation_auto(&self) -> &bool {
        &self.ventilation_auto
    }

    #[dbus_interface(property, name = "Ventilation_auto")]
    fn set_ventilation_auto(&mut self, value: bool) {
        self.ventilation_auto = value;
    }

    // "Notify" signal (note: no implementation body).
    #[dbus_interface(signal)]
    fn notify(&self, message: &str) -> zbus::Result<()>;

}

struct EnvSensor{
    bus: u16,
    address: u16,
    sensor: BME280<I2cdev,Delay>,
    temperature: f32,
    humidity: f32,
    pressure: f32,
}
#[dbus_interface(name = "org.HCPanel.EnvSensor")]
impl EnvSensor {
    // "Quit" method. A method may throw errors.
    fn quit(&self, #[zbus(header)] hdr: MessageHeader<'_>) -> zbus::fdo::Result<()> {
        let path = hdr.path()?.unwrap();
        let msg = format!("You are leaving me on the {} path?", path);

        Err(zbus::fdo::Error::Failed(msg))
    }

    #[dbus_interface(property, name = "BUS")]
    fn bus(&self) -> &u16 {
        &self.bus
    }
    #[dbus_interface(property, name = "ADDRESS")]
    fn address(&self) -> &u16 {
        &self.address
    }
    #[dbus_interface(property, name = "Temperature")]
    fn temperature(&self) -> &f32 {
        &self.temperature
    }
    #[dbus_interface(property, name = "Humidity")]
    fn humidity(&self) -> &f32 {
        &self.humidity
    }
    #[dbus_interface(property, name = "Pressure")]
    fn pressure(&self) -> &f32 {
        &self.pressure
    }

    fn refresh_sensor_data(&mut self){
        let measurements = match self.sensor.measure() {
                Ok(measurements) => measurements,
                Err(_e) => {
                    println!("Error reading sensor data {:?}", _e);
                    self.sensor.measure().unwrap()
                }
            };
        let measurements = self.sensor.measure().unwrap();
        self.humidity = measurements.humidity;
        self.temperature = measurements.temperature;
        self.pressure = measurements.pressure;
    }

    fn get_sensor_data_as_json(&mut self) -> String {
        self.refresh_sensor_data();
        format!("Temperature: {}, Humidity: {}, Pressure: {}", self.temperature, self.humidity, self.pressure)
    }
}


struct Ventilation{
    pin: u16,
    state_on: bool,
}
#[dbus_interface(name = "org.HCPanel.Ventilation")]
impl Ventilation {
    // "Quit" method. A method may throw errors.
    fn quit(&self, #[zbus(header)] hdr: MessageHeader<'_>) -> zbus::fdo::Result<()> {
        let path = hdr.path()?.unwrap();
        let msg = format!("You are leaving me on the {} path?", path);

        Err(zbus::fdo::Error::Failed(msg))
    }

    #[dbus_interface(property, name = "PIN")]
    fn get_pin(&self) -> &u16 {
        &self.pin
    }

    #[dbus_interface(property, name = "PIN")]
    fn set_pin(&mut self, value: u16) {
        self.pin = value;
    }

    #[dbus_interface(property, name = "State_on")]
    fn get_state_on(&self) -> &bool {
        &self.state_on
    }

    #[dbus_interface(property, name = "State_on")]
    fn set_state_on(&mut self, value: bool) {
        self.state_on = value;
    }
}

struct Cooling{
    pin: u16,
    state_on: bool,
}

#[dbus_interface(name = "org.HCPanel.Cooling")]
impl Cooling {
    // "Quit" method. A method may throw errors.
    fn quit(&self, #[zbus(header)] hdr: MessageHeader<'_>) -> zbus::fdo::Result<()> {
        let path = hdr.path()?.unwrap();
        let msg = format!("You are leaving me on the {} path?", path);

        Err(zbus::fdo::Error::Failed(msg))
    }

    #[dbus_interface(property, name = "PIN")]
    fn get_pin(&self) -> &u16 {
        &self.pin
    }

    #[dbus_interface(property, name = "PIN")]
    fn set_pin(&mut self, value: u16) {
        self.pin = value;
    }

    #[dbus_interface(property, name = "State_on")]
    fn get_state_on(&self) -> &bool {
        &self.state_on
    }

    #[dbus_interface(property, name = "State_on")]
    fn set_state_on(&mut self, value: bool) {
        self.state_on = value;
    }

}

struct Heating{
    pin: u16,
    state_on: bool,
}

#[dbus_interface(name = "org.HCPanel.Heating")]
impl Heating {
    // "Quit" method. A method may throw errors.
    fn quit(&self, #[zbus(header)] hdr: MessageHeader<'_>) -> zbus::fdo::Result<()> {
        let path = hdr.path()?.unwrap();
        let msg = format!("You are leaving me on the {} path?", path);

        Err(zbus::fdo::Error::Failed(msg))
    }

    #[dbus_interface(property, name = "PIN")]
    fn get_pin(&self) -> &u16 {
        &self.pin
    }

    #[dbus_interface(property, name = "PIN")]
    fn set_pin(&mut self, value: u16) {
        self.pin = value;
    }

    #[dbus_interface(property, name = "State_on")]
    fn get_state_on(&self) -> &bool {
        &self.state_on
    }

    #[dbus_interface(property, name = "State_on")]
    fn set_state_on(&mut self, value: bool) {
        self.state_on = value;
    }

}


struct Lights{
    pin: u16,
    state_on: bool,
}

#[dbus_interface(name = "org.HCPanel.Lights")]
impl Lights {
    // "Quit" method. A method may throw errors.
    fn quit(&self, #[zbus(header)] hdr: MessageHeader<'_>) -> zbus::fdo::Result<()> {
        let path = hdr.path()?.unwrap();
        let msg = format!("You are leaving me on the {} path?", path);

        Err(zbus::fdo::Error::Failed(msg))
    }

    #[dbus_interface(property, name = "PIN")]
    fn get_pin(&self) -> &u16 {
        &self.pin
    }

    #[dbus_interface(property, name = "PIN")]
    fn set_pin(&mut self, value: u16) {
        self.pin = value;
    }

    #[dbus_interface(property, name = "STATE")]
    fn get_state_on(&self) -> &bool {
        &self.state_on
    }

    #[dbus_interface(property, name = "STATE")]
    fn set_state_on(&mut self, value: bool) {
        self.state_on = value;
    }
}





pub fn run() -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::new_session()?;
    fdo::DBusProxy::new(&connection)?.request_name(
        "org.HCPanel",
        fdo::RequestNameFlags::ReplaceExisting.into(),
    )?;

    let mut object_server = zbus::ObjectServer::new(&connection);
    
    let mut thermostat = Thermostat{
    min_temp: 20,
    max_temp: 25,
    temperature_auto: true,
    heating: false,
    cooling: false,
    min_humidity: 30,
    max_humidity: 45,
    ventilation_auto: true,
    ventilation: false,    
    };
    let mut _sensor = BME280::new(I2cdev::new(String::from("/dev/i2c-1")).unwrap(), 0x76, Delay);
    _sensor.init();
    let mut bme_280 = EnvSensor{
        bus: Regex::new(r"^.*i2c-").unwrap().replace_all(&String::from("/dev/i2c-1"), "").parse::<u16>().unwrap(),
        // bus: Regex::new(r"^.*i2c-").unwrap().replace_all(&i2c_path, "").parse::<u16>().unwrap(),
        address: 0x76,
        sensor: _sensor,
        temperature: 0.0,
        humidity: 0.0,
        pressure: 0.0,
    };
    let mut ventilation = Ventilation{
        pin: 1,
        state_on: false,
    };
    let mut heating = Heating{
        pin: 12,
        state_on: false,
    };
    let mut cooling = Cooling{
        pin: 10,
        state_on: false,
    };
    let mut lights = Lights{
        pin: 1,
        state_on: false,
    };

    object_server.at(&"/org/HCPanel/Thermostat".try_into()?, thermostat)?;
    
    object_server.at(&"/org/HCPanel/Sensors/BME280".try_into()?, bme_280)?;

    object_server.at(&"/org/HCPanel/Control/Heating".try_into()?, heating)?;
    object_server.at(&"/org/HCPanel/Control/Cooling".try_into()?, cooling)?;
    object_server.at(&"/org/HCPanel/Control/Ventilation".try_into()?, ventilation)?;

    object_server.at(&"/org/HCPanel/Control/Lights".try_into()?, lights)?;
    loop {
        if let Err(err) = object_server.try_handle_next() {
            eprintln!("{}", err);
        }
    }
}