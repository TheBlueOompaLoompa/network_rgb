use rs_ws281x::{ChannelBuilder, StripType, ControllerBuilder};
use tokio;
use delay::{self, Delay};

//use colors_transform::{Rgb, Color};

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
                .brightness(80) // default: 255
                .build(),
        )
        .build().unwrap();

    //let mut hue: f32 = 0.0;

    loop {
        let leds = controller.leds_mut(0);

        /*hue = hue + 0.01;
        if hue > 359.00 { hue = 0.0; }*/

        /*let color = Rgb::from(255.0, 0.0, 0.0);
        color.set_hue(hue);*/

        //println!("{:?}", color.as_tuple());

        leds.fill([255, 255, 255, 255]);

        controller.render().unwrap();
    }
}