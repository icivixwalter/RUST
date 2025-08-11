/* IMPORTO LA LIBRERIA std::io ma utilizzo 
        std:in per leggere una riga di testo dall'imput utente dalla tastiera.
    @std@in_(esempio per leggere la riga di testo da tastiera)
*/


use std::io::{self, Write};

fn main() {

    println!("PROGETTO DI GESTIONE DELLA LIBRERIA STD::In!");

    let mut input = String::new();   //In Rust, print!() non stampa subito, ma bufferizza l'output.
    print!("Inserisci il tuo nome: ");
    // Se vuoi essere sicuro che un prompt venga mostrato prima
    //che l’utente digiti qualcosa (es. "Inserisci il tuo nome: "),
    //devi forzare lo svuotamento del buffer con .flush().
    //in questo modo viene subito visualizzato print! .....
    io::stdout().flush().unwrap();  // Forza la stampa immediata .flush che svuota il buffer

    /*
        io::stdin() accede all’input standard (la tastiera).
        .read_line(&mut input) legge una riga inserita dall’utente
        e la memorizza nella variabile input (che deve essere una String mutabile).
        .expect("Errore nella lettura") forza l’interruzione del programma con il messaggio indicato se si verifica un errore nella lettura.
        qui rimane in attesa dell'imput da tastiera
    */
    io::stdin().read_line(&mut input).expect("Errore nella lettura");
    println!("Ciao, {}!", input.trim());
}
