/* PROGETTO DI GESTIONE DELLA @LIBRERIA@ST::IO

   @std@io_(esempio di gestione dell'imput da tastiera e l'outuput a video)


*/


use std::io;



fn main() {
    println!("PROGETTO DI GESTIONE DELLA LIBRERIA STD::IO!");

 let mut input = String::new();
    println!("Inserisci qualcosa:");

      // Legge una riga dall'input standard e la memorizza in `input`
      // &mut = serve per passare un riferimento mutabile  alla funzione read_line
      // read_line non vuole possedere la proprieta ma MODIFICARLA
    io::stdin().read_line(&mut input).expect("Errore nella lettura dell'input");

     println!("SENZA &mut read_line non puo modificare l'imput - Hai inserito: {}", input);

}
