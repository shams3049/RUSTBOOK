// Import necessary modules and crates
use actix_files::Files; // For serving static files
use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware}; // Actix Web framework components
use actix_cors::Cors; // For handling Cross-Origin Resource Sharing (CORS)
use serde::Serialize; // For serializing data structures into JSON
use std::io::Error; // For handling I/O errors
use log::info; // For logging information

// Define a struct to represent the data being sent as JSON
#[derive(Serialize)] // Automatically derive serialization for this struct
struct MyData {
    message: String, // A single field containing a message
}

// Asynchronous handler function for the "/api/data" route
async fn get_data() -> impl Responder {
    info!("get_data function called"); // Log that the function was called
    let data = MyData {
        message: "This is some data".to_string(), // Create a MyData instance with a message
    };
    HttpResponse::Ok() // Return an HTTP 200 OK response
        .content_type("application/json") // Set the content type to JSON
        .json(data) // Serialize the data into JSON and include it in the response
}

// Main function to start the Actix Web server
#[actix_web::main] // Macro to mark the async main function for Actix Web
async fn main() -> Result<(), Error> {
    env_logger::init(); // Initialize the logger
    std::env::set_var("RUST_LOG", "info"); // Set the logging level to "info"

    // Create and configure the HTTP server
    HttpServer::new(|| {
        // Configure CORS to allow any origin, method, and header
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        // Build the Actix Web application
        App::new()
            .wrap(middleware::Logger::default()) // Add logging middleware
            .wrap(cors) // Add CORS middleware
            .route("/api/data", web::get().to(get_data)) // Define the "/api/data" route
            .service(Files::new("/", "./static").index_file("index.html")) // Serve static files from the "./static" directory
    })
    .bind("0.0.0.0:8080") // Bind to all interfaces to work in Docker
    .map_err(|e| {
        eprintln!("Failed to bind to address: {}", e); // Print an error if binding fails
        e
    })?
    .run() // Run the server
    .await // Wait for the server to finish running
    .map_err(|e| {
        eprintln!("Server error: {}", e); // Print an error if the server fails
        e
    })
}
