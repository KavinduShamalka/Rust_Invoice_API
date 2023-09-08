use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};

use std::{sync::Mutex, collections::HashMap};

mod billinglinvoice; //Import the todo list module
use billinglinvoice::services; //Using todolist module, access the services

//Create a structure for AppState that has field name "todolist_entries" it stores "TodoListEntry" structure in mutex vector
pub struct AppState {
    // itemlist_entries: Mutex<Vec<InvoiceListEntry>>,
    invoice_collection: Mutex<HashMap<i32, InvoiceListEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct InvoiceListEntry {
    qty: f64,
    price: f64,
    total: f64
}


#[get("/")] //get route
async fn index() -> String {
    "This is a billing invoice api".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //creates a new instance of the web::Data type.
    let app_data = web::Data::new(AppState {
        invoice_collection: Mutex::new(HashMap::new())
    });

    // sets up an instance of Actix's HttpServer
    HttpServer::new(move || { // move keyword is used to take ownership of the app_data variable inside the closure. 
        App::new() //creates a new instance of Actix's App, which is a builder for configuring an Actix web application.
            .app_data(app_data.clone()) 
            .service(index)
            .configure(services::config)//configures the application by calling the config function from the services module/
    })
    .bind(("127.0.0.1", 8090))?
    .run() //Start the httpserver
    .await //used to asynchronously wait for the HTTP server to finish running. 
}
