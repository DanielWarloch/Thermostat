mod dbus_server;

use glob::glob;

use std::{thread, time::Duration};
use std::collections::HashMap;
use std::fs;







fn main() {
    let json_conf_file_path = String::from("/usr/share/thermostat/configuration/default.json");
    
    let mut dbus_data: Vec<HashMap<String, String>> = Vec::new();
    println!("test");
    
    
    let conf = json::parse(&fs::read_to_string(json_conf_file_path).unwrap()).unwrap();
    let mut paths: Vec<String> = Vec::new();
    let mut addresses: Vec<u16> = Vec::new();

    thread::sleep(Duration::from_secs(
        1u64,
        //conf["Startup delay"].to_string().parse::<u64>().unwrap(),
    ));

    if conf["AllowBusses"][0] == "all" {
        for entry in glob(r"/dev/i2c-[0-9]*").expect("Failed to read glob pattern"){
            match entry {
                Ok(path) => paths.push(path.into_os_string().into_string().unwrap()),
                Err(e) => println!("{:?}", e),
            }
        }
    } else {
        for entry in conf["AllowedBusses"].members() {
            paths.push(entry.to_string());
        }
    }
    if conf["Address"].is_empty(){
        for entry in conf["Address"].members(){
            addresses.push(entry.to_string().parse::<u16>().unwrap());
        }
    }

   
    for addr in addresses{
        for path in &paths {
            let mut data: HashMap<String, String> = HashMap::new();
            let mut error = false;
            data.insert("Address".to_string(), addr.to_string());
            data.insert("Path".to_string(), (&path).to_string());

            if !error {
                dbus_data.push(data);
            }
        }
    }
  
    dbus_server::run();
    // let _ = match dbus_server::run(dbus_data) {
    //     Ok(_) => 0,
    //     Err(_e) => {
    //         println!("Error creating DBUS server {}", _e);
    //         ()
    //     }
    // };
















}

// // using Linux I2C Bus #1 in this example
// let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();

// // initialize the BME280 using the primary I2C address 0x76
// let mut bme280 = BME280::new_primary(i2c_bus, Delay);

// // or, initialize the BME280 using the secondary I2C address 0x77
// // let mut bme280 = BME280::new_secondary(i2c_bus, Delay);

// // or, initialize the BME280 using a custom I2C address
// // let bme280_i2c_addr = 0x88;
// // let mut bme280 = BME280::new(i2c_bus, bme280_i2c_addr, Delay);

// // initialize the sensor
// bme280.init().unwrap();

// // measure temperature, pressure, and humidity
// let measurements = bme280.measure().unwrap();

// println!("Relative Humidity = {}%", measurements.humidity);
// println!("Temperature = {} deg C", measurements.temperature);
// println!("Pressure = {} pascals", measurements.pressure);
