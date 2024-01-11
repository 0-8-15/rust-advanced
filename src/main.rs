// Importieren notwendiger Module und Typen aus der actix_web-Bibliothek.
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// This struct represents state
struct AppState {
    app_name: String,
}

// Definiert eine asynchrone Handler-Funktion `hello` für GET-Anfragen an die Wurzelroute ("/").
#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    // Sendet eine HTTP-Antwort mit dem Statuscode 200 (OK) und dem Textkörper "Hello world!".
    //HttpResponse::Ok().body("Hello world!")
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
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
    // Erstellt und startet einen neuen HTTP-Server.
    HttpServer::new(|| {
        // Erstellt eine neue Actix-App.
        App::new()
	    .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            // Fügt den `hello`-Handler als Service für GET-Anfragen hinzu.
            .service(hello)
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
