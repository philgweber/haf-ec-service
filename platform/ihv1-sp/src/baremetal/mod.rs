mod exceptions;
mod panic;

use aarch64_rt::entry;

entry!(main);
fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    // Program VBAR_EL1 to our exception handlers
    // Call into the haf-ec-service sp_main
    let service = haf_ec_service::HafEcService {
        rx_buffer_base: 0,
        tx_buffer_base: 0,
        rxtx_page_count: 0,
    };
    service.sp_main();
}
