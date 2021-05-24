use image::*;
use crate::gen8::Gen;

pub fn flip_sideways(input: [u8; 8]) -> [u8; 8] {
    let mut result = [0_u8; 8];
    for &val in input.iter() {
        for pos in 0..8 {
            let bit = (val >> pos) & 1;
            result[pos] |= bit;
            result[pos] = result[pos].rotate_right(1);
        }
    }
    result
}

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
    let mut img: GrayImage = GrayImage::new(2048, 1024);
    for x in 0..2048 {
        for y in 0..1024 {
            img.get_pixel_mut(x, y).0[0] = 255;
        }
    }
    let mut offset = 0;
    for &reverse in &[true, false] {
        generator.reset();
        for _count in 0..=(u32::max_value() / 4) {
            let (a, b) = gen_16bit_values(&mut generator, reverse);
            if a >= 1024 || b >= 1024 {
                continue;
            }
            let pix = img.get_pixel_mut(a as u32 + offset * 1024, b as u32);
            pix.0[0] /= 2;//((pix.0[0].saturating_sub(32) as u32 * 3) / 4) as u8;
        }
        offset += 1;
    }
    img.save(file_name + ".png").unwrap();
}

pub fn gen_sideways_large_map<G: Gen>(file_name: String, mut generator: G, f: usize, a: usize, b: usize) {
    let mut img: GrayImage = GrayImage::new(4096, 4096);
    for x in 0..4096 {
        for y in 0..4096 {
            img.get_pixel_mut(x, y).0[0] = 255;
        }
    }
    for _count in 0..=256*256*256 {
       // let mut offset = 0;
        let sequences = gen_sideways_values(&mut generator);
        for x in 0..16 {
            for y in 0..16 {
                if sequences[f] != (x << 4) + y {
                    continue;
                }
                let pix = img.get_pixel_mut(sequences[a] as u32 + x as u32 * 256, sequences[b] as u32 + y as u32 * 256);
                pix.0[0] = ((pix.0[0].saturating_sub(32) as u32 * 3) / 4) as u8;
               // offset += 1024;
            }
        }
    }
    img.save(file_name + ".png").unwrap();
}


fn gen_16bit_values<G: Gen>(gen: &mut G, reverse: bool) -> (u16, u16) {
    let val = [gen.next(), gen.next(), gen.next(), gen.next()];
    let r1 = ((val[0] as u16) << 8) | (val[2] as u16);
    let r2 = ((val[1] as u16) << 8) | (val[3] as u16);
    if reverse {
        (r1.reverse_bits(), r2.reverse_bits())
    } else {
        (r1, r2)
    }
}

fn gen_sideways_values<G: Gen>(gen: &mut G) -> [u8; 8] {
    let mut pre_flip = [0; 8];
    for idx in &mut pre_flip {
        *idx = gen.next();
    }
    flip_sideways(pre_flip)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flip_perspective() {
        let input: [u8;8] = [0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x7];
        let output = flip_sideways(input);
        assert_eq!(output, [0xAA,0xCC,0xF0,0x00,0x00,0x00,0x00,0x00]);
        let input: [u8;8] = [0x00,0x10,0x20,0x30,0x40,0x50,0x60,0x70];
        let output = flip_sideways(input);
        assert_eq!(output, [0x00,0x00,0x00,0x00,0xAA,0xCC,0xF0,0x00]);
    }
}