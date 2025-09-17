// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(target_os = "none")]
mod baremetal;
use odp_ffa::{Function,RxTxMap,Aligned4K};

#[cfg(not(target_os = "none"))]
fn main() {
    println!("qemu-sp stub");
}

#[cfg(target_os = "none")]
#[embassy_executor::main(executor = "embassy_aarch64_haf::Executor")]
async fn embassy_main(_spawner: embassy_executor::Spawner) {
    use ec_service_lib::service_list;

    log::info!("QEMU Secure Partition - build time: {}", env!("BUILD_TIME"));

    let tx_buffer = &mut Aligned4K([0u8; 4096]);
    let rx_buffer  = &mut Aligned4K([0u8; 4096]);

    // RxTx code owns these buffers now
    let rxtx = RxTxMap::new(rx_buffer, tx_buffer, 1);
    rxtx.clone().exec().expect("Failed to map RxTx buffers");

    service_list![
        ec_service_lib::services::Thermal::new(),
        ec_service_lib::services::FwMgmt::new(rxtx),
        ec_service_lib::services::Notify::new(),
        baremetal::Battery::new()
    ]
    .run_message_loop(async |_| Ok(()))
    .await
    .expect("Error in run_message_loop");
}
