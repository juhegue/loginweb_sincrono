use std::env;
extern crate fernet;
use base64::{Engine as _, engine::{ general_purpose}};
use isahc::prelude::*;
use webbrowser;

fn encripta(dato: &[u8]) -> String{
    let keyb64_fernet = "Key_fernet_en_base_64";
    let key_bytes = general_purpose::STANDARD.decode(keyb64_fernet).unwrap();
    let key_str = String::from_utf8_lossy(&key_bytes);

    let fernet = fernet::Fernet::new(&key_str).unwrap();
    let dato_encriptado = fernet.encrypt(dato);
    return dato_encriptado;
}

fn main() {
    let url = "http://localhost:8000";

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {

        },
        _ => {
            let param = &args[1];
            let param_bytes = param.as_bytes();
            let encriptado = encripta(param_bytes);
            let full_url = format!("{}/version/login/?param={}", url, encriptado);
            
            if param.len() > 20 {            
                if &param[18..23] == "versi" {
                    if webbrowser::open(&full_url).is_ok() {
                        println!("{}", 0);
                    }
                } else {
                    let mut response = isahc::get(full_url).expect("ERROR get");
                    if response.status().is_success() {
                        let body = response.text().expect("ERROR body");
                        println!("{}", body);
                    } else {
                        println!("ERROR status: {}", response.status());
                    }
                }
             }
        }
    }

}
