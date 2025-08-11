//per utilizzare il rand occorre extern visto che è stato dichiarato nel toml
extern crate rand;

use std::cmp::Ordering; //nuovo inserimento per il confronto numeri con un match

use rand::Rng;
use std::io; //rng è nell'ambito del programma e Rng è la libreria del crate rand ed
             //ha delle funzioni + metodi, detti traist che devono essere attivati
             //traits = cose chiamate tratti el metodo funzione se attiviamo il traits

fn main() {
    println!("\n ================== PROGRAMMA INDOVINA IL NUMERO \n");

    println!("Indovina il numero!");

    println!("Prego, digita un _tentativo.");
    let mut _tentativo = String::new();

    let numero_segreto = rand::thread_rng().gen_range(1, 101);
    /* della libreria rand utilizzo la funzione thread_rng()
    + il metodo .gen_range(1, 101);
       questo metodo è disponibile perchè abbiamo usato
       rand::Rng,

    */
    println!("Il numero segreto è: {}", numero_segreto);

    loop {
        println!("Prego, digita un numero.");

        //imposto la variabile con stringa + lego lo stream + conversione in u32
        //---------------------------------------------------------------//
        let mut _tentativo = String::new();
        
        let pasta=2;
        
        io::stdin()
            .read_line(&mut _tentativo)
            .expect("Non si riesce a leggere la riga");

        //_tentativo utilizzo del metodo parse modifcato per evitare panic
        //let _tentativo: u32 = _tentativo.trim().parse().expect("Prego, digita un numero!");
        //let _tentativo: u32 = _tentativo.trim().parse().expect("Prego, digita un numero!");

        let _tentativo = match _tentativo.trim().parse::<u32>() {
            Ok(par_num) => par_num,
            //in caso di errore _ ignora il valore dell'errore ed esegue continue cioè va avanti nel ciclo
            Err(_) => continue,
        };

        println!("Hai digitato: {}", _tentativo);
        //---------------------------------------------------------------//

        //fa il confronto con im metodto std::cmp ....

        //---------------------------------------------------------------//
        match _tentativo.cmp(&numero_segreto) {
            Ordering::Less => println!("Troppo piccolo!"),
            Ordering::Greater => println!("Troppo grande!"),
            Ordering::Equal => {
                println!("Hai vinto!");
                break;
            }
        }
        //---------------------------------------------------------------//
    }
}
