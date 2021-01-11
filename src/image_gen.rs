use image::*;
use crate::gen8::Gen;

pub fn gen_8bit_map<G: Gen>(file_name: String, mut generator: G) {
    let mut img: GrayImage = GrayImage::new(256 * 9, 256);
    for x in 0..256 * 9 {
        for y in 0..256 {
            img.get_pixel_mut(x, y).0[0] = 255;
        }
    }
    for lag in 0..=8 {
        for _count in 0..=(256 * 256) {
            let a = generator.next();
            for _ in 0..lag {
                generator.next();
            }
            let b = generator.next();
            let pix = img.get_pixel_mut(a as u32 + (lag * 256), b as u32);
            pix.0[0] = ((pix.0[0].saturating_sub(32) as u32 * 3) / 4) as u8;
            //pix.0[0] /= 2;
        }
    }
    img.save(file_name + ".png").unwrap();
}

pub fn gen_large_map<G: Gen>(file_name: String, mut generator: G) {
    let mut img: GrayImage = GrayImage::new(2048, 2048);
    for x in 0..2048 {
        for y in 0..2048 {
            img.get_pixel_mut(x, y).0[0] = 255;
        }
    }
    for lag in 0..4 {
        let mut offset = 0;
        for &(config, reverse) in &[(true, true), (true, false), (false, true), (false, false)] {
            generator.reset(config);
            for _count in 0..=(u32::max_value() / 8) {
                let a = gen_16bit_value(&mut generator, reverse);
                if a >= 512 {
                    continue;
                }
                for _ in 0..lag {
                    generator.next();
                }
                let b = gen_16bit_value(&mut generator, reverse);
                if b >= 512 {
                    continue;
                }
                let pix = img.get_pixel_mut(a as u32 + offset * 512, b as u32 + lag * 512);
                pix.0[0] /= 2;//((pix.0[0].saturating_sub(32) as u32 * 3) / 4) as u8;
            }
            offset += 1;
        }
    }
    img.save(file_name + ".png").unwrap();
}

fn gen_16bit_value<G: Gen>(gen: &mut G, reverse: bool) -> u16 {
    let result = ((gen.next() as u16) << 8) | (gen.next() as u16);
    if reverse {
        result.reverse_bits()
    } else {
        result
    }
}