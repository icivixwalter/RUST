use std::io::stdin;


fn main() {
    //chiamo hello
    hello();
    
	
	// PERMETTE DI RIMANERE VISIBILE LA SHELL FINCHE NON BATTO UN TASTO
    //---------------------------------------------------------------------------//
    // utilizzare questa libreria: use std::io::stdin;
	let mut s = String::new();
    println!("premi invio per uscire!");
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    //---------------------------------------------------------------------------//
}

pub fn hello() {
    println!("Hello, world! \n lezione 1.2. Ciao, Mondo!");

}



          