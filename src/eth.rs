use esp_idf_svc::eth::{BlockingEth, EspEth};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use log::info;

pub fn eth_configure<T>(sysloop: &EspSystemEventLoop, eth: &mut EspEth<'_, T>) {
    info!("Eth created");
    let mut eth = BlockingEth::wrap(eth, sysloop.clone()).unwrap();
    info!("Starting eth...");
    eth.start().unwrap();
    info!("Waiting for DHCP lease...");
    eth.wait_netif_up().unwrap();
    let ip_info = eth.eth().netif().get_ip_info().unwrap();
    info!("Eth DHCP info: {:?}", ip_info);
}
