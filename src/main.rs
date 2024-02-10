/*

MIT License

Copyright (c) 2024 Kaleb Alanis

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


                 _
               _(_)_                          wWWWw   _
   @@@@       (_)@(_)   vVVVv     _     @@@@  (___) _(_)_
  @@()@@ wWWWw  (_)\    (___)   _(_)_  @@()@@   Y  (_)@(_)
   @@@@  (___)     `|/    Y    (_)@(_)  @@@@   \|/   (_)\
    /      Y       \|    \|/    /(_)    \|      |/      |
 \ |     \ |/       | / \ | /  \|/       |/    \|      \|/
jgs|//   \\|///  \\\|//\\\|/// \|///  \\\|//  \\|//  \\\|//
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
aster - a Vigenère cipher cli in rust

*/

use std::env;
use std::io::{self};


struct AsterExecutionContext {
    decrypt: bool,
    key: String
}

fn derive_aster_context() -> AsterExecutionContext {
    let args: Vec<String> = env::args().collect();
    let mut decrypt = false;
    let mut key: String = String::new();
    for (i, token) in args.iter().skip(1).enumerate() {
        if token.eq_ignore_ascii_case("-d") {
            decrypt = true;
        } else if token.eq_ignore_ascii_case("-k") {
            if let Some(key_val) = args.get(i+2) {
                key = key_val.to_string();
            }
        }
    }
    AsterExecutionContext {
        decrypt,
        key: key
    }
}

fn validate_aster_context(Aster: &AsterExecutionContext) -> bool {
   Aster.key.len() > 0
}

fn shift_char_up(ch: u8, shift: u8) -> u8 {
    let sum = ch.checked_add(shift);
    match sum {
        Some(v) => v,
        None => {
            shift - (127-ch)
        }    
    }
    
}

fn shift_char_down(ch: u8, shift: u8) -> u8 {
    let sum = ch.checked_sub(shift);
    match sum {
        Some(v) => v,
        None => {
            127 - (shift - ch)
        }    
    }
}

fn vigenere(plaintext: String, key: &String, decrypt: bool) {
    let mut key_idx = 0;
    for ch in plaintext.chars() {
        let shift = key
                .chars()    
                .nth(key_idx);
        if shift.is_some() {
            let sv = shift.unwrap() as u8;
            let ch_ascii = ch as u8;
         
            let n = if decrypt { shift_char_down(ch_ascii, sv)} else { shift_char_up(ch_ascii, sv)};
            print!("{}", n as char)
        }
        key_idx += 1;
        key_idx = key_idx % key.len();
    }
    println!("");
}


fn handle_aster_execution(ctx: &AsterExecutionContext) {
    let stdin: io::Stdin = io::stdin();
    for (_, line) in stdin.lines().enumerate() {
        match line {
            Ok(s) => {
                vigenere(s, &ctx.key, ctx.decrypt);
            },
            _ => {}
        }
    }
}

    
fn main() {
    let ctx = derive_aster_context();
    if !validate_aster_context(&ctx) {
        eprint!("Aster requires a key to perform en(de)crypt actions on a plaintext");
        std::process::exit(1);
    }
    handle_aster_execution(&ctx);
}
