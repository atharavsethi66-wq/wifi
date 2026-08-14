//HTTP request

use reqwest::Client;
use std::collections::HashMap;
// use std::string;
use std::time::{SystemTime, UNIX_EPOCH};

use quick_xml::events::Event;
use quick_xml::Reader;

pub async fn login(name:&str,pass:&str)-> Result<String,reqwest::Error>{

    let a = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut data=HashMap::new();

    data.insert("mode","191".to_string());

    data.insert("username",name.to_string());
    data.insert("password",pass.to_string());
    data.insert("a",a.to_string());
    data.insert("producttype", "0".to_string());

    let client=Client::new();// for sending http request to server

    let response=client
                        .post("http://172.16.16.16:8090/login.xml")
                        .form(&data)
                        .send()
                        .await?;
        // if response pass then stirng is given 
        // if not 

        let body=response.text().await?;
        //   reading the entire xml as a string 
        // use quick-xml for cml parser to as we need it to verify the status 
Ok(body)//so ok return type is result
// Since your XML is:

// <requestresponse>
//     <status><![CDATA[LIVE]]></status>
//     <message><![CDATA[You are signed in as {username}]]></message>
// </requestresponse>

} 

pub fn check_status(xml: &str) -> Option<String> {
    let mut reader = Reader::from_str(xml);
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"status" {
                    match reader.read_event() {
                        Ok(Event::CData(data)) => {
                            return Some(data.decode().unwrap().into_owned());
                        }
                        Ok(Event::Text(text)) => {
                            return Some(text.decode().unwrap().into_owned());
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    None
}

// For example:  xml
// <requestresponse>
//     <status>LIVE</status>
//     <message>Hello</message>
// </requestresponse>

// The reader needs to inspect:

// Event 1 → requestresponse
// Event 2 → status
// Event 3 → LIVE
// Event 4 → status end
// Event 5 → message