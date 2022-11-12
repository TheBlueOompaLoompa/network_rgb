use rs_ws281x::{ChannelBuilder, StripType, ControllerBuilder};
use tokio;
use color::Hsv;

fn main() {
    let led_count = std::env::args().nth(1).expect("no led count").parse().unwrap_or(10);
    let color_r = std::env::args().nth(2).expect("no r").parse().unwrap_or(10u8);
    let color_g = std::env::args().nth(3).expect("no g").parse().unwrap_or(10u8);
    let color_b = std::env::args().nth(4).expect("no b").parse().unwrap_or(10u8);
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(18) // GPIO 10 = SPI0 MOSI
                .count(led_count) // Number of LEDs
                .strip_type(StripType::Ws2811Gbr)
                .brightness(127) // default: 255
                .build(),
        )
        .build().unwrap();

    let mut hue: f64 = 0;

    loop {
        let leds = controller.leds_mut(0);

        hue = hue + 0.01;
        if hue > 1 { hue = 0; }

        controller.render().unwrap();   
    }
}