/*
================================================================================
 FILE_TIME - Gestione anno/mese da metadati file
--------------------------------------------------------------------------------
 FUNZIONALITÀ PRINCIPALI
  1. Creazione di un'istanza `FileTime` a partire dai metadati (`Metadata`)
  2. Recupero della data di creazione se disponibile, altrimenti data di modifica
  3. Conversione di `SystemTime` in `DateTime<Utc>` per formattazione flessibile
  4. Estrazione dell'anno in formato `i32`
  5. Estrazione del mese in formato `i32`
  6. Restituzione di anno e mese in un'unica tupla `(anno, mese)`

--------------------------------------------------------------------------------
 ATTIVITÀ ESEGUITE
  - Implementato costruttore `new` per inizializzare `FileTime` con `Metadata`
  - Creata funzione interna `get_modification_time` per ottenere `DateTime<Utc>`
  - Logica di fallback: priorità data di creazione, poi data di modifica
  - Funzioni `get_anno` e `get_mese` con parsing e gestione valori di default
  - Funzione `get_anno_mese` per restituire una tupla con anno e mese
  - Implementati test automatici per verifica anno, mese e tupla anno/mese
  - Funzione helper di test per creare file temporaneo e ottenere metadati

--------------------------------------------------------------------------------
 SEGNALIBRI / INDICE CODICE
 [struct FileTime]          → Contiene i metadati del file
 [new]                      → Costruttore FileTime
 [get_modification_time]    → Recupero data creazione o modifica
 [get_anno]                 → Ritorna solo l'anno (i32)
 [get_mese]                 → Ritorna solo il mese (i32)
 [get_anno_mese]            → Ritorna tupla (anno, mese)
 [tests]                    → Test unitari per anno, mese, anno/mese
 [crea_file_temp]           → Funzione di utilità per i test

--------------------------------------------------------------------------------
 NOTE TECNICHE
 - Usa `chrono` per conversione e formattazione date
 - Compatibile con sistemi che forniscono metadati di creazione
 - Su filesystem senza data di creazione, viene usata la data di modifica
================================================================================
*/

use std::fs::Metadata;
use chrono::{DateTime, Utc};

pub struct FileTime {
    pub metadati_file: Metadata, // Memorizza i metadati del file
}

impl FileTime {
    // Costruttore per creare una nuova istanza di FileTime
    pub fn new(metadati: Metadata) -> FileTime {
        FileTime {
            metadati_file: metadati,
        }
    }

    // Funzione helper per ottenere `SystemTime` trasformato in `DateTime<Utc>`
    //2025.08.17_@modifica01_@MESE@DI@MODIFICA_(Restuisco il mese in forma di numero di modifica del file)
    fn get_modification_time(&self) -> Option<DateTime<Utc>> {
        // Usa esclusivamente la data di modifica del file.
        self.metadati_file.modified().ok()
           .map(Into::into) // Converte `SystemTime` in `DateTime<Utc>`
    }



    // Metodo per ottenere l'anno di modifica/creazione del file
    pub fn get_anno(&self) -> i32 {
        if let Some(dt) = self.get_modification_time() {
            let anno_str = format!("{}", dt.format("%Y")); // Formatta l'anno
            anno_str.trim().parse::<i32>().unwrap_or(0)    // Converte in i32
        } else {
            0 // Se la data non è disponibile, ritorna 0
        }
    }

    // Metodo per ottenere il mese di modifica/creazione del file
    pub fn get_mese(&self) -> i32 {
        if let Some(dt) = self.get_modification_time() {
            let mese_str = format!("{}", dt.format("%m")); // Formatta il mese
            mese_str.trim().parse::<i32>().unwrap_or(0)    // Converte in i32
        } else {
            0 // Se la data non è disponibile, ritorna 0
        }
    }

    // Metodo per ottenere una tupla (anno, mese)
    pub fn get_anno_mese(&self) -> (i32, i32) {
        (self.get_anno(), self.get_mese())
    }
}

//              TEST
//______________________________________________________//

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Local};
    use std::{fs, io::Write};

    // Funzione helper per creare un file temporaneo e ottenere i suoi metadati
    fn crea_file_temp() -> Metadata {
        let temp_file_path = "temp_test_file.txt";
        let mut file = fs::File::create(temp_file_path).unwrap(); // Crea un file temporaneo
        writeln!(file, "Test temporaneo").unwrap();              // Scrive qualcosa nel file
        let metadata = fs::metadata(temp_file_path).unwrap();    // Ottiene i metadati
        metadata
    }

    #[test]
    fn test_get_anno() {
        let now = Local::now();
        let anno_corrente = now.year();

        let metadata = crea_file_temp();
        let file_time = FileTime::new(metadata);

        let anno = file_time.get_anno();
        println!("Anno trovato: {}", anno);

        assert_eq!(
            anno, anno_corrente,
            "Il test fallisce perché l'anno trovato non corrisponde a quello corrente"
        );

        fs::remove_file("temp_test_file.txt").unwrap();
    }

    #[test]
    fn test_get_mese() {
        let now = Local::now();
        let mese_corrente = now.month();

        let metadata = crea_file_temp();
        let file_time = FileTime::new(metadata);

        let mese = file_time.get_mese();
        println!("Mese trovato: {}", mese);

        assert_eq!(
            mese, mese_corrente as i32,
            "Il test fallisce perché il mese trovato non corrisponde a quello corrente"
        );

        fs::remove_file("temp_test_file.txt").unwrap();
    }

    #[test]
    fn test_get_anno_mese() {
        let now = Local::now();
        let anno_corrente = now.year();
        let mese_corrente = now.month();

        let metadata = crea_file_temp();
        let file_time = FileTime::new(metadata);

        let (anno, mese) = file_time.get_anno_mese();
        println!("Anno trovato: {}, Mese trovato: {}", anno, mese);

        assert_eq!(
            anno, anno_corrente,
            "Il test fallisce perché l'anno trovato non corrisponde a quello corrente"
        );

        assert_eq!(
            mese, mese_corrente as i32,
            "Il test fallisce perché il mese trovato non corrisponde a quello corrente"
        );

        fs::remove_file("temp_test_file.txt").unwrap();
    }
}
