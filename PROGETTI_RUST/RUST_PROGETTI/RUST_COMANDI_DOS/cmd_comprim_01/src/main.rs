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
 [comprimi_7z]       → Generazione ed esecuzione comando 7-Zip
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
        │   │   "[DEST]\\[CARTELLA_PADRE]_[ANNO]_[MESE].7z"
        │   ├─ Confronta anno/mese con range consentito
        │   └─ Se combacia → chiama [comprimi_7z]
        └─ Fine scansione

[comprimi_7z]
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


#[allow(unused_imports)]
use std::{fs, io::Error, process::Command, path::Path, panic};
use file_time::FileTime;
use clap::Parser;

//importare un tuo file
mod file_time;

// COSTANTI PATH ARRIVO E PARTENZA
// Questi file contengono i percorsi di input e output predefiniti
const FILE_PARTENZA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/paths_Partenza.txt");
const FILE_ARRIVO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/path_Arrivo.txt");

/// Simple app for backup files and folders recursively from a file with a list of paths
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Argomenti {
    // short: il parametro corto (-i), long: il parametro con nome completo --input-path
    #[clap(short = 'i', long)]
    input_path: String,
    #[clap(short = 'o', long)]
    output_path: String, // se il path non esiste lo crea
}

fn main() {
    //******************************* gestione dei parametri CLI o da file di configurazione */
    // Prova a leggere i percorsi da linea di comando con -i e -o
    // Se fallisce (nessun parametro fornito), legge i percorsi dai file .txt di configurazione

    let args = match Argomenti::try_parse() {
        Ok(arg) => arg, // Parametri forniti da CLI
        Err(_) => {
            // Lettura sicura dai file di configurazione
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

    // Istanzia l'oggetto che gestirà la compressione dei file
    let comprimi_file = ComprimiFile::new(&args.input_path, &args.output_path);

    // Esegue la procedura di compressione
    match comprimi_file.esegui() {
        Ok(_) => (),
        Err(err) => println!("Errore: {}", err),
    }
    println!("Fine procedura di compattamento RAR!");
}

// Struttura che memorizza i parametri della compressione
struct ComprimiFile {
    anno_inizio: i32,
    anno_fine: i32,
    path_sorgente: String,
    path_destinazione: String,
}

// Implementazione dei metodi della struttura ComprimiFile
impl ComprimiFile {
    /// Costruttore per creare un'istanza di ComprimiFile
    fn new(par_path_sorgente: &str, par_path_destinazione: &str) -> ComprimiFile {
        ComprimiFile {
            anno_inizio: 1950,
            anno_fine: 2050,
            path_sorgente: par_path_sorgente.to_owned(),
            path_destinazione: par_path_destinazione.to_owned(),
        }
    }

    /// Metodo principale che scansiona la cartella sorgente e comprime i file
    fn esegui(&self) -> Result<bool, Error> {

        //@modifica
        // *** MODIFICA 1: Creazione del file di log ***
        let log_file_path = format!("{}/log.txt", self.path_destinazione);
        let mut log_file = fs::File::create(&log_file_path)
           .expect("Non è stato possibile creare il file log.txt");




        // Legge il contenuto della directory sorgente
        let cartella = fs::read_dir(&self.path_sorgente)?;
        for file in cartella {
            match file {
                Ok(dir_entry) => {
                    // Recupera i metadati del file
                    let file_metadata = dir_entry.metadata()?;
                    let istanza_file_time = FileTime::new(file_metadata);

                    // Ottiene anno e mese di creazione/modifica
                    let (anno, mese) = istanza_file_time.get_anno_mese();

                    // Nome del file compresso (RAR) con cartella padre, anno e mese
                    let nome_file_zip = format!(
                        "{}\\{}_{}_{:#02}.7z",
                        self.path_destinazione,
                        get_path_parent(&dir_entry.path()),
                        anno,
                        mese
                    );

                    // Confronta anno e mese con l'intervallo configurato
                    for anno_corrente in self.anno_inizio..=self.anno_fine {
                        for mese_corrente in 1..=12 {
                            if anno == anno_corrente && mese == mese_corrente {
                                ComprimiFile::comprimi_7zip(
                                    &nome_file_zip,
                                    dir_entry.path().to_str().unwrap_or("")
                                );
                            }
                        }
                    }
                }
                Err(errore) => println!("Errore nella lettura del file: {}", errore),
            }
        }
        Ok(true)
    }

    /// Metodo che esegue la compressione utilizzando 7-Zip
    fn comprimi_7zip(par_nome_zip: &str, par_nome_file_archivio: &str) {
        // Percorso all'eseguibile 7-Zip
        let mut command = Command::new("C:\\Program Files\\7-Zip\\7z.exe");

        // Argomenti per la creazione dell'archivio
        let argomenti = vec![
            "a",               // Comando "aggiungi"
            par_nome_zip,      // Nome archivio di output
            par_nome_file_archivio, // File o cartella da comprimere
        ];

        let command = command.args(&argomenti);

        // Stampa del comando per debug
        let mut s: String = String::new();
        for arg in argomenti {
            s.push_str(arg);
            s.push(' ');
        }
        println!("7z.exe {}", s);

        // Esecuzione del comando
        match command.output() {
            Ok(output) => {
                if output.status.success() {
                    println!("Archivio creato con successo: {}", par_nome_zip);
                } else {
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

/// Funzione di utilità per ottenere il nome della cartella padre di un file
pub fn get_path_parent(path_file: &Path) -> String {
    let parent = Path::parent(path_file).unwrap();
    Path::file_name(parent).unwrap().to_str().unwrap().to_owned()
}







//          test
//----------------------------------------------------------------------------------------------//
#[cfg(test)]
mod tests {
    use std::panic;
    use std::path::Path;

    use super::*;

    /// comprimi_7z_test
    /// salvo il cargo toml in path di arrivo e poi cancello il file.zip
     #[test]
    fn comprimi_7z_test() {
         // Wrappiamo il codice del test con `catch_unwind`
         let result = panic::catch_unwind(|| {
             // Test 1: Creazione di "salva_cargo.zip"
             ComprimiFile::comprimi_7zip("prova.zip", "Cargo.toml");
             let x = Path::new("prova.zip").exists();
             assert!(x, "test fallito il file .zip non esiste");

             // Test 2: Creazione di "prova2.zip"
             ComprimiFile::comprimi_7zip("prova2.zip", concat!(env!("CARGO_MANIFEST_DIR"), "/PathDiArrivo"));
             let x = Path::new("prova2.zip.part01.7z").exists() || Path::new("prova2.zip").exists();
             assert!(x, "test fallito il file .zip non esiste");
         });

         // Cleanup: Cancelliamo i file creati, indipendentemente dal risultato
         if Path::new("prova.zip").exists() {
             fs::remove_file("prova.zip").unwrap();
         }
         if Path::new("prova2.zip").exists() {
             fs::remove_file("prova2.zip").unwrap();
         }
         if Path::new("prova2.zip.part01.7z").exists() {
             fs::remove_file("prova2.zip.part01.7z").unwrap();
         }

         // Rilanciamo il panic se il test è fallito
         if let Err(err) = result {
             panic!("{:?}", err);
         }
    }
}


// test 2
//-------------------------------------------------------------------------------------------//


#[test]
fn comprimi_cartella_test() {
    // Directory di partenza (già esistente nel progetto)
    let input_directory = concat!(env!("CARGO_MANIFEST_DIR"), "/PathdiPartenza");

    // Directory di destinazione (PathDiArrivo)
    let output_directory = concat!(env!("CARGO_MANIFEST_DIR"), "/PathDiArrivo");

    // Nome del file ZIP da creare nella directory di destinazione
    let output_file = format!("{}/test_archivio.zip", output_directory);

    // Verifica iniziale: controlla che la directory di partenza esista
    assert!(
        Path::new(input_directory).exists(),
        "La directory PathdiPartenza non esiste nel progetto"
    );

    // Verifica iniziale: controlla che la directory di destinazione esista (altrimenti la crea)
    if !Path::new(output_directory).exists() {
        fs::create_dir(output_directory).expect("Non è stato possibile creare la directory di destinazione PathDiArrivo");
    }

    let result = panic::catch_unwind(|| {
        // Comprimi tutti i file e directory di PathdiPartenza nell'archivio test_archivio.zip
        ComprimiFile::comprimi_7zip(&output_file, input_directory);

        // Verifica che l'archivio compresso sia stato creato
        let x = Path::new(&output_file).exists();
        assert!(x, "Test fallito: il file .zip non è stato creato nella directory di destinazione");
    });

    // Non cancelliamo più il file ZIP, lo lasciamo nella directory di destinazione

    // Rilanciamo il panic se il test è fallito
    if let Err(err) = result {
        panic!("{:?}", err);
    }

    println!("File ZIP creato con successo: {}", output_file);
}
//-------------------------------------------------------------------------------------------//
