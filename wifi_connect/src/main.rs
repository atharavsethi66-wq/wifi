use std::io;

use wifi_connect::routes::wifi;


#[tokio::main]
async fn main()
{
    let mut name=String::new();
    let mut pass=String::new();

    println!("Username :");
    io::stdin().read_line(&mut name).unwrap();

    println!("pass :");
    io::stdin().read_line(&mut pass).unwrap();

    let name=name.trim();
    let pass=pass.trim();

    // inside ok any variabke name 
    match wifi::login(name, pass).await {
    Ok(response) => {
        match wifi::check_status(&response) {
            Some(status) => {
                println!("Status: {}", status);
                if status == "LIVE" {
                    println!("Successfully logged in!");
                } else {
                    println!("Login failed.");
                }
            }
            None => {
                println!("Could not find status.");
            }
        }
    }
    Err(error) => {
        println!("Request failed: {}", error);
    }
}
}