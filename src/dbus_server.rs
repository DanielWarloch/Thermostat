use std::error::Error;
use std::convert::TryInto;
use zbus::fdo;
use zbus_macros::dbus_interface;
use zbus::MessageHeader;
use regex::Regex;
use linux_embedded_hal::{Delay, I2cdev};
use bme280::BME280;
use std::cell::RefCell;




struct Thermostat {
    min_temp: f32,
    max_temp: f32,
    temp_hysteresis_up: f32,
    temp_hysteresis_down: f32,
    auto: bool,
    heating: bool,
    cooling: bool,
    min_humidity: f32,
    max_humidity: f32,
    humidity_hysteresis_up: f32,
    humidity_hysteresis_down: f32,
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
    fn min_temp(&self) -> &f32 {
        &self.min_temp
    }

    #[dbus_interface(property, name = "Max_temp")]
    fn max_temp(&self) -> &f32 {
        &self.max_temp
    }

    #[dbus_interface(property, name = "Temp_hysteresis_up")]
    fn temp_hysteresis_up(&self) -> &f32 {
        &self.temp_hysteresis_up
    }

    #[dbus_interface(property, name = "Temp_hysteresis_down")]
    fn temp_hysteresis_down(&self) -> &f32 {
        &self.temp_hysteresis_down
    }

    #[dbus_interface(property, name = "Auto")]
    fn auto(&self) -> &bool {
        &self.auto
    }

    #[dbus_interface(property, name = "Heating")]
    fn heating(&self) -> &bool {
        &self.heating
    }

    #[dbus_interface(property, name = "Cooling")]
    fn cooling(&self) -> &bool {
        &self.cooling
    }

    #[dbus_interface(property, name = "Min_humidity")]
    fn min_humidity(&self) -> &f32 {
        &self.min_humidity
    }

    #[dbus_interface(property, name = "Max_humidity")]
    fn max_humidity(&self) -> &f32 {
        &self.max_humidity
    }

    #[dbus_interface(property, name = "Ventilation")]
    fn ventilation(&self) -> &bool {
        &self.ventilation
    }

    // "Notify" signal (note: no implementation body).
    #[dbus_interface(signal)]
    fn notify(&self, message: &str) -> zbus::Result<()>;

}

struct EnvSensor{
    bus: u8,
    address: u8,
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
    fn bus(&self) -> &u8 {
        &self.bus
    }
    #[dbus_interface(property, name = "ADDRESS")]
    fn address(&self) -> &u8 {
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

    fn get_sensor_data_as_string(&mut self) -> String {
        self.refresh_sensor_data();
        format!("Temperature: {}, Humidity: {}, Pressure: {}", self.temperature, self.humidity, self.pressure)
    }
}


struct Ventilation{
    pin: u8,
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
    fn get_pin(&self) -> &u8 {
        &self.pin
    }

    #[dbus_interface(property, name = "STATE")]
    fn get_state_on(&self) -> &bool {
        &self.state_on
    }
}

struct Heating{
    heating_output: bool,
    cooling_output: bool,
}
#[dbus_interface(name = "org.HCPanel.Heating")]
impl Heating {
    // "Quit" method. A method may throw errors.
    fn quit(&self, #[zbus(header)] hdr: MessageHeader<'_>) -> zbus::fdo::Result<()> {
        let path = hdr.path()?.unwrap();
        let msg = format!("You are leaving me on the {} path?", path);

        Err(zbus::fdo::Error::Failed(msg))
    }

    #[dbus_interface(property, name = "Heating_output")]
    fn heating_output(&self) -> &bool {
        &self.heating_output
    }

    #[dbus_interface(property, name = "Cooling_output")]
    fn cooling_output(&self) -> &bool {
        &self.cooling_output
    }
}


struct Lights{
    pin: u8,
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
    fn get_pin(&self) -> &u8 {
        &self.pin
    }

    #[dbus_interface(property, name = "STATE")]
    fn get_state_on(&self) -> &bool {
        &self.state_on
    }
}





pub fn run() -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::new_session()?;
    fdo::DBusProxy::new(&connection)?.request_name(
        "org.HCPanel",
        fdo::RequestNameFlags::ReplaceExisting.into(),
    )?;

    let mut object_server = zbus::ObjectServer::new(&connection);
    println!("test3");
    
    let mut thermostat = Thermostat{
    min_temp: 20.0,
    max_temp: 25.0,
    temp_hysteresis_up: 1.0,
    temp_hysteresis_down: 1.0,
    auto: true,
    heating: false,
    cooling: false,
    min_humidity: 40.0,
    max_humidity: 65.0,
    humidity_hysteresis_down: 5.0,
    humidity_hysteresis_up: 5.0,
    ventilation: false,
    
    };
    let mut _sensor = BME280::new(I2cdev::new(String::from("/dev/i2c-1")).unwrap(), 0x76, Delay);
    _sensor.init();
    let mut bme_280 = EnvSensor{
        bus: Regex::new(r"^.*i2c-").unwrap().replace_all(&String::from("/dev/i2c-1"), "").parse::<u8>().unwrap(),
        // bus: Regex::new(r"^.*i2c-").unwrap().replace_all(&i2c_path, "").parse::<u8>().unwrap(),
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
        heating_output: false,
        cooling_output: false,
    };
    let mut lights = Lights{
        pin: 1,
        state_on: false,
    };

    println!("test4");
    object_server.at(&"/org/HCPanel/Thermostat".try_into()?, thermostat)?;
    
    object_server.at(&"/org/HCPanel/Sensors/BME280".try_into()?, bme_280)?;
    // object_server.at(&"/org/HCPanel/Sensors/other".try_into()?, bme_280)?;

    object_server.at(&"/org/HCPanel/Control/Heating/Kitchen".try_into()?, heating)?;
    object_server.at(&"/org/HCPanel/Control/Ventilation/Kitchen".try_into()?, ventilation)?;

    object_server.at(&"/org/HCPanel/Control/Lights/Kitchen".try_into()?, lights)?;
    println!("test5");
    loop {
        if let Err(err) = object_server.try_handle_next() {
            eprintln!("{}", err);
        }
        println!("test6aaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        // println!("{}", bme_280.get_sensor_data_as_string());
    }
    println!("test6");
}