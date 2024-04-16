use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    id: u64,
    ofd_id: String,
    receive_date: String,
    subtype: String,
    address: String,
    content: Content,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Content {
    message_fiscal_sign: u64,
    code: u32,
    fiscal_document_format_ver: u32,
    fiscal_drive_number: String,
    kkt_reg_id: String,
    user_inn: String,
    fiscal_document_number: u32,
    date_time: u64,
    fiscal_sign: u32,
    shift_number: u32,
    request_number: u32,
    operation_type: u32,
    total_sum: u32,
    operator: String,
    items: Vec<Item>,
    nds_no: u32,
    user: String,
    retail_place_address: String,
    retail_place: String,
    applied_taxation_type: u32,
    fns_url: String,
    cash_total_sum: u32,
    ecash_total_sum: u32,
    prepaid_sum: u32,
    credit_sum: u32,
    provision_sum: u32,
    region: String,
    number_kkt: String,
    redefine_mask: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    name: String,
    price: u32,
    quantity: f64,
    nds: u32,
    nds_sum: u32,
    product_type: u32,
    payment_type: u32,
    sum: u32,
    items_quantity_measure: u32,
}