#![no_std]
#![no_main]

const FRAME_TIME: u32 = 20;

use panic_halt as _;

use riscv_rt::entry;
use longan_nano::hal::{pac, prelude::*, delay::McycleDelay};
use longan_nano::lcd_pins;
use longan_nano::lcd::{self, Lcd};
use longan_nano::led::{Led, rgb};
use embedded_hal::blocking::delay::DelayMs;
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::{Rgb565, raw::RawU16};
use embedded_graphics::primitives::{Rectangle, PrimitiveStyleBuilder, Styled};

fn draw_rect<C>(lcd: &mut Lcd, ul: (i32, i32), lr: (i32, i32), c: C)
    where C: Into<Rgb565>
{
    let tl = Point::new(ul.0, ul.1);
    let size = Size::new((lr.0 - ul.0) as u32, (lr.1 - ul.1) as u32);
    let rect =  Rectangle::new(tl, size);
    let style = PrimitiveStyleBuilder::new()
        .fill_color(c.into())
        .build();
    let srect = Styled::new(rect, style);
    let _ = srect.draw(lcd);
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks.
    let mut rcu = dp.RCU.configure().ext_hf_clock(8.mhz()).sysclk(108.mhz()).freeze();

    // Take peripherals.
    let mut afio = dp.AFIO.constrain(&mut rcu);
    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

    let (mut red, mut green, mut blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    let mut leds: [&mut dyn Led; 3] = [&mut red, &mut green, &mut blue];

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (160, 80);
    let (ball_width, ball_height) = (4, 4);

    let mut delay = McycleDelay::new(&rcu.clocks);

    // Blacken LEDs
    for c in &mut leds {
        c.off();
    }

    // Clear screen
    draw_rect(
        &mut lcd,
        (0, 0),
        (width, height),
        RawU16::from(0u16),
    );

    let mut left = 1.0f32;
    let mut top = 1.0f32;
    let mut dx = 0.6f32;
    let mut dy = 0.8f32;
    let mut c = 0;
    let m = 64 * leds.len();
    #[allow(clippy::identity_op)]
    static BALL_COLORS: [u16; 8] = [
        (0x03 << 11) + (0x00 << 5) + 0x07,
        (0x07 << 11) + (0x00 << 5) + 0x03,
        (0x1f << 11) + (0x00 << 5) + 0x00,
        (0x07 << 11) + (0x03 << 5) + 0x00,
        (0x03 << 11) + (0x07 << 5) + 0x00,
        (0x00 << 11) + (0x1f << 5) + 0x03,
        (0x00 << 11) + (0x07 << 5) + 0x07,
        (0x00 << 11) + (0x03 << 5) + 0x1f,
    ];
    loop {
        // Change LED color.
        leds[c / 64].off();
        c = (c + 1) % m;
        leds[c / 64].on();

        // Get integer coords.
        let ileft = left as i32;
        let itop = top as i32;

        // Erase ball.
        draw_rect(
            &mut lcd,
            (ileft, itop),
            (ileft + ball_width - 1, itop + ball_height - 1),
            RawU16::from(0u16),
        );

        // Update ball position.
        if ileft <= 0 || ileft + ball_width >= width {
            dx = -dx;
        }
        left += dx;
        if itop <= 0 || itop + ball_height >= height {
            dy = -dy;
        }
        top += dy;
        let ileft = left as i32;
        let itop = top as i32;

        // Draw ball.
        draw_rect(
            &mut lcd,
            (ileft, itop),
            (ileft + ball_width - 1, itop + ball_height - 1),
            RawU16::from(BALL_COLORS[c / 24]),
        );

        // Delay to show ball.
        delay.delay_ms(FRAME_TIME);
    }
}
