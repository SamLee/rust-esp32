use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    nvs::{EspNvsPartition, NvsDefault}, wifi::EspWifi, eventloop::EspSystemEventLoop
};
use log::info;
use embedded_svc::wifi::{ClientConfiguration, Wifi, Configuration};

pub fn get_wifi(modem: Modem, ssid: String, password: String) -> EspWifi<'static> {
    info!("Starting wifi");
    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspNvsPartition::<NvsDefault>::take().unwrap();
    let mut wifi = EspWifi::new(modem, sysloop, Some(nvs)).unwrap();

    wifi.set_configuration(&Configuration::Client(ClientConfiguration{
        ssid: ssid.as_str().into(),
        password: password.as_str().into(),
        ..Default::default()
    })).unwrap();

    wifi.start().unwrap();
    wifi.connect().unwrap();
    
    while !wifi.is_connected().unwrap(){
        let config = wifi.get_configuration().unwrap();
        info!("Waiting for station {:?}", config);
    }
    
    info!("Started wifi");
    return wifi; 
}
