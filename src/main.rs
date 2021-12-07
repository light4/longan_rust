#![no_std]
#![no_main]

use panic_halt as _;

use embedded_sdmmc::{Directory, Volume, VolumeIdx};

use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::mono_font::{ascii::FONT_8X13, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::raw::LittleEndian;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use longan_nano::hal::delay::McycleDelay;
use longan_nano::hal::{pac, prelude::*};
use longan_nano::lcd::Lcd;
use longan_nano::sdcard::SdCard;
use longan_nano::{lcd, lcd_pins, sdcard, sdcard_pins};
use riscv_rt::entry;

const FERRIS: &[u8] = include_bytes!("ferris.raw");
const BADAPPLE_FILENAME: &'static str = "bmp.bin";

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();
    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    let sdcard_pins = sdcard_pins!(gpiob);
    let mut sdcard = sdcard::configure(dp.SPI1, sdcard_pins, sdcard::SdCardFreq::Safe, &mut rcu);

    let mut delay = McycleDelay::new(&rcu.clocks);
    myprint(&mut lcd, " Hello Rust! ");
    delay.delay_ms(800);

    // Clear screen
    Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(&mut lcd)
        .unwrap();

    // Load Image Data
    let raw_image: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(&FERRIS, 86);
    Image::new(&raw_image, Point::new(width / 2 - 43, height / 2 - 32))
        .draw(&mut lcd)
        .unwrap();

    if let Err(_) = sdcard.device().init() {
        // Create a text at position (20, 30) and draw it using style defined above
        myprint(&mut lcd, " No SDCard! ");
    } else {
        let _ = sdcard.device().card_size_bytes().unwrap();

        // open the first partition
        let mut volume = sdcard.get_volume(VolumeIdx(0)).unwrap();

        let root_dir = sdcard.open_root_dir(&volume).unwrap();
        if let Ok(_) = sdcard.find_directory_entry(&volume, &root_dir, BADAPPLE_FILENAME) {
            play_badapple(&mut lcd, &mut sdcard, &mut volume, &root_dir);
        }
    }
    myprint(&mut lcd, "Done");
    delay.delay_ms(800);

    loop {}
}

fn play_badapple(lcd: &mut Lcd, sdcard: &mut SdCard, volume: &mut Volume, dir: &Directory) {
    let mut file = sdcard
        .open_file_in_dir(
            volume,
            dir,
            BADAPPLE_FILENAME,
            embedded_sdmmc::Mode::ReadOnly,
        )
        .unwrap();
    const OFFSET: usize = 25600;
    let mut buffer: [u8; OFFSET] = [0; OFFSET];
    for _ in 0..2189 {
        sdcard.read(volume, &mut file, &mut buffer).unwrap();
        let raw_image: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(&buffer, 160);
        Image::new(&raw_image, Point::new(0, 0)).draw(lcd).unwrap();
        file.seek_from_current(OFFSET.try_into().unwrap()).unwrap();
    }
    sdcard.close_file(volume, file).unwrap();
}

fn myprint(lcd: &mut Lcd, s: &str) {
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    // Clear screen
    Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(lcd)
        .unwrap();

    let style = MonoTextStyleBuilder::new()
        .font(&FONT_8X13)
        .text_color(Rgb565::BLACK)
        .background_color(Rgb565::GREEN)
        .build();

    // Create a text at position (20, 30) and draw it using style defined above
    Text::new(s, Point::new(40, 35), style).draw(lcd).unwrap();
}
