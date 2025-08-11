/* PROGETTO DI GESTIONE DEL TEMPO

Qui,  utilizziamo  use  std::thread  per  importare  il  modulo  thread,
creandone  uno nuovo  che  esegue  un  loop,  e  poi  facciamo
"dormire"  il  thread  principale  per  un  breve periodo.
   std@time.05?_(esempio di gestione del tempo)

Qui,  Instant::now()  cattura  il  tempo  corrente,  e  elapsed()  calcola  quanto  ne  è
passato, permettendo di misurare la durata di un'operazione.



*/


// Importa due tipi dal modulo std::time:
// - Duration: rappresenta un intervallo di tempo
// - Instant: rappresenta un punto temporale preciso
use std::time::{Duration, Instant};

fn main() {

    println!("PROGETTO DI GESTIONE DEL TEMPO!");
    //@std@time.05?
    // Cattura il tempo corrente all'inizio dell'operazione
    let inizio = Instant::now();

    // Simula un'operazione che impiega tempo (es. un processo lento)
    // In questo caso, il thread "dorme" per 2 secondi
    std::thread::sleep(Duration::from_secs(2));

    // Calcola quanto tempo è passato dall'istante iniziale
    let durata = inizio.elapsed();

    // Stampa il tempo trascorso, formattato con 2 decimali
    println!("Operazione completata in: {:.2?} secondi", durata);
}

