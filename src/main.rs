use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// #[tokio::main] ist ein Macro, welches die main-Funktion in einen Tokio-Task umwandelt.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hier wird der TCP-Listener erstellt und auf Port 8080 gebunden.
    let listener = match TcpListener::bind("127.0.0.1:8080").await {
        Ok(listener) => {
            println!("TCP-Listener erfolgreich erstellt.");
            listener
        }
        Err(e) => {
            println!("Fehler beim Erstellen des TCP-Listeners: {}", e);
            // into() konvertiert den Fehler in ein Box<dyn std::error::Error>
            // Box<dyn std::error::Error> ist ein Trait-Objekt, welches alle Typen akzeptiert, die das std::error::Error Trait implementieren.
            // dyn bedeutet dynamic, also dynamisch. Das bedeutet, dass der Typ zur Laufzeit bestimmt wird.
            // Trait-Objekte können nur über Referenzen verwendet werden, da sie sonst ihre Größe nicht kennen würden.
            return Err(e.into());
        }
    };

    // Hier wird der TCP-Listener auf eingehende Verbindungen gehorcht.
    loop {
        // _ ist ein Platzhalter für Variablen, die nicht verwendet werden.
        let (socket, _address) = match listener.accept().await {
            // Wenn eine Verbindung akzeptiert wurde, wird ein neuer Socket und die Adresse des Clients zurückgegeben.
            Ok((socket, address)) => {
                println!("Neue Verbindung von: {}", address);
                (socket, address)
            }
            // Wenn ein Fehler auftritt, wird dieser ausgegeben und die Schleife fortgesetzt.
            Err(e) => {
                println!("Fehler beim Akzeptieren der Verbindung: {}", e);
                continue;
            }
        };

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                println!("Fehler bei der Bearbeitung des Clients: {}", e);
            }
        });
    }
}

async fn handle_client(mut socket: TcpStream) -> Result<(), std::io::Error> {
    let mut name_buffer: [u8; 256] = [0; 256];
    // Erstelle einen Puffer mit einer Größe von 1024 Bytes
    // [0; 1024] ist ein Array mit 1024 Elementen, die alle den Wert 0 haben.
    let mut buffer: [u8; 1024] = [0; 1024];

    // Lies Dateinamen vom Client
    let name_bytes_read = match socket.read(&mut name_buffer).await {
        Ok(bytes_read) => {
            println!("{} Bytes vom Client empfangen.", bytes_read);
            bytes_read
        }
        Err(e) => {
            println!("Fehler beim Lesen vom Client: {}", e);
            return Err(e);
        }
    };
    let mut file = match File::create(format!(
        "files/{}",
        String::from_utf8_lossy(&name_buffer[..name_bytes_read]).trim()
    )).await{
        Ok(file) => {
            println!("Datei erfolgreich erstellt.");
            file
        }
        Err(e) => {
            println!("Fehler beim Erstellen der Datei: {}", e);
            return Err(e);
        }
    };

    // Lies Daten vom Client
    let bytes_read = match socket.read(&mut buffer).await {
        Ok(bytes_read) => {
            println!("{} Bytes vom Client empfangen.", bytes_read);
            bytes_read
        }
        Err(e) => {
            println!("Fehler beim Lesen vom Client: {}", e);
            return Err(e);
        }
    };

    // Konvertiere die empfangenen Daten in einen String und gebe diesen aus.
    // [..bytes_read] ist ein Slice, der nur die empfangenen Bytes enthält.
    let answer_bytes = format!(
        "{} Bytes vom Client empfangen.: {}",
        bytes_read,
        String::from_utf8_lossy(&buffer[..bytes_read])
    );
    print!("Antwort an Client: ");
    println!("{}", answer_bytes);

    // Nur zum Zwecke dieses Beispiels, schreibe die empfangenen Daten zurück zum Client
    if bytes_read > 0 {
        match socket.write_all(answer_bytes.as_bytes()).await {
            Ok(_) => println!("Antwort an Client gesendet."),
            Err(e) => {
                println!("Fehler beim Senden der Antwort an den Client: {}", e);
                return Err(e);
            }
        };
        file.write_all(&buffer[..bytes_read]).await?;
    }

    Ok(())
}
