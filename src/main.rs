#![no_main]
#![no_std]

use panic_probe as _;
use defmt_rtt as _;

use cortex_m::{self, delay::{self, Delay}};
use cortex_m_rt::entry;
use hal::{
    clocks::Clocks, dma::{Dma, DmaChannel, DmaInput, DmaPeriph}, gpio::{OutputType, Pin, PinMode, Port}, i2c::I2c, low_power, pac, timer::{Timer, TimerInterrupt}
};

#[entry]
fn main() -> ! {
   let dp = pac::Peripherals::take().unwrap();
   let cp = pac::CorePeripherals::take().unwrap();


   let clock_cfg = Clocks::default();
   clock_cfg.setup().unwrap();

   let mut delay = Delay::new(cp.SYST, 90_000_000);

   let mut pe8 = Pin::new(Port::E, 8, PinMode::Output);
   pe8.set_high();

   let mut timer = Timer::new_tim3(dp.TIM3, 0.2, Default::default(), &clock_cfg);
   timer.enable_interrupt(TimerInterrupt::Update);

   let mut scl = Pin::new(Port::B, 6, PinMode::Alt(4));
   scl.output_type(OutputType::OpenDrain);

   let mut sda = Pin::new(Port::B, 7, PinMode::Alt(4));
   sda.output_type(OutputType::OpenDrain);

//    let mut dma = Dma::new(dp.DMA1);
//    hal::dma::mux(DmaPeriph::Dma1, DmaChannel::C1, DmaInput::I2c1Tx);

   let mut i2c = I2c::new(dp.I2C1, Default::default(), &clock_cfg);

   loop {
       i2c.write(0x50, &[1, 2, 3]);
       pe8.toggle();
       delay.delay_ms(500);

       // Or:
    //    i2c.write_dma(0x50, &BUF, DmaChannel::C1, Default::default(), DmaPeriph::Dma1);

    //    low_power::sleep_now();
   }
}