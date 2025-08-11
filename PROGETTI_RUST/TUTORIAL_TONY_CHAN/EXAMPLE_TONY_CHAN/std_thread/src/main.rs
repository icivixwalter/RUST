/* PROGETTO DI GESTIONE DEI THREAD

Qui,  utilizziamo  use  std::thread  per  importare  il  modulo  thread,
creandone  uno nuovo  che  esegue  un  loop,  e  poi  facciamo
"dormire"  il  thread  principale  per  un  breve periodo.
   @std@thread_(esempio di gestione della concorrenza con i thread)



Utilizziamo  Mutex  per  garantire  l'accesso  sicuro  a  un  contatore  condiviso  tra  più
thread. Mentre con Arc gestiamo il contatore in maniera sicura e condivisa.

spiegazioni:

@spawn.01?
==============================================================================================================
    Cos’è thread::spawn?
    spawn è una funzione del modulo std::thread che crea un nuovo thread
    e lo avvia immediatamente.
    Sintassi generale:
            let handle = std::thread::spawn(|| {
                // codice eseguito nel nuovo thread
            });

    La funzione prende una closure (una funzione anonima) che
        contiene il codice da eseguire nel nuovo thread.

    Restituisce un handle (JoinHandle), che permette
        al thread che l’ha creato di aspettare la fine
        del thread secondario (con .join()).

    Perché si chiama spawn?


@unwrap().03?  Il termine spawn in informatica significa "generare", "creare", "generare un nuovo processo o thread".
            In Rust (e in molti altri linguaggi e sistemi), spawn indica la creazione e
    Perché si chiama spawn?

    Perché si chiama spawn?
            avvio di un thread in modo asincrono
            (cioè il nuovo thread lavora in parallelo a quello principale).

            Non è solo creare un thread, ma anche avviarlo subito,
            quindi "farlo nascere" ed eseguire la closure passata.

    Cosa succede se non uso spawn?
            Se non usassi spawn, tutto il codice sarebbe eseguito nel thread principale in sequenza.
            spawn serve proprio a creare concorrenza: più thread che lavorano contemporaneamente.

    In sintesi:
        thread::spawn crea e avvia un thread separato.
        Il codice dentro la closure è eseguito contemporaneamente al thread principale.
        Ti permette di fare operazioni parallele e concorrenti.



@sleep.02?
==============================================================================================================
    Spiegazione dettagliata
            thread::sleep(...) → sospende l'esecuzione del thread attivo per un certo tempo.
            Duration::from_millis(50) → crea una durata temporale di 50 millisecondi.

==============================================================================================================


==============================================================================================================
*/


// Importa il modulo 'thread' per creare ed eseguire thread paralleli
use std::thread;

// Importa 'Duration' per gestire le pause temporali tra le operazioni
use std::time::Duration;

fn main() {

     println!("PROGETTO DI GESTIONE DELLA LIBRERIA STD::Thread!");

    // Avvia un nuovo thread secondario usando thread::spawn
    // Il thread eseguirà la closure (funzione anonima) definita tra le parentesi
    //@spawn.01?
    let handle = thread::spawn(|| {
        // Il thread secondario esegue un ciclo da 1 a 9
        for i in 1..10 {
            // Stampa un messaggio dal thread secondario
            println!("Ciao dal thread secondario! {}", i);
            // Fa una pausa di 100 millisecondi per ogni iterazione - @sleep.02?
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Il thread principale esegue un ciclo da 1 a 4
    for i in 1..5 {
        // Stampa un messaggio dal thread principale
        println!("Ciao dal thread principale! {}", i);
        // Fa una pausa più breve: 50 millisecondi per ogni iterazione
        thread::sleep(Duration::from_millis(50));
    }

    //@unwrap().03?
    // Aspetta che il thread secondario finisca la sua esecuzione
    // 'join' blocca l'esecuzione del thread principale finché il secondario non termina
    // 'unwrap' serve per gestire eventuali errori, e in caso di errore provoca un panic
    handle.join().unwrap();
}
