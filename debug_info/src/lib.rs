// In der separaten Crate für das Makro:

/*
proc_macro::TokenStream: Dies ist der Typ, der für die Ein- und Ausgabe des Makros verwendet wird. Er repräsentiert einen Stream von Token, die der Rust-Compiler verarbeiten kann.
quote: Ein Hilfswerkzeug aus dem quote-Crate, das verwendet wird, um Rust-Code als Token-Stream zu generieren.
syn: Dieses Crate wird verwendet, um Rust-Code zu parsen, der als Eingabe für das Makro bereitgestellt wird.
*/
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/*
    #[proc_macro]: Eine Makro-Definition. Dieses Makro wird auf Rust-Elemente (wie Strukturen) angewendet und kann deren Verhalten zur Kompilierungszeit ändern.
    debug_info: Der Name des Makros.
    item: Der Code des Elements (z.B. einer Struktur), auf das das Makro angewendet wird.
*/
#[proc_macro]
pub fn debug_info(item: TokenStream) -> TokenStream {
    /*
       Hier wird der eingehende Code (item), der die Struktur darstellt, geparst,
       um eine syn::ItemStruct-Instanz zu erzeugen.
       Dies ermöglicht die Bearbeitung der Strukturdefinition im Rust-Code.
    */
    let input = syn::parse_macro_input!(item as syn::ItemStruct);

    /*
       struct_name: Speichert den Namen der Struktur.
       field_names: Erzeugt eine Liste der Feldnamen der Struktur.
       Diese Zeile verwendet Iteratoren, um über jedes Feld (repräsentiert durch f)
       in der Struktur zu iterieren und dessen Namen zu extrahieren.
    */
    let struct_name = &input.ident;
    let field_names = input.fields.iter().map(|f| &f.ident);

    /*
       Hier beginnt die Verwendung von quote!,
       um den generierten Code als Token-Stream zu erstellen.
       Dieser Code wird dann in das aufrufende Rust-Programm eingefügt.
    */
    let gen = quote! {
        // Der ursprüngliche Strukturcode
        #input

        /*
            Hier wird eine Implementierung (impl) für die gegebene Struktur erzeugt,
            die eine öffentliche Methode debug_info enthält.

            Die debug_info-Methode gibt den Namen der Struktur und die Namen aller ihrer Felder aus.
            stringify!(#struct_name): Konvertiert den Namen der Struktur in einen String zur Laufzeit.
            Das innere Makro innerhalb von quote! (gekennzeichnet durch #(...) *)
            wird für jedes Feld der Struktur wiederholt und gibt den Namen jedes Feldes aus.
        */
        impl #struct_name {
            pub fn debug_info(&self) {
                println!("Debug-Informationen für {}", stringify!(#struct_name));
                println!("  Felder:");
                #(
                    println!("    {}", stringify!(#field_names));
                )*
            }
        }
    };
    /*
       Der generierte Token-Stream (gen) wird in den Typ TokenStream konvertiert und zurückgegeben,
       sodass der Compiler diesen in ausführbaren Rust-Code umwandeln kann.
    */
    gen.into()
}

// log entering an exiting of a function. take string as parameter and prints it on exit

#[proc_macro_attribute]
// Definiert eine öffentliche Funktion `log` für ein prozedurales Makro.
pub fn log(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // `item` repräsentiert den TokenStream des Rust-Items (z.B. eine Funktion), auf das das Makro angewendet wird.
    // Hier wird es geparst als eine Funktionsdefinition (`ItemFn`).
    let input = parse_macro_input!(item as ItemFn);

    // `attr` sollte `attrs` sein. Es ist ein Fehler im Code. Es geparst einen Literal-String (LitStr) aus den Makro-Attributen.
    // Dies ist der Teil, der überarbeitet werden muss, da `attr` nicht definiert ist.
    let arg = parse_macro_input!(attrs as LitStr);

    // Extrahiert den String-Wert aus dem Literal-String.
    let message = arg.value();

    // Sammelt die Eingabeparameter der Funktion.
    let inputs = &input.sig.inputs;

    // Extrahiert den Rückgabetyp der Funktion.
    let output = &input.sig.output;

    // Nimmt den Namen der Funktion.
    let function_name = &input.sig.ident;

    // Nimmt den Codeblock (Körper) der Funktion.
    let block = &input.block;

    // Sammelt die Attribute (z.B. `#[inline]`, `#[cfg(..)]`), die auf die Funktion angewendet werden.
    let attrs = &input.attrs;

    // Extrahiert die Sichtbarkeit (z.B. `pub`) der Funktion.
    let vis = &input.vis;

    // Erstellt den neuen Funktionscode. `quote!` wird verwendet, um den Rust-Code als TokenStream zu generieren.
    // Dieser neue Code fügt Log-Nachrichten vor und nach der Ausführung der ursprünglichen Funktion ein.
    let gen = quote! {
        #(#attrs)* #vis fn #function_name(#inputs) #output {
            println!("Logger says: {} before execution", #message);
            let result = (|| #block)();
            println!("Logger says: {} after execution", #message);
            result
        }
    };

    // Wandelt den generierten TokenStream in den erforderlichen Rückgabetyp um.
    gen.into()
}

#[proc_macro_attribute]
pub fn measure_time(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let function_name = &input.sig.ident;
    let block = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;
    let gen = quote! {
        #(#attrs)* #vis fn #function_name(#inputs) #output {
            let start = std::time::Instant::now();
            let result = (|| #block)();
            let duration = start.elapsed();
            println!("Time elapsed: {}ms", duration.as_millis());
            result
        }
    };
    gen.into()
}
