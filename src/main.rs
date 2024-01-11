// Importieren notwendiger Module und Typen aus der actix_web-Bibliothek.
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::guard;
use std::sync::Mutex;

// This struct represents state
struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

// Definiert eine asynchrone Handler-Funktion `hello` für GET-Anfragen an die Wurzelroute ("/").
#[get("/hihi")]
async fn hello(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

// Definiert eine asynchrone Handler-Funktion `echo` für POST-Anfragen an die Route "/echo".
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    // Sendet eine HTTP-Antwort mit dem Statuscode 200 (OK) und gibt den Anfrage-Textkörper zurück.
    HttpResponse::Ok().body(req_body)
}

// Definiert eine weitere asynchrone Handler-Funktion `manual_hello`.
async fn manual_hello() -> impl Responder {
    // Sendet eine HTTP-Antwort mit dem Statuscode 200 (OK) und dem Textkörper "Hey there!".
    HttpResponse::Ok().body("Hey there!")
}

// Markiert die Hauptfunktion als asynchronen Einstiegspunkt für Actix Web.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    // Erstellt und startet einen neuen HTTP-Server.
    // !!! Use paperclip instead.
    HttpServer::new(move || {
        // Erstellt eine neue Actix-App.
        App::new()
            .app_data(counter.clone()) // <- register the created data
            // Fügt den `hello`-Handler als Service für GET-Anfragen hinzu.
	    .service(
                web::scope("/")
                    .guard(guard::Host("127.0.0.1"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )            .service(hello)
            // Fügt den `echo`-Handler als Service für POST-Anfragen hinzu.
            .service(echo)
            // Definiert eine Route "/hey" und setzt `manual_hello` als Handler für GET-Anfragen.
            .route("/hey", web::get().to(manual_hello))
    })
    // Bindet den Server an die Adresse "127.0.0.1" auf Port 8080.
    .bind(("127.0.0.1", 8080))?
    // Startet den Server und wartet auf eingehende Verbindungen.
    .run()
    // Wartet auf das Ende des Servers.
    .await
}
