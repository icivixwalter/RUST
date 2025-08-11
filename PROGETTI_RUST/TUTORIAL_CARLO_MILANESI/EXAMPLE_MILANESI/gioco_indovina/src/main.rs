use ::std::io; //libreria di imput utente e per la stampa di output

//NELLA MAIN () = non ci sono argomenti  e con println! = utilizzo delle macro per la stampa della stringa sullo schermo
fn main() {
    println!("======================== PROGETTO INDOVINA ======================");

    println!("Indovina il numero!");

    println!("Prego, digita un tentativo.");

    let mut tentativo = String::new();

    io::stdin()
        .read_line(&mut tentativo)
        .expect("Non si riesce a leggere la riga");

    println!("Hai digitato: {}", tentativo);
}
