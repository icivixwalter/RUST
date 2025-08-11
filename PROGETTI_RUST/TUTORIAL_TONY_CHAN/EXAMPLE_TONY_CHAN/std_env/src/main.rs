/* IMPORTO LA LIBRERIA std::env
        -  std::env:  permette  di  interagire  con  l'ambiente  del  sistema,
        come  le  variabili  d'ambiente e gli argomenti della riga di comando:
    @std@env.06?_(libreria per la gestine dell'ambiente di sistema)

Come visto, use std::env importa il modulo env, e poi utilizziamo env::args() per
raccogliere gli argomenti della riga di comando in un vettore.

È possibile importare tutto il contenuto di un modulo usando *, oppure importare
specifici elementi usando la virgola per separarli:  come ad esempio
        use std::collections::*;



*/


// Importa il modulo std::env per accedere alle variabili d'ambiente,
// inclusi gli argomenti passati al programma da linea di comando.
use std::env;

//Questo importa tutti i tipi e funzioni dal modulo std::collections.
// importo elementi specifici usando la virgola per separarli:
use std::cmp::{min, max};

fn main() {
    println!("PROGETTO PER LA GESTIONE DELL'AMBIENTE");
    // Recupera gli argomenti passati al programma (incluso il nome del binario)
    // env::args() restituisce un iteratore di String.
    // collect() lo trasforma in un vettore (Vec<String>).
    let args: Vec<String> = env::args().collect();

    // Stampa un'intestazione
    println!("UTILIZZO DEL VETTORE DI STRINGHE ");

    // Itera su ciascun argomento e lo stampa a video
    for _arg in &args {

    println!("Argomenti passati: {:?}", args);
    }


//Qui, importiamo solo le funzioni min e max dal modulo std::cmp.

 let a = 10;
    let b = 20;

    println!("Minimo: {}", min(a, b));
    println!("Massimo: {}", max(a, b));

}
