mod exceptions;
mod panic;

use aarch64_rt::entry;

entry!(main);
fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    let mut thermal = ec_service_lib::services::Thermal::new();
    let mut battery = ec_service_lib::services::Battery::new();
    let mut fw_mgmt = ec_service_lib::services::FwMgmt::new();
    let mut notify = ec_service_lib::services::Notify::new();

    let services = ec_service_lib::service_array![thermal, battery, fw_mgmt, notify];

    // Program VBAR_EL1 to our exception handlers
    // Call into the haf-ec-service sp_main
    let service = ec_service_lib::HafEcService {
        rx_buffer_base: 0,
        tx_buffer_base: 0,
        rxtx_page_count: 0,
        services: services.as_ref(),
    };
    service.sp_main();
}
