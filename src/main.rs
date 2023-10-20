use std::env;
extern crate fernet;
use base64::{Engine as _, engine::{ general_purpose}};
use webbrowser;

fn encripta(dato: &[u8]) -> String{
    let keyb64_fernet = "Key_fernet_en_base_64";
    let key_bytes = general_purpose::STANDARD.decode(keyb64_fernet).unwrap();
    let key_str = String::from_utf8_lossy(&key_bytes);

    let fernet = fernet::Fernet::new(&key_str).unwrap();
    let dato_encriptado = fernet.encrypt(dato);
    return dato_encriptado;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let diagram_url = "http://localhost:8000/cambio_version/login/?param=";

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            return Ok(());
        },
        _ => {
            let query = &args[1];
            let query_bytes = query.as_bytes();

            let check = "check".to_string() + query;
            let check_bytes = check.as_bytes();

            let mut encriptado = encripta(check_bytes);
            let mut url = diagram_url.to_string() + &encriptado;

            let resp = reqwest::get(url)
                .await?
                .text()
                .await?;

            let cuantos: Result<i32, std::num::ParseIntError> = resp.parse();

            match cuantos {
                Ok(valor) => {
                    if valor > 0 {
                        encriptado = encripta(query_bytes);
                        url = diagram_url.to_string() + &encriptado;
                        if webbrowser::open(&url).is_ok() {
                            println!("{}", valor);
                            //println!("{:#?}", resp);
                        }
                    }
                }
                Err(_) => {
                    // println!("No se pudo realizar la conversión a entero.");
                }
            }
        }
    }
    Ok(())
}
