mod config;
mod control_number;
mod payload;
mod read_csv;
mod to_json;
mod stedi;

use std::fs::File;
use std::io::prelude::*;

use chrono::NaiveDate;
use clap::Parser;

use crate::config::Config;
use crate::payload::*;
use crate::read_csv::*;
use crate::to_json::*;
use crate::stedi::*;

const CASE: &str = "CA";
const ADD: &str = "AI";
const CHANGE: &str = "CI";
const DELETE: &str = "DI";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    contract_number: String,    
    #[clap(short, long, value_parser)]
    buyer_file: String,
    #[clap(short, long, value_parser)]
    start_date: String,
    #[clap(short, long, value_parser)]
    end_date: String,
    #[clap(short, long, value_parser)]
    purpose: String,
    
    #[clap(short, long, value_parser, default_value = "")]
    new_end_date_if_any: String,
    #[clap(short, long, value_parser, default_value = "")]
    outgoing_contract_number_if_any: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    // println!("{:?}", config);

    let args: Args = Args::parse();
    // println!("{:?}", args);

    let purpose = match args.purpose.as_str() {
        "new" => DocumentType::New,
        "change" => DocumentType::Change,
        "resubmit" => DocumentType::Resubmit,
        "renew" => DocumentType::Renew,
        "cancel" => DocumentType::Cancel,        
        _ => DocumentType::New,
    };    

    let mut payload = Payload::new();    

    if (purpose.clone() == DocumentType::Cancel || purpose.clone() == DocumentType::Renew) && &args.new_end_date_if_any == "" {
        panic!("New end date is required for cancel or renew purposes.");
    }

    let mut document = Document::new(purpose.clone());
    document.add_datetime(Datetime::new(DatetimeType::ContractEffective, &args.start_date));    
    
    if &args.new_end_date_if_any != "" {
        document.add_datetime(Datetime::new(DatetimeType::ContractExpiration, &args.new_end_date_if_any));
        document.add_datetime(Datetime::new(DatetimeType::ContractPriorExpiration, &args.end_date));
    } else {
        document.add_datetime(Datetime::new(DatetimeType::ContractExpiration, &args.end_date));
    }

    let mut contract = Contract::new(args.contract_number.as_str());        

    match purpose {
        DocumentType::New => {
            if &args.outgoing_contract_number_if_any != "" {
                contract.add_reference(ReferenceType::MutuallyDefined, &args.outgoing_contract_number_if_any);
            }

            let buyer_data = parse_csv::<EndBuyerData>(config.get_buyers_path(args.buyer_file.as_str()).as_path());    

            for buyer in buyer_data.into_iter() {
                let mut processed = Dealer::new(match buyer.name.as_str() {
                    "MEDASSETS" => DealerType::BuyingGroup,
                    "PREMIER HOSPITALS" => DealerType::BuyingGroup,
                    _ => DealerType::EndUser,
                }, buyer.name.as_str(), match buyer.name.as_str() {
                    "MEDASSETS" => IdentifierType::Hin,
                    "PREMIER HOSPITALS" => IdentifierType::Hin,
                    _ => IdentifierType::VendorDefined,
                }, buyer.id.as_str());

                if buyer.address != "" {
                    processed.set_address(Address::new(buyer.address.as_str(), buyer.city.as_str(), buyer.state.as_str(), buyer.zipcode.as_str()));
                }

                processed.add_reference(ReferenceType::AddDistributor, &buyer.change);

                if buyer.start != "" {            
                    processed.add_datetime(Datetime::new(DatetimeType::AgreementEffective, match buyer.start.len() {
                        8 => format!("{}-{}-{}", &buyer.start[0..4], &buyer.start[4..6], &buyer.start[6..8]),
                        _ => buyer.start,
                    }.as_str()));
                }

                if buyer.end != "" {            
                    processed.add_datetime(Datetime::new(DatetimeType::AgreementExpiration, match buyer.end.len() {
                        8 => format!("{}-{}-{}", &buyer.end[0..4], &buyer.end[4..6], &buyer.end[6..8]),
                        _ => buyer.end,
                    }.as_str()));
                }
                
                contract.add_dealer(processed);
            };

            let contract_data = parse_csv::<ContractData>(config.get_contracts_path(args.contract_number.as_str()).as_path());    
                
            for (idx, agreement) in contract_data.iter().enumerate() {
                let line_number: i32 = (idx + 1).try_into().unwrap();
                
                let mut processed = Agreement::new(&line_number, match agreement.purpose.as_str() {
                    "add" => ADD,
                    "delete" => DELETE,
                    "change" => CHANGE,
                    _ => ADD,
                });
                
                processed.add_detail(agreement.description.as_str());
                processed.add_line(&line_number, agreement.part.as_str());
                processed.add_pricing(Pricing::new(agreement.price, 1, CASE, match agreement.start.len() {
                    8 => format!("{}-{}-{}", &agreement.start[0..4], &agreement.start[4..6], &agreement.start[6..8]),
                    _ => agreement.start.clone(),            
                }.as_str(), match args.end_date.len() {
                    8 => format!("{}-{}-{}", &args.end_date[0..4], &args.end_date[4..6], &args.end_date[6..8]),
                    _ => args.end_date.clone(),
                }.as_str()));        

                contract.add_agreement(processed);
            }

            document.add_contract(contract);
            payload.add_document(document);
        },
        // change type - the buyer and contract csv files should contain only changes
        DocumentType::Change => {
            let buyer_data = parse_csv::<EndBuyerData>(config.get_buyers_path(args.buyer_file.as_str()).as_path());

            if buyer_data.len() > 0 {
                for buyer in buyer_data.into_iter() {
                    let mut processed = Dealer::new(match buyer.name.as_str() {
                        "MEDASSETS" => DealerType::BuyingGroup,
                        "PREMIER HOSPITALS" => DealerType::BuyingGroup,
                        _ => DealerType::EndUser,
                    }, buyer.name.as_str(), match buyer.name.as_str() {
                        "MEDASSETS" => IdentifierType::Hin,
                        "PREMIER HOSPITALS" => IdentifierType::Hin,
                        _ => IdentifierType::VendorDefined,
                    }, buyer.id.as_str());

                    if buyer.address != "" {
                        processed.set_address(Address::new(buyer.address.as_str(), buyer.city.as_str(), buyer.state.as_str(), buyer.zipcode.as_str()));
                    }

                    if buyer.start != "" {            
                        processed.add_datetime(Datetime::new(DatetimeType::AgreementEffective, match buyer.start.len() {
                            8 => format!("{}-{}-{}", &buyer.start[0..4], &buyer.start[4..6], &buyer.start[6..8]),
                            _ => buyer.start,
                        }.as_str()));
                    }

                    if buyer.end != "" {            
                        processed.add_datetime(Datetime::new(DatetimeType::AgreementExpiration, match buyer.end.len() {
                            8 => format!("{}-{}-{}", &buyer.end[0..4], &buyer.end[4..6], &buyer.end[6..8]),
                            _ => buyer.end,
                        }.as_str()));
                    }
                    
                    contract.add_dealer(processed);
                };
            }

            let contract_data = parse_csv::<ContractData>(config.get_contracts_path(args.contract_number.as_str()).as_path());    
                
            if contract_data.len() > 0 {
                for (idx, agreement) in contract_data.iter().enumerate() {
                    let line_number: i32 = (idx + 1).try_into().unwrap();
                    
                    let mut processed = Agreement::new(&line_number, match agreement.purpose.as_str() {
                        "add" => ADD,
                        "delete" => DELETE,
                        "change" => CHANGE,
                        _ => ADD,
                    });
                    
                    processed.add_detail(agreement.description.as_str());
                    processed.add_line(&line_number, agreement.part.as_str());
                    processed.add_pricing(Pricing::new(agreement.price, 1, CASE, match agreement.start.len() {
                        8 => format!("{}-{}-{}", &agreement.start[0..4], &agreement.start[4..6], &agreement.start[6..8]),
                        _ => agreement.start.clone(),            
                    }.as_str(), match args.end_date.len() {
                        8 => format!("{}-{}-{}", &args.end_date[0..4], &args.end_date[4..6], &args.end_date[6..8]),
                        _ => args.end_date.clone(),
                    }.as_str()));        

                    contract.add_agreement(processed);
                }                                
            }

            document.add_contract(contract);
            payload.add_document(document);
        },
        DocumentType::Resubmit => {
            if &args.outgoing_contract_number_if_any != "" {
                contract.add_reference(ReferenceType::MutuallyDefined, &args.outgoing_contract_number_if_any);
            }

            let buyer_data = parse_csv::<EndBuyerData>(config.get_buyers_path(args.buyer_file.as_str()).as_path());    

            for buyer in buyer_data.into_iter() {
                let mut processed = Dealer::new(match buyer.name.as_str() {
                    "MEDASSETS" => DealerType::BuyingGroup,
                    "PREMIER HOSPITALS" => DealerType::BuyingGroup,
                    _ => DealerType::EndUser,
                }, buyer.name.as_str(), match buyer.name.as_str() {
                    "MEDASSETS" => IdentifierType::Hin,
                    "PREMIER HOSPITALS" => IdentifierType::Hin,
                    _ => IdentifierType::VendorDefined,
                }, buyer.id.as_str());

                if buyer.address != "" {
                    processed.set_address(Address::new(buyer.address.as_str(), buyer.city.as_str(), buyer.state.as_str(), buyer.zipcode.as_str()));
                }

                if buyer.start != "" {            
                    processed.add_datetime(Datetime::new(DatetimeType::AgreementEffective, match buyer.start.len() {
                        8 => format!("{}-{}-{}", &buyer.start[0..4], &buyer.start[4..6], &buyer.start[6..8]),
                        _ => buyer.start,
                    }.as_str()));
                }

                if buyer.end != "" {            
                    processed.add_datetime(Datetime::new(DatetimeType::AgreementExpiration, match buyer.end.len() {
                        8 => format!("{}-{}-{}", &buyer.end[0..4], &buyer.end[4..6], &buyer.end[6..8]),
                        _ => buyer.end,
                    }.as_str()));
                }
                
                contract.add_dealer(processed);
            };

            let contract_data = parse_csv::<ContractData>(config.get_contracts_path(args.contract_number.as_str()).as_path());    
                
            for (idx, agreement) in contract_data.iter().enumerate() {
                let line_number: i32 = (idx + 1).try_into().unwrap();
                
                let mut processed = Agreement::new(&line_number, match agreement.purpose.as_str() {
                    "add" => ADD,
                    "delete" => DELETE,
                    "change" => CHANGE,
                    _ => ADD,
                });
                
                processed.add_detail(agreement.description.as_str());
                processed.add_line(&line_number, agreement.part.as_str());
                processed.add_pricing(Pricing::new(agreement.price, 1, CASE, match agreement.start.len() {
                    8 => format!("{}-{}-{}", &agreement.start[0..4], &agreement.start[4..6], &agreement.start[6..8]),
                    _ => agreement.start.clone(),            
                }.as_str(), match args.end_date.len() {
                    8 => format!("{}-{}-{}", &args.end_date[0..4], &args.end_date[4..6], &args.end_date[6..8]),
                    _ => args.end_date.clone(),
                }.as_str()));        

                contract.add_agreement(processed);
            }    

            document.add_contract(contract);
            payload.add_document(document);
        },
        DocumentType::Renew => {
            let end_date = NaiveDate::parse_from_str(args.end_date.as_str(), "%Y-%m-%d").unwrap();
            let new_end_date = NaiveDate::parse_from_str(args.new_end_date_if_any.as_str(), "%Y-%m-%d").unwrap();

            if end_date > new_end_date {
                panic!("New end date must be greater than current end date");                
            }

            document.add_contract(contract);
            payload.add_document(document);
        },
        DocumentType::Cancel => {
            let end_date = NaiveDate::parse_from_str(args.end_date.as_str(), "%Y-%m-%d").unwrap();
            let new_end_date = NaiveDate::parse_from_str(args.new_end_date_if_any.as_str(), "%Y-%m-%d").unwrap();

            if end_date < new_end_date {
                panic!("New end date must be less than current end date");                
            }
            document.add_contract(contract);
            payload.add_document(document);
        },
    }

    // println!("{:?}", payload);

    // save payload to json file
    to_json::<Payload>(&payload);
    // combine payload with schema file
    combine_schema_with_output_to_json();

    // send post request to api to turn json into mapped_json
    let (map_id, api_key, guide_id) = config.get_stedi_params();
    let edi_string = make_api_call_to_stedi_for_edi_string(map_id, api_key, guide_id)?;

    // dbg!(&edi_string);

    // save edi_string to output.edi
    write_to_file(&edi_string)?;

    Ok(())
}

fn write_to_file(data: &str) -> Result<(), Box<dyn std::error::Error>> {    
    let mut file = File::create("output.edi")?;

    let lines: Vec<&str> = data.trim_start_matches("\"").trim_end_matches("\"").split("\\n").collect();
    let mut count: i32 = 0;

    // dbg!(&lines);

    for line in lines {        
        if regex::Regex::new(r"^N(3|4)$").unwrap().is_match(line) {
            count += 1;
            continue;
        }        
        if regex::Regex::new(r"^SE\*").unwrap().is_match(line) {
            let parts: Vec<&str> = line.split("*").collect();
            let new_count = parts[1].parse::<i32>().unwrap() - count;

            // dbg!(&parts);
            // dbg!(&new_count);

            writeln!(file, "{}*{}*{}", parts[0], new_count, parts[2])?;
            continue;
        }
        writeln!(file, "{}", line)?;
    }

    Ok(())
}