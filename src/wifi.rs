use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::peripheral;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::wifi::BlockingWifi;
use esp_idf_svc::wifi::EspWifi;
use log::{debug, info};

pub fn wifi(
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
    ssid: heapless::String<32>,
    password: heapless::String<64>,
) -> Box<EspWifi<'static>> {
    debug!("using wifi configuration {} / {}", ssid, password);
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None).unwrap();

    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop).unwrap();

    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))
        .unwrap();

    info!("Starting wifi...");
    wifi.start().unwrap();

    info!("Scanning...");
    let ap_infos = wifi.scan().unwrap();

    let channel = ap_infos
        .into_iter()
        .filter(|a| a.ssid == ssid)
        .max_by_key(|a| a.signal_strength)
        .map(|ap| ap.channel);

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid,
        password,
        channel,
        ..Default::default()
    }))
    .unwrap();

    info!("Connecting wifi...");

    wifi.connect().unwrap();

    info!("Waiting for DHCP lease...");
    wifi.wait_netif_up().unwrap();

    let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();
    info!("Wifi DHCP info: {:?}", ip_info);

    Box::new(esp_wifi)
}
