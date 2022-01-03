// build avr-hal documentation:
// clone git@github.com:Rahix/avr-hal.git
// cd into avr-hal/mcu/atmega-hal
// cargo doc --features atmega168 --open

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_ref)]

extern crate panic_halt;

use core::mem;
use atmega168_hal::clock::*;
use atmega168_hal::delay::Delay;
use atmega168_hal::pac::USART0;
use atmega168_hal::port::portd::PD0;
use atmega168_hal::port::portd::PD1;
use atmega168_hal::port::mode::Floating;
use atmega168_hal::port::mode::Input;
use atmega168_hal::port::mode::Output;
use atmega168_hal::prelude::*;
use atmega168_hal::usart::Baudrate;
use atmega168_hal::usart::BaudrateExt;
use atmega168_hal::usart::Usart;
use atmega168_hal::usart::Event::RxComplete;

static mut VALUE: mem::MaybeUninit::<[u8; 8]> = mem::MaybeUninit::<[u8; 8]>::uninit();
static mut SERIAL: mem::MaybeUninit::<Usart<USART0, PD0<Input<Floating>>, PD1<Output>, MHz8>> = mem::MaybeUninit::<Usart<USART0, PD0<Input<Floating>>, PD1<Output>, MHz8>>::uninit();

#[atmega168_hal::entry]
fn main() -> ! {

    let dp = atmega168_hal::pac::Peripherals::take().unwrap();

    let mut port_b = dp.PORTB.split();
    let mut port_c = dp.PORTC.split();
    let mut port_d = dp.PORTD.split();

    // USART
    let baudrate: Baudrate<MHz8> = 57600_u32.into_baudrate();
    let usart = dp.USART0;
    usart.ucsr0b.write(|w| w.rxcie0().set_bit());

    unsafe {
        let mut s = Usart::new(
            usart,
            port_d.pd0,
            port_d.pd1.into_output(&mut port_d.ddr),
            baudrate,
            );
        // Listen for the interrupt that notifies about a complete receive event of the usart
        s.listen(RxComplete);

        // Assign the global usart serial connection
        SERIAL.write(s);
    }

    // counter clock pin
    let mut clock = port_b.pb1.into_output(&mut port_b.ddr);

    // reset rows to zero
    let mut reset_cols = port_b.pb0.into_output(&mut port_b.ddr); 

    // column
    let mut column:i8 = 0;

    // outputs
    let mut row_8 = port_c.pc0.into_output(&mut port_c.ddr);
    let mut row_7 = port_c.pc1.into_output(&mut port_c.ddr);
    let mut row_6 = port_d.pd2.into_output(&mut port_d.ddr);
    let mut row_5 = port_d.pd3.into_output(&mut port_d.ddr);
    let mut row_4 = port_d.pd4.into_output(&mut port_d.ddr);
    let mut row_3 = port_d.pd5.into_output(&mut port_d.ddr);
    let mut row_2 = port_d.pd6.into_output(&mut port_d.ddr);
    let mut row_1 = port_d.pd7.into_output(&mut port_d.ddr);

    let mut delay = Delay::<MHz8>::new();


    unsafe {
        avr_device::interrupt::enable();
    }

    unsafe {
        VALUE.write([1,3,1,1,31,6,17,55]);
    }

    loop {
        reset_cols.set_low().void_unwrap();

        let c = column as usize;
        let value = unsafe { VALUE.assume_init() };

        if 1 == (1 & (value[c] >> 0)) {
            row_1.set_low().void_unwrap();
        }
        if 1 == (1 & (value[c] >> 1)) {
            row_2.set_low().void_unwrap();
        }        
        if 1 == (1 & (value[c] >> 2)) {
            row_3.set_low().void_unwrap();
        }
        if 1 == (1 & (value[c] >> 3)) {
            row_4.set_low().void_unwrap();
        } 
        if 1 == (1 & (value[c] >> 4)) {
            row_5.set_low().void_unwrap();
        }
        if 1 == (1 & (value[c] >> 5)) {
            row_6.set_low().void_unwrap();
        }
        if 1 == (1 & (value[c] >> 6)) {
            row_7.set_low().void_unwrap();
        }
        if 1 == (1 & (value[c] >> 7)) {
            row_8.set_low().void_unwrap();
        }

        delay.delay_us(20u32);

        row_1.set_high().void_unwrap();
        row_2.set_high().void_unwrap();
        row_3.set_high().void_unwrap();
        row_4.set_high().void_unwrap();
        row_5.set_high().void_unwrap();
        row_6.set_high().void_unwrap();
        row_7.set_high().void_unwrap();
        row_8.set_high().void_unwrap();

        clock.set_low().void_unwrap();
        clock.set_high().void_unwrap();

        delay.delay_us(20u32);

        column += 1;

        if column == 8 {
            column = 0;
            reset_cols.set_high().void_unwrap();
        }
    }
}

#[avr_device::interrupt(atmega168)]
fn USART_RX() {

    let mut serial = unsafe { SERIAL.assume_init_mut() };

    let byte_0 = serial.read_byte();
    let byte_1 = serial.read_byte();
    let byte_2 = serial.read_byte();
    let byte_3 = serial.read_byte();
    let byte_4 = serial.read_byte();
    let byte_5 = serial.read_byte();
    let byte_6 = serial.read_byte();
    let byte_7 = serial.read_byte();

    //ufmt::uwriteln!(&mut serial, "Got {}\r", b).void_unwrap();

    unsafe {
        VALUE.write([byte_0, byte_1, byte_2, byte_3, byte_4, byte_5, byte_6, byte_7]);
    }
   
    serial.flush();

    //let value:[i32; 8] = [1,2,3,4,255,160,97,888];
    //
}
