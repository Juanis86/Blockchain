// Importa las bibliotecas y dependencias necesarias
use warp::{Filter, Rejection, Reply};
use serde::Serialize;
use warp::http::StatusCode;
use std::collections::HashMap;
use diesel::prelude::*;
use diesel::SqliteConnection;
mod db;

fn establish_connection() -> SqliteConnection {
    let database_url = "your_database_url_here"; // Ejemplo: "sqlite://my_database.db"
    SqliteConnection::establish(&database_url).expect("Error connecting to the database")
}

// Define la estructura de los contratos inteligentes
#[derive(Debug, Serialize)]
struct Contract {
    id: u64,
    name: String,
    category: String,
    tags: Vec<String>,
    // Otros campos relevantes
}

// Simula una base de datos de contratos inteligentes
fn get_dummy_data() -> HashMap<u64, Contract> {
    let mut contracts = HashMap::new();
    contracts.insert(1, Contract {
        id: 1,
        name: "Contrato A".to_string(),
        category: "Categoría A".to_string(),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    });
    contracts.insert(2, Contract {
        id: 2,
        name: "Contrato B".to_string(),
        category: "Categoría B".to_string(),
        tags: vec!["tag3".to_string(), "tag4".to_string()],
    });
    // Agregar más contratos según sea necesario
    contracts
}

// Función para buscar y filtrar contratos inteligentes
fn search_contracts(query: String) -> Vec<Contract> {
    let database_url = "your_database_url_here"; // Ejemplo: "sqlite://my_database.db"
    let pool = db::create_db_pool(database_url);
    let contracts = get_dummy_data();
    let mut results = Vec::new();

    for (_, contract) in contracts.iter() {
        if contract.name.contains(&query) || contract.category.contains(&query) {
            results.push(contract.clone());
        } else {
            for tag in &contract.tags {
                if tag.contains(&query) {
                    results.push(contract.clone());
                    break;
                }
            }
        }
    }

    results
}

// Función para obtener listados de contratos inteligentes
fn get_contract_listings() -> Vec<Contract> {
    let contracts = get_dummy_data();
    contracts.values().cloned().collect()
}

// Configuración del servidor
#[tokio::main]
async fn main() {
    let search_route = warp::path!("search" / String)
        .and_then(|query: String| async move {
            let results = search_contracts(query);
            let json_response = warp::reply::json(&results);
            Ok(json_response)
        });

    let listings_route = warp::path!("listings")
        .and_then(|| async move {
            let listings = get_contract_listings();
            let json_response = warp::reply::json(&listings);
            Ok(json_response)
        });

    // Configura las rutas y ejecuta el servidor
    warp::serve(search_route.or(listings_route))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
