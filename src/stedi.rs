use std::fs;

use oxhttp::{Client, model::{Request, Response, HeaderName, Method}};

pub fn make_api_call_to_stedi_for_edi_string(map_id: &str, api_key: &str, guide_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://mappings.stedi.com/2021-06-01/mappings/{}/map", map_id);
    let api_key = format!("Key {}", api_key);

    let combined_json = fs::read_to_string("combined.json").unwrap();    

    let client = Client::new();

    let mut request_builder = Request::builder(Method::POST, url.parse().unwrap());    
    request_builder.headers_mut().set(HeaderName::CONTENT_TYPE, "application/json".parse().unwrap());
    request_builder.headers_mut().set(HeaderName::AUTHORIZATION, api_key.parse().unwrap());
        
    let response: Response = client.request(request_builder.with_body(combined_json)).unwrap();
    
    let body = response.into_body().to_string().unwrap();

    // dbg!(&body);

    let edi_config = fs::read_to_string("stedi.json").unwrap();

    let edi_request_body = format!("{{\"guideId\": \"{}\", \"input\": {}, \"envelope\": {}}}", guide_id, body, edi_config);

    // dbg!(&edi_request_body);

    let mut edi_request_builder = Request::builder(Method::POST, "https://edi-translate.us.stedi.com/2022-01-01/x12/from-json".parse().unwrap());    
    edi_request_builder.headers_mut().set(HeaderName::CONTENT_TYPE, "application/json".parse().unwrap());
    edi_request_builder.headers_mut().set(HeaderName::AUTHORIZATION, api_key.parse().unwrap());

    let edi_response = client.request(edi_request_builder.with_body(edi_request_body)).unwrap();

    let edi_response_body = edi_response.into_body().to_string().unwrap();

    let edi_response_json: serde_json::Value = serde_json::from_str(&edi_response_body).unwrap();

    // dbg!(&edi_response_body);

    Ok(edi_response_json["output"].to_string())
}