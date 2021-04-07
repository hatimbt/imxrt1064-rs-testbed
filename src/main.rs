#![no_std]
#![no_main]

//use imxrt_ral as ral;
use teensy4_fcb as _;
use teensy4_panic as _;
use imxrt_hal as hal;
use imxrt_hal::{self, gpio::GPIO};

#[cortex_m_rt::entry]
fn main() -> ! {
    
    let p = hal::Peripherals::take().unwrap();
    
    let mut user_led = GPIO::new(p.iomuxc.ad_b0.p09)
        .output();

    user_led.clear();

    loop{
    }

}
