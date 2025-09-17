// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(target_os = "none")]
mod baremetal;
use core::ptr;

static mut DN_TXHDR_0: *mut u32 = 0x7000_0000 as *mut u32;
static mut DN_TXHDR_1: *mut u32 = 0x7000_0004 as *mut u32;
static mut DN_TXHDR_2: *mut u32 = 0x7000_0008 as *mut u32;
static mut DN_TXDATA_PORT: *mut u32 = 0x7000_000C as *mut u32;
static mut UP_RXHDR_0: *mut u32 = 0x7000_0010 as *mut u32;
static mut UP_RXHDR_1: *mut u32 = 0x7000_0014 as *mut u32;
static mut UP_RXDATA_PORT: *mut u32 = 0x7000_0018 as *mut u32;
static mut MISC_CONTROL_REG: *mut u32 = 0x7000_0020 as *mut u32;
static mut MASTER_CAP: *mut u32 = 0x7000_002C as *mut u32;
static mut GLOBAL_CONTROL_0: *mut u32 = 0x7000_0030 as *mut u32;
static mut GLOBAL_CONTROL_1: *mut u32 = 0x7000_0034 as *mut u32;
static mut SLAVE0_DECODE_EN: *mut u32 = 0x7000_0040 as *mut u32;
static mut SLAVE0_CONFIG: *mut u32 = 0x7000_0068 as *mut u32;
static mut SLAVE0_INT_EN: *mut u32 = 0x7000_006C as *mut u32;
static mut SLAVE0_INT_STS: *mut u32 = 0x7000_0070 as *mut u32;
static mut SLAVE0_RXMSG_HDR0: *mut u32 = 0x7000_0074 as *mut u32;
static mut SLAVE0_RXMSG_HDR1: *mut u32 = 0x7000_0078 as *mut u32;
static mut SLAVE0_RXMSG_DATA_PORT: *mut u32 = 0x7000_007C as *mut u32;
static mut SLAVE0_RXVW: *mut u32 = 0x7000_009C as *mut u32;
static mut SLAVE0_RXVW_DATA: *mut u32 = 0x7000_00A0 as *mut u32;
static mut SLAVE0_RXVW_INDEX: *mut u32 = 0x7000_00A4 as *mut u32;
static mut SLAVE0_RXVW_MISC_CNTL: *mut u32 = 0x7000_00A8 as *mut u32;
static mut ESPI_TRAN_CONTROL: *mut u32 = 0x7000_0124 as *mut u32;

// Define masks
static CMD_STATUS_MASK: u32 = 1<<3;
static DNCMD_INT_STATUS: u32 = 1<<28;
static RXMSG_INT_STATUS: u32 = 1<<29;
static RXOOB_INT_STATUS: u32 =  1<<30;

#[cfg(not(target_os = "none"))]
fn main() {
    println!("qemu-sp stub");
}

fn clear_all_int() {
    unsafe {
        ptr::write_volatile(SLAVE0_INT_STS, 0xFFFFFFFF);
        ptr::write_volatile(UP_RXHDR_0,CMD_STATUS_MASK);  // Clear any pending OOB data
    }
}

fn start_cmd() {
    unsafe {
        let val:u32 =  ptr::read_volatile(DN_TXHDR_0) | CMD_STATUS_MASK;
        ptr::write_volatile(DN_TXHDR_0,val);
    }
}

fn wait_cmd_done() {
    unsafe {
        while ptr::read_volatile(DN_TXHDR_0) & CMD_STATUS_MASK  != 0 {}
    }
}

// Wait for specified event
fn wait_event(evt: u32) {
    unsafe {
        while ptr::read_volatile(SLAVE0_INT_STS) & evt == 0 {}
        ptr::write_volatile(SLAVE0_INT_STS, evt); // Clear event
    }
}

fn clear_event(evt: u32) {
    unsafe {
        ptr::write_volatile(SLAVE0_INT_STS, evt);
    }
}

fn set_vwire(index: u8, data: u8) {
    unsafe {
        wait_cmd_done();
        clear_event(DNCMD_INT_STATUS);
        ptr::write_volatile(DN_TXHDR_0, 0x5 | ((2<<8) as u32)); // VWire , count = 2
        ptr::write_volatile(DN_TXHDR_1, 0x0); // Clear any old data
        ptr::write_volatile(DN_TXHDR_2, 0x0); // Clear any old data
        // Two VWire events 
        ptr::write_volatile(DN_TXDATA_PORT, (((data & 0xF0) as u32) << 24) | ((index as u32) << 16) | (index as u32) | ((data as u32) << 8)); // Write out index + data
        start_cmd();
        wait_event(DNCMD_INT_STATUS);
        wait_cmd_done();
    }
}

fn put_oob(target: u8, mctp_code: u8, mctp_bytes: u8, data: &[u8]) {
    unsafe {
        wait_cmd_done();
        clear_event(DNCMD_INT_STATUS);
        // Command Type 6 = OOB packet type
        // Cycle Type 0x21 = OOB message channel
        // Tag 0x3 << 20 = Random tag value
        // Length 11 << 24 = This is length of OOB data only not 3 bytes of header
        ptr::write_volatile(DN_TXHDR_0, 6 | (0x21 << 8) | (0x3 << 20) | (11 << 24)); 
        ptr::write_volatile(DN_TXHDR_1, target as u32 | ((mctp_code as u32) << 8) | ((mctp_bytes as u32) << 16));
        ptr::write_volatile(DN_TXHDR_2, 0x12345678); // Maybe the first byte of data needs to go here?

        for chunk in data.chunks_exact(4) {
            let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            unsafe {
                log::info!("Writing value: {:08X}",word);
                ptr::write_volatile(DN_TXDATA_PORT,word); // Next 4 bytes of data
            }
        }

        log::info!("Send the OOB command");
        start_cmd();
        wait_event(DNCMD_INT_STATUS);
        wait_cmd_done();

        log::info!("OOB command complete");
    }
}

fn get_oob() -> u32 {

    unsafe {
        while ptr::read_volatile(UP_RXHDR_0) & (CMD_STATUS_MASK)  != 0 {}
        let mut rx_len = (ptr::read_volatile(UP_RXHDR_0) >> 24) as u8;
        log::info!("RX OOB len: {:02X}", rx_len);

        // Decode the SMBUS header from UP_RXHDR_1
        let rxhdr1 = ptr::read_volatile(UP_RXHDR_1);
        log::info!("Target:     {:02X}", rxhdr1 as u8);
        log::info!("Opcode:     {:02X}", (rxhdr1 >> 8) as u8);
        log::info!("Length:     {:02X}", (rxhdr1 >> 16) as u8);

        while rx_len > 4 {
            let _data = ptr::read_volatile(UP_RXDATA_PORT);
            rx_len -= 4;
        }
        let data = ptr::read_volatile(UP_RXDATA_PORT);
        
        // Clear the RX data pending
        ptr::write_volatile(UP_RXHDR_0,CMD_STATUS_MASK);
        return data;
    }

    return 0;
}

fn peripheral_write(addr: u32, data: u32) {
    unsafe {
        wait_cmd_done();
        clear_event(DNCMD_INT_STATUS);
        // Command Type 4 = Peripheral downstream
        // Cycle Type 1 = Memory Write 32
        // Tag 0x3 << 20 = Random tag value
        // Length 0 << 24 = Number of bytes expected in response
        ptr::write_volatile(DN_TXHDR_0, 4 | (1 << 8) | (0x3 << 20) | (0x0 << 24)); 
        ptr::write_volatile(DN_TXHDR_1, addr); // address 
        ptr::write_volatile(DN_TXHDR_2, 0); // Maybe the first byte of data needs to go here?
        ptr::write_volatile(DN_TXDATA_PORT,data); // Next 4 bytes of data
        start_cmd();
        wait_event(DNCMD_INT_STATUS);
        wait_cmd_done();
    }

}

// Read 32-bit value at offset addr
fn peripheral_read(addr: u32) -> u32 {
    unsafe {
        wait_cmd_done();
        clear_event(DNCMD_INT_STATUS);
        
        log::info!("Sending downstream Read 32 request");
        // Command Type 4 = Peripheral message
        // Cycle Type 0 = Memory Read 32
        // Tag 0x3 << 20 = Random tag value
        // Length = data length returned 4
        ptr::write_volatile(DN_TXHDR_0, 4 | (0 << 8) | (0x3 << 20) | (7 << 24)); 
        ptr::write_volatile(DN_TXHDR_1, addr); // address 
        ptr::write_volatile(DN_TXHDR_2, 0); // Zero out so no left over data
        start_cmd();
        wait_event(DNCMD_INT_STATUS);
        wait_cmd_done();

        // Now we expect upstream response
        log::info!("Waiting for upstream message");
        wait_event(RXMSG_INT_STATUS);
        let hdr0 = ptr::read_volatile(SLAVE0_RXMSG_HDR0);
        
        log::info!("Cycle Type: {:02X}", hdr0 as u8);
        log::info!("Tag:        {:02X}", (hdr0 >> 8) as u8);
        log::info!("Length:     {:02X}", (hdr0 >> 16) as u8);
        log::info!("Data0:      {:02X}", (hdr0 >> 24) as u8);

        // Return 32-bits in HDR1
        ptr::read_volatile(SLAVE0_RXMSG_HDR1)
    }
}


fn set_configuration(reg: u16, val: u32) {
    unsafe {
        wait_cmd_done();
        clear_event(DNCMD_INT_STATUS);
        ptr::write_volatile(DN_TXHDR_0, 0x0 | (((reg & 0xff) as u32) << 16));  
        ptr::write_volatile(DN_TXHDR_1, val);
        ptr::write_volatile(DN_TXHDR_2,0x0);
        ptr::write_volatile(DN_TXDATA_PORT,0x0);
        start_cmd();
        wait_event(DNCMD_INT_STATUS);
        wait_cmd_done();
    }
}

fn get_configuration(reg: u16) -> u32 {
    unsafe {
        wait_cmd_done();
        clear_event(DNCMD_INT_STATUS);
        ptr::write_volatile(DN_TXHDR_1,0x0);
        ptr::write_volatile(DN_TXHDR_2,0x0);
        ptr::write_volatile(DN_TXDATA_PORT,0x0);
        ptr::write_volatile(DN_TXHDR_0,0x1);
        ptr::write_volatile(DN_TXHDR_0, 0x1 | (((reg & 0xff) as u32) << 16));  
        start_cmd();
        wait_event(DNCMD_INT_STATUS);
        wait_cmd_done();
        
        return ptr::read_volatile(DN_TXHDR_1);
    }
}

fn send_inband_reset() {
    unsafe {
        wait_cmd_done();
        ptr::write_volatile(DN_TXHDR_1,0x0);
        ptr::write_volatile(DN_TXHDR_2,0x0);
        ptr::write_volatile(DN_TXDATA_PORT,0x0);
        ptr::write_volatile(DN_TXHDR_0, 0x2); // In-band reset
        start_cmd();
        wait_cmd_done();

        ptr::write_volatile(SLAVE0_CONFIG, 0xC0000008); // Set back to default

    }
}

fn init_espi_bus() {
    unsafe {
        // Disable ROM_INIT_ACCESS so we use SLAVE_CONFIG values
        let tran_control = ptr::read_volatile(ESPI_TRAN_CONTROL) & !(1<<20); // Clear ROM_INIT_ACCESS
        ptr::write_volatile(ESPI_TRAN_CONTROL, tran_control);


        // Issue SW_RST
        log::info!("Assert SW_RST");
        ptr::write_volatile(GLOBAL_CONTROL_1, 0x1); // Set SW_RST high 
        log::info!("Deassert SW_RST");
        ptr::write_volatile(GLOBAL_CONTROL_1, 0x0); // Set SW_RST back low
        let _rxvw = ptr::read_volatile(SLAVE0_RXVW);
        ptr::write_volatile(SLAVE0_RXVW,0xFFFF6F00); // clear any active bits
        log::info!("MASTER_CAP: {:08X}", ptr::read_volatile(MASTER_CAP));
        ptr::write_volatile(GLOBAL_CONTROL_0, 0x7B | (0x1400 << 8) | (0x3F << 24));
        ptr::write_volatile(SLAVE0_RXVW_MISC_CNTL,0xf);
        ptr::write_volatile(GLOBAL_CONTROL_1, (1<<20) | (0x2FF << 8) | (1 << 1)); //  Timeout and BUS_MASTER_EN, ALERT_EN
        ptr::write_volatile(SLAVE0_CONFIG, 0xC0000008); // Set back to default
    }
}

#[cfg(target_os = "none")]
#[embassy_executor::main(executor = "embassy_aarch64_haf::Executor")]
async fn embassy_main(_spawner: embassy_executor::Spawner) {
    use ec_service_lib::service_list;

    log::info!("QEMU Secure Partition - build time: {}", env!("BUILD_TIME"));
    
    unsafe {
        
        init_espi_bus();
        log::info!("Skipping bus init");

            // Clear all pending ineterrupts
            clear_all_int();
            ptr::write_volatile(SLAVE0_INT_EN,0);
            ptr::write_volatile(SLAVE0_DECODE_EN,0x4); // Enable port 80/60-64

            log::info!("Inband reset started");
            send_inband_reset();
            log::info!("Inband reset complete");
            
            log::info!("ESPI_TRAN_CONTROL {:08X}", ptr::read_volatile(ESPI_TRAN_CONTROL));
            log::info!("DN_TXHDR_0 {:08X}", ptr::read_volatile(DN_TXHDR_0));
            log::info!("MISC_CONTROL_REG {:08X}", ptr::read_volatile(MISC_CONTROL_REG));
            log::info!("GLOBAL_CONTROL_0 {:08X}", ptr::read_volatile(GLOBAL_CONTROL_0));
            log::info!("GLOBAL_CONTROL_1 {:08X}", ptr::read_volatile(GLOBAL_CONTROL_1));
            log::info!("SLAVE0_DECODE_EN {:08X}", ptr::read_volatile(SLAVE0_DECODE_EN));
            log::info!("SLAVE0_CONFIG {:08X}", ptr::read_volatile(SLAVE0_CONFIG));
            log::info!("SLAVE0_INT_EN {:08X}", ptr::read_volatile(SLAVE0_INT_EN));
            log::info!("SLAVE0_INT_STS {:08X}", ptr::read_volatile(SLAVE0_INT_STS));
            log::info!("SLAVE0_RXVW_MISC_CNTL {:08X}", ptr::read_volatile(SLAVE0_RXVW_MISC_CNTL));

    
            // GET_CONFIGURATION
            log::info!("GET_CONFIGURATION");
            let gen_cap = get_configuration(0x08);
            log::info!("DEVICE_ID    {:08X}", get_configuration(0x4));
            log::info!("GENERAL_CAP  {:08X}", gen_cap);
            log::info!("CHANNEL0_CAP {:08X}", get_configuration(0x10));
            log::info!("CHANNEL1_CAP {:08X}", get_configuration(0x20));
            log::info!("CHANNEL2_CAP {:08X}", get_configuration(0x30));
            log::info!("CHANNEL3_CAP {:08X}", get_configuration(0x40));

            log::info!("Enabling Alert Mode and CRC checking");
            set_configuration(0x8, gen_cap | (1<<31) | (1<<28));

            // Enable VWire Channel
            log::info!("Enable VW in SLAVE0_CONFIG");
            ptr::write_volatile(SLAVE0_CONFIG,0xC0000004); 
            set_configuration(0x20,1);

            // Wait for VWire ready to go high
            loop {
                let vwire = get_configuration(0x20);
                if vwire & 1 != 0 {
                    break;
                }
            }

            // Maximum of 4 virtual wires 
            set_configuration(0x20,0x30003);

            log::info!("GENERAL_CAP  {:08X}", get_configuration(0x8));
            log::info!("SLAVE0_CONFIG {:08X}", ptr::read_volatile(SLAVE0_CONFIG));
            log::info!("SLAVE0_INT_EN {:08X}", ptr::read_volatile(SLAVE0_INT_EN));
            log::info!("SLAVE0_INT_STS {:08X}", ptr::read_volatile(SLAVE0_INT_STS));

            log::info!("De-asserting PLTRST and SUS_STS");
            set_vwire(0x3, 0x22); // PLTRST and SUS_STS = 1

            log::info!("Now performing tests with EC");

            // Enable OOB and Peripheral channel
            ptr::write_volatile(SLAVE0_CONFIG,0xC000000E);
            set_configuration(0x10,0x7115); // Peripheral 64 byte payload with 4K max read and enable bit
            set_configuration(0x30,0x111); // OOB 64 byte transfer and enable bit

            log::info!("Peripheral write");

            log::info!("Peripheral read");
            //let val = peripheral_read(0x10);
            //log::info!("Value read = {:08X}", val);

            log::info!("OOB write");
            let oob_data: [u8;16] = [
                0x1, // Source address
                0x1, // hdeader version
                0x2, // Destination endpoint ID
                0x1, // Source EID
                0xD3, // flags
                0xFE, // Command data
                0,0,0,0,0,0,0,0,0,0, // Padding
            ];
            put_oob(0x2,0xf,0x10, &oob_data);

            log::info!("OOB read");
            log::info!("SLAVE0_INT_STS {:08X}", ptr::read_volatile(SLAVE0_INT_STS));

            let rsp = get_oob();


        loop {

        }
    }

    service_list![
        ec_service_lib::services::Thermal::new(),
        ec_service_lib::services::FwMgmt::new(),
        ec_service_lib::services::Notify::new(),
        baremetal::Battery::new()
    ]
    .run_message_loop(async |_| Ok(()))
    .await
    .expect("Error in run_message_loop");
}
