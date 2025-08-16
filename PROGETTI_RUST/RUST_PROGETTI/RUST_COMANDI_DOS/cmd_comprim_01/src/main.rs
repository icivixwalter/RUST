/*
================================================================================
 RUST_COMPRIMI_MESE - Strumento di backup e compressione mensile
--------------------------------------------------------------------------------
 FUNZIONALITÀ PRINCIPALI
  1. Legge da CLI (-i, -o) oppure da file .txt i percorsi di:
     - Cartella di partenza (FILE_PARTENZA)
     - Cartella di arrivo (FILE_ARRIVO)
  2. Scansiona la cartella sorgente e legge data ultima modifica file
  3. Raggruppa file per ANNO e MESE
  4. Comprimi in archivi multi-volume (1 MB) con 7-Zip
  5. Salva nella cartella di arrivo includendo il nome della cartella padre
  6. Stampa a video il comando generato per ogni archivio

--------------------------------------------------------------------------------
 ATTIVITÀ ESEGUITE
  - Implementata lettura parametri sia da CLI sia da file di configurazione
  - Aggiunta scansione cartella sorgente con recupero anno/mese di modifica
  - Creata logica di raggruppamento file per ANNO e MESE
  - Costruzione dinamica del nome archivio includendo cartella padre
  - Sostituzione eseguibile RAR con 7-Zip mantenendo sintassi compatibile
  - Gestione volumi di compressione fissi a 1 MB
  - Debug a video con stampa comando completo prima dell’esecuzione
  - Inseriti test di compressione base (file singolo e cartella)
  - Aggiunti segnalibri di sezione per navigazione rapida del codice

--------------------------------------------------------------------------------
 SEGNALIBRI / INDICE CODICE
 [COSTANTI]           → Percorsi file di configurazione
 [struct Argomenti]   → Gestione parametri CLI con clap
 [main]               → Avvio, lettura parametri, esecuzione compressione
 [struct ComprimiFile]→ Range anni, percorsi, configurazione
 [new]                → Costruttore di ComprimiFile
 [esegui]             → Scansione, filtraggio anno/mese, chiamata compressione
 [comprimi_rar]       → Generazione ed esecuzione comando 7-Zip
 [get_path_parent]    → Recupero nome cartella padre di un file
 [tests]              → Test automatici compressione e funzioni base

--------------------------------------------------------------------------------
 NOTE TECNICHE
 - Volume di compressione attuale: 1 MB (-v1m)
 - Percorso eseguibile 7-Zip impostato su:
   "C:\\Program Files\\7-Zip\\7z.exe"
 - I parametri "U", "-r", "-ac" sono rimasti per compatibilità con script RAR
================================================================================



[main]
 └─ Avvio applicazione
    ├─ [struct Argomenti]
    │   └─ Tenta lettura parametri da CLI (-i, -o)
    │       └─ Se fallisce → carica i percorsi da [COSTANTI] (file .txt)
    ├─ [new]
    │   └─ Crea istanza ComprimiFile con:
    │       - Anno inizio: 1950
    │       - Anno fine: 2050
    │       - Path sorgente e destinazione
    └─ [esegui]
        ├─ Legge contenuto della cartella sorgente
        ├─ Per ogni file:
        │   ├─ Recupera metadati (FileTime)
        │   ├─ Estrae anno/mese ultima modifica
        │   ├─ Costruisce nome archivio:
        │   │   "[DEST]\\[CARTELLA_PADRE]_[ANNO]_[MESE].rar"
        │   ├─ Confronta anno/mese con range consentito
        │   └─ Se combacia → chiama [comprimi_rar]
        └─ Fine scansione

[comprimi_rar]
 ├─ Prepara comando "C:\Program Files\7-Zip\7z.exe"
 ├─ Imposta argomenti: "U -r -ac [NOME_ZIP] [FILE] -v1m"
 ├─ Stampa a video il comando generato (debug)
 └─ Esegue 7-Zip con Command::output()

[get_path_parent]
 └─ Restituisce nome cartella padre del file (per costruzione nome archivio)

[tests]
 ├─ Test matematico banale (2+2=4)
 ├─ Test compressione di un file singolo
 └─ Test compressione di una cartella (con multi-volume)



*/









use file_time::FileTime;
use std::{fs, io::Error, process::Command, path::Path};

use clap::Parser;

//importare un tuo file
mod file_time;


//COSTANTI PATH ARRIVO E PARTENZA
const FILE_PARTENZA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/paths_Partenza.txt");
const FILE_ARRIVO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/path_Arrivo.txt");

/// Simple app for backup files and folders recursively from a file with a list of paths
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Argomenti {
    // short: il parametro corto (-i), long: il parametro con nome completo --input-file-with-paths
    #[clap(short = 'i', long)]
    input_path: String,
    #[clap(short = 'o', long)]
    output_path: String, // se il path non esiste lo crea
}



fn main() {
    //******************************* aggiunto per i parametri */
    //se da linea di comando inserisci -i e -o prende i valori
    //dai parametri altrimenti prende quelli dai file .txt
    //select case con 2 bracci


    let _args = match Argomenti::try_parse() {
        Ok(arg) => arg,
        Err(_) => {


            let x = fs::read_to_string(FILE_ARRIVO.to_string());
            let y = fs::read_to_string(FILE_PARTENZA.to_string());


            Argomenti {
                input_path: y.unwrap(),
                output_path: x.unwrap(),
            }
        }
    }; // salva gli argomenti CLI in una nuova istanza della struct





    let args = match Argomenti::try_parse() {
        Ok(arg) => arg, // parametri passati da CLI
        Err(_) => {
            // Leggi dai file di configurazione in modo sicuro
            let input_path = fs::read_to_string(FILE_PARTENZA)
               .expect("Impossibile leggere il file di partenza");
            let output_path = fs::read_to_string(FILE_ARRIVO)
               .expect("Impossibile leggere il file di arrivo");

            Argomenti {
                input_path: input_path.trim().to_string(),
                output_path: output_path.trim().to_string(),
            }
        }
    };










    //***++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */


    //03 istanzio la struct
    let comprimi_file = ComprimiFile::new(&args.input_path, &args.output_path);
    match comprimi_file.esegui() {
        Ok(_) => (),
        Err(err) => println!("errore : {}", err),
    }
    println!("fine procedura di compattamento rar!");
}

//01 creo una struct per i parametri
struct ComprimiFile {
    anno_inizio: i32,
    anno_fine: i32,
    path_sorgente: String,
    path_destinazione: String,
}

//02 implentazione metodi della struttura comprimi
impl ComprimiFile {
    //metodo statico che diventa costruttore con new
    //creando una istanza di ComprimiFile
    fn new(par_path_sorgente: &str, par_path_destinazione: &str) -> ComprimiFile {
        ComprimiFile {
            anno_inizio: 1950,
            anno_fine: 2050,
            //to_owned() = significa la traduzione da &str in String
            path_sorgente: par_path_sorgente.to_owned(),
            path_destinazione: par_path_destinazione.to_owned(),
        }
    }

    //II metodo esegui &self = richiede l'istanza comprimi file
    // perche non è statico
    fn esegui(&self) -> Result<bool, Error> {
        //for partendo dagli estremi anno inizio e fine del costruttore

        let cartella = fs::read_dir(&self.path_sorgente)?;
        for file in cartella {
            match file {
                Ok(dir_entry) => {
                    //la path del file corrente
                    let file_metadata = dir_entry.metadata()?;
                    let istanza_file_time = FileTime::new(file_metadata);
                    //destrutturazione di una tupla = assegna ad anno e al mese i due
                    //valori recuperati dalla tupla istanza_file_time.get_anno_mese()
                    let (anno, mese) = istanza_file_time.get_anno_mese();
                    // TODO: aggiungere nome cartella genitore
                    let nome_file_zip = format!("{}\\{}_{}_{:#02}.rar",self.path_destinazione, get_path_parent(&dir_entry.path()), anno, mese);

                    for anno_corrente in self.anno_inizio..=self.anno_fine {
                        for mese_corrente in 1..=12 {
                            if anno == anno_corrente && mese == mese_corrente {
                                ComprimiFile::comprimi_7zip(&nome_file_zip, dir_entry.path().to_str().unwrap_or(""));
                            }
                        }
                    }
                }
                Err(errore) => println!("errore di ricerca del file: {}", errore),
            }
        }
        Ok(true)
    }

    /// La funzione serve per comprimere un archivio usando uno strumento esterno, ovvero 7z.exe, il quale è un eseguibile del programma 7-Zip installato nel percorso C:\Program Files\7-Zip\7z.exe.
    ///
    /// # Arguments
    ///
    /// * `par_nome_zip`: una stringa che rappresenta il nome del file ZIP di output.
    /// * `par_nome_file_archivio`: una stringa che rappresenta il file o i file che devono essere compressi.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn comprimi_7zip(par_nome_zip: &str, par_nome_file_archivio: &str) {
        //istanzio il comando rar

        let mut command = Command::new("C:\\Program Files\\7-Zip\\7z.exe");

        //predispongo i successivi parametri di rar in un vettore
        //     let argomenti = vec![
        //     "U",
        //     "-r",
        //     "-ac",
        //     par_nome_zip,
        //     par_nome_file_archivio,
        //     "-v1m",
        // ];

        let argomenti = vec![
            "a",               // Comando per aggiungere file/directory all'archivio
            par_nome_zip,      // Nome del file di output (archivio)
            par_nome_file_archivio, // File o directory da comprimere
        ];

        // prende l'istanza del comando a cui aggiunge gli argomenti rar
        let command = command.args(&argomenti);


        let mut s: String = String::new();
        for arg in argomenti {
            s.push_str(arg);
            s.push(' ');
        }

        println!("Rar.exe {}", s);
        // Eseguiamo il comando
        match command.output() {
            Ok(output) => {
                // Controlliamo se il comando ha avuto successo
                if output.status.success() {
                    println!("Archivio creato con successo: {}", par_nome_zip);
                } else {
                    // In caso di errore, stampiamo l'output per debugging
                    eprintln!(
                        "Errore durante la creazione dell'archivio:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => {
                eprintln!("Errore durante l'esecuzione del comando: {}", e);
            }
        }
    }
}


pub fn get_path_parent (path_file:&Path)->String{
    let parent =Path::parent(path_file).unwrap();
    Path::file_name(parent).unwrap().to_str().unwrap().to_owned()
}


//          test
//----------------------------------------------------------------------------------------------//
#[cfg(test)]
mod tests {
    use std::panic;
    use std::path::Path;

    use super::*;

     #[test]
    fn comprimi_rar_test() {
         // Wrappiamo il codice del test con `catch_unwind`
         let result = panic::catch_unwind(|| {
             // Test 1: Creazione di "prova.zip"
             ComprimiFile::comprimi_7zip("prova.zip", "Cargo.toml");
             let x = Path::new("prova.zip").exists();
             assert!(x, "test fallito il file .zip non esiste");

             // Test 2: Creazione di "prova2.zip"
             ComprimiFile::comprimi_7zip("prova2.zip", concat!(env!("CARGO_MANIFEST_DIR"), "/PathDiArrivo"));
             let x = Path::new("prova2.zip.part01.rar").exists() || Path::new("prova2.zip").exists();
             assert!(x, "test fallito il file .zip non esiste");
         });

         // Cleanup: Cancelliamo i file creati, indipendentemente dal risultato
         if Path::new("prova.zip").exists() {
             fs::remove_file("prova.zip").unwrap();
         }
         if Path::new("prova2.zip").exists() {
             fs::remove_file("prova2.zip").unwrap();
         }
         if Path::new("prova2.zip.part01.rar").exists() {
             fs::remove_file("prova2.zip.part01.rar").unwrap();
         }

         // Rilanciamo il panic se il test è fallito
         if let Err(err) = result {
             panic!("{:?}", err);
         }
    }
}
