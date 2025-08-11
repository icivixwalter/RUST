/*
    APRO FILE ACCESS @GESTIONE@IN@MIGLIORAMENTO

    deve prima creare una variabili di sistema :
        MSACCESS_PATH
    con questa path
        C:\Program Files (x86)\Microsoft Office\Office\MSACCESS.EXE
    Devi riportare il collegamento per interno dell'eseguibile di Access che è
    access 2000 ma in ufficio è access 2007 per cui occorre cambiarlo e ricreare 
    l'eseguibile exe

*/

use std::process::Command;

//APRO GESTIONE IN MIGLIORAMENTO
fn main() {
    let path = "c:\\CASA\\LTT\\SORG\\GESTIONI_SORG_(IN_MIGLIORAMENTO)\\MDB\\GESTIONI_SORG_(IN_MIGLIORAMENTO).mdb";

      // Specifica il programma associato ai file .mdb devi indicare la path completa
    let mdb_program = "C:\\Program Files (x86)\\Microsoft Office\\Office\\MSACCESS.EXE"; // Cambia questo valore in base al programma che utilizzi

    //crea una nuova istanza di access.exe e gli passa come argomento la path + stamapa
    let output = Command::new(mdb_program)
        .args(&[path])
        .output();

    match output {
        Ok(output) => { //se la stampa è con successo apri il file mdb
            if output.status.success() {
                println!("File .mdb aperto con successo.");
            } else {
                //se ci sono problemi all'apertura da questo errore
                eprintln!("Errore nell'apertura del file .mdb. Errore:\n{:?}", output);
            }
        }
        Err(e) => {
            //se il problema è il processo non apre nulla e da errore in compilazione
            eprintln!("Errore durante l'esecuzione del processo figlio: {}", e);
        }
    }
}

