#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::prelude::*;
use longan_nano::lcd_pins;
use longan_nano::lcd::Lcd;
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::Rectangle;

fn draw_rect<C>(lcd: &mut Lcd, ul: (i32, i32), lr: (i32, i32), c: C)
    where C: Into<Rgb565>
{
    lcd
        .draw(Rectangle::new(
            Coord::new(ul.0, ul.1),
            Coord::new(lr.0, lr.1),
        )
        .fill(Some(c.into())));
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp.RCU.configure().ext_hf_clock(8.mhz()).sysclk(108.mhz()).freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = Lcd::new(dp.SPI0, lcd_pins, &mut rcu);
    let (width, height) = (160, 80);
    let (ball_width, ball_height) = (4, 4);

    // Clear screen
    draw_rect(
        &mut lcd,
        (0, 0),
        (width, height),
        0u16,
    );

    // Draw white square at center.
    let (left, top) = ((width - ball_width) / 2, (height - ball_height) / 2);
    draw_rect(
        &mut lcd,
        (left, top),
        (left + ball_width - 1, top + ball_height - 1),
        0xffffu16,
    );

    loop {}
}
