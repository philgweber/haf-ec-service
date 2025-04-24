mod panic;
mod services;

#[allow(unused)]
use aarch64_rt::entry;
use ec_service_lib::{async_msg_loop, sp_logger::SpLogger, Service as _};
use log::error;

entry!(aarch64_rt_main);
fn aarch64_rt_main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    log::set_logger(&SpLogger).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    main();
}

#[embassy_executor::main(executor = "embassy_aarch64_haf::Executor")]
async fn embassy_main(_spawner: embassy_executor::Spawner) {
    log::info!("eSPI Secure Partition - build time: {}", env!("BUILD_TIME"));
    let mut thermal = ec_service_lib::services::Thermal::new();

    async_msg_loop(async |msg| {
        if msg.uuid() == thermal.service_uuid() {
            thermal.ffa_msg_send_direct_req2(msg).await
        } else {
            error!("Unknown UUID {}", msg.uuid());
            Err(odp_ffa::Error::Other("Unknown UUID"))
        }
    })
    .await
    .expect("Error in async_msg_loop");
}
