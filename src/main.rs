
use std::error::Error;
use std::io;
use std::io::{BufWriter, Write};

use rand_core::RngCore;

use pcg_mwc::Mwc256XXA64;

mod gen8;
mod gen16;
mod gen32;
mod parm_search;
mod image_gen;

//const MLAG3: u64 = 0xfeb344657c0af413;
//const MLAG2: u64 = 0x7c0a1367d4953bad;

fn main() -> Result<(), Box<dyn Error>> {
    let out = io::stdout();
    let mut handle = BufWriter::new(out.lock());
    let mut mwc = Mwc256XXA64::new(1, 1);
    loop {
        let value = mwc.next_u64();
        handle.write_all(&value.to_le_bytes())?;
    }

 //   gen_8_bit_candidates();
 //   gen_64_bit_candidates();

    //image_gen::gen_8bit_map("32+1h".to_owned(), gen8::Gen8::default());
    // image_gen::gen_8bit_map("lcg-xsh-rr".to_owned(), gen8::Lcg{ x: 123 });
    // image_gen::gen_8bit_map("xoshiro32++".to_owned(), gen8::xoshiro{ s: [1, 0, 0, 0] });
    //image_gen::gen_large_map("large-32+1h".to_owned(), gen8::Gen8::default());
    // image_gen::gen_large_map("large-lcg-xsh-rr".to_owned(), gen8::Lcg{ x: 123 });
  // image_gen::gen_large_map("large2-xoshiro32++".to_owned(), gen8::Xoshiro::default());

    Ok(())
}