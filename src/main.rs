#![no_std]
#![no_main]

extern crate panic_halt;

use atmega168_hal::prelude::*;
use atmega168_hal::delay::Delay;
use atmega168_hal::clock::*;

#[atmega168_hal::entry]
fn main() -> ! {

    let dp = atmega168_hal::pac::Peripherals::take().unwrap();
    let mut port_b = dp.PORTB.split();
    let mut port_d = dp.PORTD.split();

    // counter clock pin
    let mut clock = port_b.pb1.into_output(&mut port_b.ddr);

    // reset rows to zero
    let mut reset_cols = port_b.pb0.into_output(&mut port_b.ddr); 

    // column
    let mut column:i8 = 0;

    // outputs
    let mut row_1 = port_d.pd0.into_output(&mut port_d.ddr);
    let mut row_2 = port_d.pd1.into_output(&mut port_d.ddr);
    let mut row_3 = port_d.pd2.into_output(&mut port_d.ddr);
    let mut row_4 = port_d.pd3.into_output(&mut port_d.ddr);
    let mut row_5 = port_d.pd4.into_output(&mut port_d.ddr);
    let mut row_6 = port_d.pd5.into_output(&mut port_d.ddr);
    let mut row_7 = port_d.pd6.into_output(&mut port_d.ddr);
    let mut row_8 = port_d.pd7.into_output(&mut port_d.ddr);

    let mut delay = Delay::<MHz8>::new();

    // value table
    let value:[i32; 8] = [1,3,7,15,31,63,127,255];
//    let value:[i32; 8] = [255,127,63,31,15,7,3,1];
    //let value:[i32; 8] = [1,2,3,4,255,160,97,888];

    loop {

        reset_cols.set_low().void_unwrap();
            
        let c = column as usize;
            

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
