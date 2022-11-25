use rs_ws281x::{ChannelBuilder, StripType, ControllerBuilder};
use tokio;
use palette;

fn main() {
    let mut led_count = std::env::args().nth(1).unwrap_or("-1".to_string()).parse().expect("Failed to parse number of leds!");

    if led_count == -1 {
        println!("Warning: Missing led count, defaulting to 10.");
        led_count = 10;
    }

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

    let mut hue: f64 = 0.0;

    loop {
        let leds = controller.leds_mut(0);

        hue = hue + 0.01;
        if hue > 1.0 { hue = 0.0; }

        let color_rgba = palette::RgbHue::from_degrees(hue);
        println!("{:?}", color_rgba);

        controller.render().unwrap();
    }
}