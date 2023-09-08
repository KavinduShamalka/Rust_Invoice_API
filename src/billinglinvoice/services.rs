use std::process::id;

use serde_json::json;

use actix_web::{get, post, put, delete, web, Responder, HttpResponse};

use crate::{AppState, InvoiceListEntry};

use super::models::{CreateInvoiceData, UpdateInvoiceData};

#[get("/invoicelist/entries")] 
async fn get_entries(data: web::Data<AppState>) -> impl Responder { 
  let item_collection = data.invoice_collection.lock().unwrap();

  HttpResponse::Ok().json(&*item_collection)
}

#[post("/invoicelist/entries/post")]
async fn create_entry(data: web::Data<AppState>, param_obj: web::Json<CreateInvoiceData>) -> impl Responder {

  let mut itemlist_entries = data.invoice_collection.lock().unwrap();

  let total = param_obj.price * param_obj.qty;

  if itemlist_entries.contains_key(&param_obj.id) {

    HttpResponse::Ok().json(json!({"message": "Invoice ID already exists."}));
    println!("Invoice ID is already exists.");

  } else {
    
    itemlist_entries.insert(param_obj.id.clone(), InvoiceListEntry {
      qty: param_obj.qty.clone(),
      price: param_obj.price,
      total
    });

    println!("Successfully Added.");
  
  }

  // let key = itemlist_entries.get_key_value(&param_obj.id);


  HttpResponse::Ok().json(&*itemlist_entries)

}


#[put("/invoicelist/entries/put/{id}")]
async fn update_entry(data: web::Data<AppState>, param_obj: web::Json<UpdateInvoiceData>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let mut invoice = data.invoice_collection.lock().unwrap();

 match invoice.get_mut(&id) {
  Some(item_list) => {
    item_list.qty += param_obj.qty;
    item_list.total = item_list.price * item_list.qty;
  },
  None => (),
 }
  HttpResponse::Ok().json(&*invoice)

}

#[delete("/invoicelist/entries/delete/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let mut invoicelist_entries =  data.invoice_collection.lock().unwrap();
  let id = path.into_inner();

  invoicelist_entries.remove(&id);

  HttpResponse::Ok().json(json!({"message": "Invoice ID Deleted."}));
  HttpResponse::Ok().json(&*invoicelist_entries)
}




pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(get_entries)
    .service(create_entry)
    .service(update_entry)
    .service(delete_entry);
}