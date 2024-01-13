use std::{sync::Mutex, thread, time::Duration};

use embedded_svc::ws::FrameType;
use esp_idf_hal::peripherals::Peripherals;

#[cfg(feature = "qemu")]
use esp_idf_svc::eth::{EspEth, EthDriver};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::{
        ws::{EspHttpWsConnection, EspHttpWsDetachedSender},
        Configuration, EspHttpServer,
    },
    log::EspLogger,
};
use log::info;
#[cfg(feature = "qemu")]
use rust_esp_debug::eth::eth_configure;
#[cfg(not(feature = "qemu"))]
use rust_esp_debug::wifi::wifi;

pub static DETACHED_SENDERS: Mutex<Vec<EspHttpWsDetachedSender>> = Mutex::new(vec![]);
static WIFI_SSID: &str = "<SSID>";
static WIFI_PASSWORD: &str = "<PASSWORD>";

pub fn start_server<'a>() -> EspHttpServer<'a> {
    let server_config = Configuration::default();
    let mut server = EspHttpServer::new(&server_config).unwrap();
    server.ws_handler("/data", data_stream).unwrap();
    server
}

pub fn data_stream(ws_connection: &mut EspHttpWsConnection) -> Result<(), String> {
    match ws_connection {
        EspHttpWsConnection::New(..) => {
            let mut senders = DETACHED_SENDERS.lock().unwrap();
            senders.push(ws_connection.create_detached_sender().unwrap());
        }
        EspHttpWsConnection::Closed(..) => {
            return Ok(());
        }
        EspHttpWsConnection::Receiving(..) => (),
    }
    Ok(())
}

fn main() {
    EspLogger::initialize_default();
    let _ = EspLogger.set_target_level("*", log::LevelFilter::Info);

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();

    #[cfg(not(feature = "qemu"))]
    let _wifi = wifi(
        peripherals.modem,
        sysloop,
        heapless::String::from(WIFI_SSID),
        heapless::String::from(WIFI_PASSWORD),
    );
    #[cfg(feature = "qemu")]
    let _eth = {
        let mut eth = Box::new(
            EspEth::wrap(EthDriver::new_openeth(peripherals.mac, sysloop.clone()).unwrap())
                .unwrap(),
        );
        eth_configure(&sysloop, &mut eth);
        eth
    };

    let _server = start_server();

    info!("hammer the websocket with data");
    let test_message = "42";
    loop {
        {
            let mut senders = DETACHED_SENDERS.lock().unwrap();
            if senders.len() > 0 {
                // we have some detacheded senders, try to send to all of them and keep only the ones where send is successfull
                senders.retain_mut(|sender| {
                    sender
                        .send(FrameType::Text(false), test_message.as_bytes())
                        .is_ok()
                });
            } else {
                // nothing connected at the moment, don't starve the cpu
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
