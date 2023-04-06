use crate::control_number::ControlNumber;
use serde::{Serialize};

// TODO: abstract components of the payload to their own files

const MANUFACTURER: &str = "BUSSE HOSPITAL DISPOSABLES";
const DUNS_NUMBER: &str = "0849233000001";

#[derive(Debug, Serialize)]
pub struct Payload {
    #[serde(rename = "default")]
    pub documents: Vec<Document>,
}

#[derive(Debug, Serialize)]
pub struct Document {
    #[serde(rename = "documentType")]
    pub document_type: String,
    #[serde(rename = "controlNumber")]
    pub control_number: i32,    
    pub datetimes: Vec<Datetime>,
    pub contracts: Vec<Contract>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DocumentType {
    New,
    Change,
    Resubmit,
    Renew,
    Cancel,
}

impl DocumentType {
    pub fn to_string(&self) -> String {
        match self {
            DocumentType::New => "00".to_string(),
            DocumentType::Change => "04".to_string(),
            DocumentType::Resubmit => "18".to_string(),
            DocumentType::Renew => "30".to_string(),
            DocumentType::Cancel => "56".to_string(),
        }
    }
}

pub enum DatetimeType {
    ContractEffective,
    ContractExpiration,
    ContractPriorExpiration,
    AgreementEffective,
    AgreementExpiration,
}

impl DatetimeType {
    pub fn to_string(&self) -> String {
        match self {
            DatetimeType::ContractEffective => "092".to_string(),
            DatetimeType::ContractExpiration => "093".to_string(),
            DatetimeType::ContractPriorExpiration => "094".to_string(),            
            DatetimeType::AgreementEffective => "129".to_string(),
            DatetimeType::AgreementExpiration => "130".to_string(),
        }
    }
}

pub enum ReferenceType {    
    BuyersContractNumber,
    ContractType,
    MutuallyDefined,
    AddDistributor,
}

impl ReferenceType {
    pub fn to_string(&self) -> String {
        match self {
            ReferenceType::BuyersContractNumber => "BC".to_string(),
            ReferenceType::ContractType => "CT".to_string(),
            ReferenceType::MutuallyDefined => "ZZ".to_string(),
            ReferenceType::AddDistributor => "TD".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Contract {
    #[serde(rename = "contractId")]
    pub contract_id: String,    
    pub references: Vec<Reference>,
    pub dealers: Vec<Dealer>,
    pub agreements: Vec<Agreement>,    
}

#[derive(Debug, Serialize)]
pub struct Agreement {
    #[serde(rename = "lineNumber")]
    pub line_number: i32,
    pub reference: String,
    pub details: Vec<Detail>,
    #[serde(rename = "lineInformation")]
    pub line_information: Vec<Line>,
    pub pricing: Vec<Pricing>,
}

pub enum DealerType {
    Manufacturer,
    BuyingGroup,
    EndUser,
}

impl DealerType {
    pub fn to_string(&self) -> String {
        match self {
            DealerType::Manufacturer => "MF".to_string(),
            DealerType::BuyingGroup => "BG".to_string(),
            DealerType::EndUser => "EB".to_string(),
        }
    }
}

pub enum IdentifierType {
    Duns,
    Hin,
    VendorDefined,
}

impl IdentifierType {
    pub fn to_string(&self) -> String {
        match self {
            IdentifierType::Duns => "UL".to_string(),
            IdentifierType::Hin => "21".to_string(),
            IdentifierType::VendorDefined => "92".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Dealer {
    pub group_type: String,
    pub group_name: String,
    pub id_type: String,
    pub id: String,
    pub address: Address,
    pub references: Vec<Reference>,
    pub datetimes: Vec<Datetime>,
}

#[derive(Debug, Serialize)]
pub struct Detail {
    pub description: String,    
}

#[derive(Debug, Serialize)]
pub struct Line {
    #[serde(rename = "lineNumber")]
    pub line_number: i32,
    #[serde(rename = "itemId")]
    pub item_id: String,    
}

#[derive(Debug, Serialize)]
pub struct Pricing {
    pub price: f32,
    pub quantity: i32,
    pub uom: String,
    pub datetimes: Vec<Datetime>,
}

#[derive(Debug, Serialize)]
pub struct Address {
    pub addr1: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

#[derive(Debug, Serialize)]
pub struct Datetime {
    #[serde(rename = "datetimeType")]
    pub datetime_type: String,
    pub datetime: String,
}

#[derive(Debug, Serialize)]
pub struct Reference {
    #[serde(rename = "referenceType")]
    pub reference_type: String,
    pub reference: String,
}

impl Payload {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
        }
    }

    pub fn add_document(&mut self, document: Document) {
        self.documents.push(document);
    }
}

impl Document {
    pub fn new(document_type: DocumentType) -> Self {
        let control_number = ControlNumber::to_i32(ControlNumber::create().as_str());
        
        Self {
            document_type: document_type.to_string(),
            control_number,
            datetimes: Vec::new(),
            contracts: Vec::new(),
        }
    }

    pub fn add_datetime(&mut self, datetime: Datetime) {
        self.datetimes.push(datetime);
    }

    pub fn add_contract(&mut self, contract: Contract) {
        self.contracts.push(contract);
    }
}

impl Contract {
    pub fn new(contract_id: &str) -> Self {
        let references = vec![Reference::new(
            ReferenceType::ContractType, "LOI"),
            Reference::new(ReferenceType::BuyersContractNumber, contract_id),
        ];
        
        let dealers: Vec<Dealer> = vec![Dealer::new(DealerType::Manufacturer, MANUFACTURER, IdentifierType::Duns, DUNS_NUMBER)];

        Self {            
            contract_id: contract_id.to_string(),            
            references,
            dealers,
            agreements: Vec::new(),            
        }
    }

    pub fn add_reference(&mut self, reference_type: ReferenceType, reference: &str) {
        self.references.push(Reference {
            reference_type: reference_type.to_string(),
            reference: reference.to_string(),
        });
    }

    pub fn add_dealer(&mut self, dealer: Dealer) {
        self.dealers.push(dealer);
    }

    pub fn add_agreement(&mut self, agreement: Agreement) {
        self.agreements.push(agreement);
    }

}

impl Agreement {
    pub fn new(line_number: &i32, reference: &str) -> Self {
        Self {
            line_number: *line_number,
            reference: reference.to_string(),
            details: Vec::new(),
            line_information: Vec::new(),
            pricing: Vec::new(),
        }
    }

    pub fn add_detail(&mut self, description: &str) {
        self.details.push(Detail {
            description: description.to_string(),
        });
    }

    pub fn add_line(&mut self, line_number: &i32, item_id: &str) {
        self.line_information.push(Line {
            line_number: *line_number,
            item_id: item_id.to_string(),
        });
    }

    pub fn add_pricing(&mut self, pricing: Pricing) {
        self.pricing.push(pricing);
    }
}

impl Dealer {
    pub fn new(group_type: DealerType, group_name: &str, id_type: IdentifierType, id: &str) -> Self {
        Self {
            group_type: group_type.to_string(),
            group_name: group_name.to_string(),
            id_type: id_type.to_string(),
            id: id.to_string(),
            address: Address {
                addr1: "".to_string(),
                city: "".to_string(),
                state: "".to_string(),
                zip: "".to_string(),
            },
            references: Vec::new(),
            datetimes: Vec::new(),
        }
    }

    pub fn set_address(&mut self, address: Address) {
        self.address = address
    }

    pub fn add_reference(&mut self, reference_type: ReferenceType, reference: &str) {
        self.references.push(Reference {
            reference_type: reference_type.to_string(),
            reference: reference.to_string(),
        });
    }

    pub fn add_datetime(&mut self, datetime: Datetime) {
        self.datetimes.push(datetime);
    }
}

impl Datetime {
    pub fn new(datetime_type: DatetimeType, datetime: &str) -> Self {
        Self {
            datetime_type: datetime_type.to_string(),
            datetime: datetime.to_string(),
        }
    }
}

impl Pricing {
    pub fn new(price: f32, quantity: i32, uom: &str, start: &str, end: &str) -> Self {
        Self {
            price: price,
            quantity: quantity,
            uom: uom.to_string(),
            datetimes: vec![
                Datetime::new(DatetimeType::AgreementEffective, start),
                Datetime::new(DatetimeType::AgreementExpiration, end),],
        }
    }
}

impl Reference {
    pub fn new(reference_type: ReferenceType, reference: &str) -> Self {
        Self {
            reference_type: reference_type.to_string(),
            reference: reference.to_string(),
        }
    }
}

// impl Detail {
//     pub fn new(description: &str) -> Self {
//         Self {
//             description: description.to_string(),
//         }
//     }
// }

// impl Line {
//     pub fn new(line_number: i32, item_id: &str) -> Self {
//         Self {
//             line_number: line_number,
//             item_id: item_id.to_string(),
//         }
//     }
// }

impl Address {
    pub fn new(addr1: &str, city: &str, state: &str, zip: &str) -> Self {
        Self {
            addr1: addr1.to_string(),
            city: city.to_string(),
            state: state.to_string(),
            zip: zip.to_string(),
        }
    }
}