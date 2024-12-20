
#[actix_web::main]
async fn main() {
    let signature = common::http::oss::get_temp_signature().await;
    match signature {
        Ok(signature) => {
            println!("signature = {:?}", signature);
        }
        Err(e) => {
            println!("error = {:?}", e);
        }
    }

}

