// use std::collections::HashMap;

use serde::Deserialize;

// use crate::AppState;

#[derive(Deserialize, Clone)]
pub struct CreateInvoiceData {
  pub id: i32,
  pub qty: f64,
  pub price: f64
}

#[derive(Deserialize, Clone)]
pub struct UpdateInvoiceData {
  pub qty: f64,
}


// pub struct AllList {
//   pub all: HashMap<u32, AppState>
// }

