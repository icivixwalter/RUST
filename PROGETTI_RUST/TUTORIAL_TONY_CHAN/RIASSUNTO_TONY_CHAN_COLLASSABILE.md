<details>
<summary><h1>RIASSUNTO_TONY_CHAN.md</h1></summary>
         @RAUST@TUTORIAL@TONY@CHAN
<details>
<summary><h2>GUIDA MARK DOWN</h2></summary>
<details>
<summary><h3>Note</h3></summary>
         si trova qui: https://learnxinyminutes.com/it/markdown/
   
         esempio i titoli:
   
</details>
</details>
</details>
<details>
<summary><h1>Questo è un <h1></h1></summary>
<details>
<summary><h2>Questo è un <h2></h2></summary>
<details>
<summary><h3>Questo è un <h3></h3></summary>
<details>
<summary><h4>Questo è un <h4></h4></summary>
<details>
<summary><h5>Questo è un <h5></h5></summary>
<details>
<summary><h6>Questo è un <h6>1</h6></summary>
   
</details>
</details>
</details>
</details>
<details>
<summary><h3>Guida completa per sviluppatori e principianti @giacom\o</h3></summary>
   Libro cha ha acquistato giacomo.
   
</details>
<details>
<summary><h3>Introduzione</h3></summary>
 

   Sviluppato inizialmente da  Graydon Hoare e sponsorizzato  
   da Mozzilla nato con l'obiettivo di combinare prestazioni elevate e  
   controllo della memoria di C e C++ orientato alla prevenzione della 
   sicurezza e degli errori tipo:  
      data races
      buffer overflows
   Sponsorizzato da Mozzilla perche migliorava le prestazioni di Firefox-
   Quali paradigmi di programmazione offre:
      programmazione imperativa
      programmazione funzionale
      programmazione orientata agli oggetti
   
      concetto di ownership   = per la gestione
         sicura della memoria e l'assenza di errore di concorenza.
      Cargo = il suo sistema di gestione dei pacchetti e build toll valido per sistema embeddet e motori di gioco.
   
      Sintassi familiare con C e C++
   
<details>
<summary><h4>Breve storia</h4></summary>
   
   
   Inizia del 2006 quando Graydon Hoare creo nel tempo libero questo linguaggio cercando di
   risolvere i problemi di sicurezza di C e C++ progettato per offrire prestazioni di basso livello di C e C++ 
   senza i problemi di memoria di
   
      buffer overflows
   
      data races        = In ambito informatico, una data race (o corsa ai dati) è una condizione che si verifica 
      quando più thread o processi accedono contemporaneamente a una risorsa condivisa (come una variabile), 
      e almeno uno di questi accessi è una scrittura. Questo può portare a risultati inattesi, imprevedibili e, 
      spesso, errati, poiché l'ordine in cui i thread accedono alla risorsa non è controllato.
   
   Nel 2009 fu sponsorizzato da mozzilla per miglioare le prestazione di Firefox e nel 2010 fu creato il primo 
   compilatore rustc scritto interamente in Rust.
   
   Innovativo è stato il sistema di "ownership = proprieta"
   
   Nel **2015** ragiunse la prima versione stabile 1.0 e con esso fu utilizzato per il progetto  del nuovo
   
      motore di rendering
   
      Un motore di **rendering**
   
        è un componente software o hardware che trasforma dati
        codificati, come il codice HTML di una pagina web o i dati di
        un modello 3D, in una rappresentazione visiva, che può essere
        visualizzata su uno schermo. In altre parole, è il responsabile di prendere le istruzioni e trasformarle in immagini o video
        che possiamo vedere.
   Nel 2017 vennero introdotte nuove funzionalita
   
      non-lexical lifetimes   = semplificarono la gestione della memoria (durata della validita di un riferimento)
      async/await             = resero la programmazione asincrona piu accessibile e potente.
   
   
   Nel 2021 con la versione 1.50 rusto miglioro il suo ecosistema ed ha
   introdotto ottimizzazione del compilatore e nuove libreria standard.
   
</details>
<details>
<summary><h4>Lo standard e le ultime versioni Lo standard viene mantenuta dalla</h4></summary>
   comunita attiva Rust Fundation con un modello di rilascio stabile e  
   organizzato con versioni stabili ogni 6 settimane e con impegno alla 
   retro compatibilita.
   
   Le innovazioni ultime sono:
   
      - miglioramento della programmazione asincrona
      - trait migliorati per il polimorfismo
      - Cargo ha semplifato la gestione dei pacchetti e dipendenze;
      - vers. 1.81 migliorato la funzione estena con extern C che
         prima creava panico non gestito oggi blocca il compilatore;
      - i metodi ambigui sono stati migliorati con la nuova versione
            i metodi sono piu prevedibili
   
</details>
</details>
<details>
<summary><h3>Rust pro e contro</h3></summary>
   
<details>
<summary><h5>Tra i punti di forza la sicurezza di gestione e previene gli errori di</h5></summary>
   
         - null pointer dreference
         - buffer overflows
         - la concorrenza senza la garbace colletion
      in questo modo è un liguaggio adatto per il software critico dove la sicurezza e l'affidambilita sono fondamentali;
      motoridi gioco e sofware di rete.
   
      La memoria viene gestia in modo sicuro con
         - ownership
            ogni valore stringa o numero ha un proprietario alla volta
            fino all'uscita dello scope dove la memoria viene deallocata
         - borrow checker
               Ruolo principale del borrow checker
   
                  Garantire la sicurezza della memoria: Rust utilizza un
                  sistema di ownership che stabilisce chi possiede un
                  dato in memoria e per quanto tempo. Il borrow checker
                  verifica che le regole di ownership siano rispettate:
                  Ogni dato ha un proprietario. Può esistere solo un
                  proprietario alla volta. Il dato viene deallocato
                  automaticamente quando il proprietario esce dallo scope.
   
                  Gestire il borrowing: Borrowing consente di accedere ai dati senza trasferire la proprietà. Questo può avvenire
                  in due modi:
                        - Mutabile: un solo riferimento mutabile  alla volta.
                        - Immutabile: multipli riferimenti immutabili
                        sono consentiti, ma non contemporaneamente a
                        riferimenti  mutabili.
   
                  Il borrow checker garantisce che questi riferimenti non entrino in conflitto tra loro.
   
                  Prevenire data race: Una data race si verifica quando:
                  Due o più thread accedono simultaneamente alla stessa risorsa. Almeno uno di essi modifica la risorsa.
                  Non ci sono meccanismi di sincronizzazione per gestire l’accesso. Il borrow checker impedisce questi 
                  scenari, obbligando l'accesso sicuro alle risorse condivise.
   
                  Impedire l'uso di dati non validi: Il borrow checker garantisce che i dati non vengano utilizzati dopo
                  essere stati spostati, invalidati o rilasciati.
            - Cargo, ecosistema che semplifica  la gestione dei pacchetti
                  e tooll, questo è uno dei punti forza.
            - Rust supporta i paradigmi (modelli o insieme di teorie
                  che la comunita scientifica utilizza per la propria
                  ricerca pratica e quindi il cambio di paradigma o
                  rivoluzione scientifica avviene quando un modello
                  sostituisce un'altro) e i tre paradigmi sono:
                     - imperativa;
                     - funzionale;
                     - concorrente;
                  rendendo rust flessibile adatto a diversi progetti e
                  permettendo agli sviluppatore di sceglire l'approccio
                  piu adatto.
   
                  Esempi:
   
                  1. ✅ Paradigma Imperativo
                     Nel paradigma imperativo descrivi passo dopo passo cosa deve fare il computer.
   
                     🔧 Esempio: Calcolo della somma di numeri
                     rust
                     Copia
                     Modifica
   
                           fn main() {
                               let mut sum = 0;
                               for i in 1..=5 {
                                   sum += i;
                               }
                               println!("La somma è: {}", sum);
                           }
                     Spiegazione:
   
                           let mut sum = 0; definisce una variabile mutabile.
   
                           Il ciclo for è tipico dello stile imperativo.
   
                           Modifichiamo lo stato (variabile sum) passo per passo.
                  2. ✅ Paradigma Funzionale
   
                     Rust supporta caratteristiche funzionali: funzioni pure, closure, iteratori, immutabilità, ecc.
   
                     🔧 Esempio: Somma con stile funzionale usando iteratori
   
                           fn main() {
                                  let sum: i32 = (1..=5).sum();
                                  println!("La somma è: {}", sum);
                              }
   
                        Oppure con map e filter:
                           fn main() {
                               let squares: Vec<i32> = (1..=5)
                                   .map(|x| x * x)
                                   .filter(|x| x % 2 == 0)
                                   .collect();
   
                               println!("Quadrati pari: {:?}", squares);
                           }
   
                        Spiegazione:
   
                           map, filter, collect sono tipici della programmazione funzionale.
   
                           Non c’è mutabilità o gestione manuale dello stato.
                  3. ✅ Paradigma Concorrente
   
                     Rust ha un eccellente supporto alla programmazione concorrente e parallela, grazie alla ownership e al
                     compilatore che garantisce la sicurezza dei thread.
   
                     🔧 Esempio: Esecuzione concorrente con std::thread
                           use std::thread;
   
                              fn main() {
                                  let handle = thread::spawn(|| {
                                      for i in 1..=5 {
                                          println!("Thread secondario: {}", i);
                                      }
                                  });
   
                                  for i in 1..=5 {
                                      println!("Thread principale: {}", i);
                                  }
   
                                  handle.join().unwrap(); // Attende la fine del thread secondario
                              }
   
                      Spiegazione:
   
                        thread::spawn crea un nuovo thread.
   
                        join() sincronizza e attende il completamento.
   
                        Nessun uso esplicito di unsafe, tutto è gestito in sicurezza dal compilatore.
   
                     Altri strumenti per la concorrenza:
                        std::sync::mpsc per canali (message passing)
   
                        tokio per programmazione asincrona
   
                        rayon per parallelismo con iteratori paralleli
   
               ✅ Conclusione
   
                  Paradigma   | Caratteristiche in Rust                        | Esempio
                  ------------|------------------------------------------------|-------------------------------
                  Imperativo  | Stato mutabile, cicli, controllo esplicito     | for, let mut, if, while
                  Funzionale  | Closure, immutabilità, iteratori               | map, filter, fold, sum()
                  Concorrente | Sicurezza a compile-time, gestione thread-safe | thread::spawn, join, channel
   
      Gli svantaggi
      - curva di apprendimento difficile
      - sintassi complessa e gestione della ownership complicata;
      - linguaggio eccessivo per progetti semplici;
      - tempo di compilazione lungo;
   
</details>
<details>
<summary><h4>Strumenti e tools di lavoro</h4></summary>
      
         puo essere utilizzato un semplice codice di testo ma quando i progetti
         sono complesso occorre un IDE; inoltre non utilizza il modello 
      
         JIT = just in time come C# ma il modello:
      
            AOT = ahead-of-time = ossia il codice viene tradotto in binari
            eseguibili  nella compilazione prima della sua esecuzione, garantendo
            sicurezza e ottimizzazione delle prestazione.
   
</details>
<details>
<summary><h4>LINGUAGGIO MACCHINA</h4></summary>
      La programmazione si distingue in BASSO LIVELLO = come assembly che costringe i  
      programmatori a lavorare molto vicino all'hardware ed a controllare la memoria.  
      
      Rust è un linguaggio ad ALTO LIVELLO  progettato per fornire astrazioni moderne  
      e controllando le risorse come la memoria.
      Essendo un linguaggio AOT = aead-of-time = significa che il compilatore rustc  
      trasforma in binario eseguibile il listato del programmatore risultando diverso  
      dai linguaggi Just-In-Time tipo Java o C# dove il linguaggio viene parzialmente  
      interpretato. Inoltre a differenza di C++ o Assembly dove il programmatore deve  
      deallocare la memoria, rust utilizza  il suo sitema di ownership e borrow cheching  
      consentendo sicurezza della memoria.
      Rust è anche cross-compilation = puo essere compilato per diverse piattaforme.  
</details>
<details>
<summary><h4>AI INTELLIGENZA ARTIFICIALE</h4></summary>
      Possiamo utilizzare Ai per avere un tutor in rust.  
   
</details>
</details>
<details>
<summary><h3>Debug, Boilerplate code e convenzioni</h3></summary>
         - debugging
            Rust utilizza macro dbg!() che stampa su console il valore e la posizione del codice  
            es.   let x = 5; dbg!(x) //stampa il valore 5;  
   
         - println!()  = consente di stampare variabili in fase di esecuzione ma alcuni tipi devono  
         essere implementati con trait Debug; e puoi anche creare tipi personalizzati da richiamare  
         con #[derive(Debug)]
   
         - boilreplate code  
            rust riduce la possibilita di costruire codice codice ripetitivo e rindodante con :
               @ i trait = che permetto di generare codice ripetitivo automaticamente  
               @ macro   = tipo vec![] crea un vettore con tutti gli elementi in modo automatico.  
         - zero-cost abstractions  
            puoi utilizzare strutture ad altro livello tipo Result + Option senza costi extra in termini  
            di prestazioni e potento scrivere codice boilerplate per la gestione degli errori e dei valori  
            nulli.
   
   
</details>
<details>
<summary><h3>Termini comuni in rust</h3></summary>
 
 
      * Inferire  
           Il compilatore puo inferire o ricavare i tipi di variabili   
           senza dichiararli im modo esplicito ma sono dedotti automaticamente dalla verbosita.
      * Monomorfizzazione:  processo in cui il compilatore genera funzioni generiche  
           per ciascun tipo utilizzato ottimizzando il codice ed eliminando l'overhead associato al dispach dinamico.  
               spiegazione: (In sintesi, l'overhead del dispatch dinamico è il costo aggiuntivo, in termini di tempo di esecuzione, associato alla determinazione  
               della funzione da chiamare a runtime. Questo costo è assente nel dispatch statico, che risolve la chiamata di funzione a tempo di compilazione.)  
      * Puntatori e puntatori raw:  
         a)  i puntatori in Rust, come & e &mut, sono sicuri perché rispettano le regole  
             del  borrowing  e  dell'ownership;  
         b) i  puntatori  raw,  come  *const  e  *mut,  sono  meno  sicuri  perché  
            permettono l'accesso diretto alla memoria senza le garanzie del borrowing.  
      * Heap e stack:  
         a) heap  =  il primo è usato per dati dinamici con dimensioni variabili, richiedendo però più risorse  
            per gestire la memoria.  
         b) stack = L’altro per dati di dimensioni fisse. Ha un accesso molto più rapido.  
      * Ownership  e  borrowing:  
         a) ownership    =  l'ownership  rappresenta  il  controllo  esclusivo  di  una  variabile  
         b) borrowing    =   prendere in prestito il valore di una variabile temporaneamente, sia in modo  
            _**mutabile**_   che _**immutabile**_, mantenendo la sicurezza della memoria. 
      * Operazioni  atomiche: 
         sono  operazioni  che  vengono  eseguite  come  un'unica  operazione  indivisibile,  
         usate per sincronizzare l'accesso concorrente ai dati tra thread senza incorrere in race conditions.  
   
      * Struct,  trait  e  crate:  
            - struct       = è  una  struttura  dati  che  raggruppa  variabili;  
            -  trait       = i  trait  definiscono  il comportamento  che  può  essere  
                           implementato  dalle  strutture  
            - crate        =  è  un'unità  di compilazione, come una libreria o un pacchetto.  
      * Null  pointer,  race  condition:
            - null  pointer   =   rappresenta  un  puntatore  che  non  punta  a  nessun  
                              valore  valido
            - race  condition =   si  verifica  quando  più  thread  accedono  a  dati  condivisi  
                     senza la corretta sincronizzazione, causando comportamenti imprevedibili. 
      * Attributi e riferimenti:  
         - gli attributi    = servono a modificare il comportamento del compilatore o del codice.  
         - i riferimenti    =  (&  e  &mut)  sono  puntatori  sicuri  che  permettono  l'accesso  
                     - a  valori  senza  trasferirne l'ownership. 
                     - 
      * Pattern  Matching:  
            consente  di  DESTRUTTURARE, CONFRONTARE  E GESTIRE I DATI in  modo  sicuro  e  conciso.  
            Utilizzando  la parola chiave  match, si  possono  esaminare  diversi casi di un  tipo,  
            come un  enum  o  Option, e gestire tutte le possibilità in modo esaustivo,  
            migliorando la sicurezza del codice. 
      * Polimorfismo ad hoc:  
            La capacita di una funzione o di un metodo di lavorare con diversi tipi di dati ma con  
            comportamenti diversi per ciascuno di loro. Si utilizza il 
            **<span style="font-size: 24px;">trait</span>**  che consentono di definire un insieme di
            metodi che devono essere implementati dai tipi che aderiscono al metodo. Ogni metodo puo
            avere una implementazione specifica per comportamenti differenziati.
   
      * Pattern
            Sono schemi ripetibili ed efficaci come soluzioni ricorrente ad esemp Option e Result  
            che gestiscono valori opzionali e gli errori in modo sicuro.  
      * Anti Pattern 
            Pratica dannosa che comporta la cattiva manutenzione degli errori. Ad es. utilizzo eccessivo ..
            dei puntatori raw con vulnerabilita della memoria in quanto non hanno il sistema  
            ownership e borrowing.  
</details>
<details>
<summary><h3>Commentare il codice</h3></summary>
      - Commenti su riga      //
      - Commenti multilinea   /**/
      - Commenti per la documentazione  ///  che sono utili per le funzioni le strutture ed i moduli  
            per spiegare come utilizzarli e con l'utility rustdoc genero la documentazione Html. 
   
   
   
   
   
   
   

</details>
</details>
<details>
<summary><h2>1 Le prime basi</h2></summary>
<details>
<summary><h3>ownership</h3></summary>
         Rust utilizza **ownership** che permette di sapere chi possiede una risorsa in ogni
         momento evitando i problemi dell'accesso concorrente.  
         FFI Foreign Function Interface che permette l'integrazione  con C e C++ quindi permette si  
         di scrivere codice sicuro, ma anche senza sacrificare la flessibilita sia per i progetti  
         nuovi e per quelli gia avviati.
</details>
<details>
<summary><h3>Cargo e la sintassi di base</h3></summary>
      Strumenti  di gestione dei paccjhetti e il build sistem per rust e gestisce tutto il ciclo  
      di vita dall'inizio dello svilupp fino alla distribuzione.
      * cargo new primo_progetto 
            crea un nuovo progetto con questo schema  
               primo_progetto 
               ├── Cargo.toml       = Il file Cargo.toml contiene i metadati del progetto e la lista delle dipendenze.
               └── src 
                   └── main.rs  
      * cargo test 
         * test
            Compila il progetto, esegue i test definiti e mostra i risultati. 
               Puoi utilizzare cargo test  che compila il progetto ed esegure i test definiti e mostra  
               risultati.
                  * use             = importa i moduli e le funzionalità specifiche da librerie esterne o interne.  
                  * main            = punto di ingresso dell'applicazione che non accetta argomenti per default  
                                       ma puo essere utilizzata.
                  * mut             = le variabili sono immutabili per default salvo con l'utilizzo di mut.  
                     esempio:  
            
                           fn main() { 
                               let messaggio = "Ciao, mondo!"; // Variabile immutabile 
                               println!("{}", messaggio); // Stampa sulla console 
                            
                               let mut numero = 42; // Variabile mutabile 
                               numero = 43; // È possibile modificarla 
                               println!("Il numero è {}", numero); 
                           } 
         * cicli ripetuti  = if,else, loop, while e for  ..
         * Result  e  Option.
                  permette la gestione sicura degli errori; esempio funzione che restituiscono un  
                  Result che viene gestito con un match oppure utilizzanto :
                        - unwrap(), .expect(); oppure l'operatore ? per propgare l'errore; esempio
                            
                            fn main() { 
                               let risultato = divisione(10, 2); 
                                
                               match risultato { 
                                   Ok(valore) => println!("Risultato: {}", valore), 
                                   Err(e) => println!("Errore: {}", e), 
                               } 
                           } 
                            
                           fn divisione(a: i32, b: i32) -> Result<i32, String> { 
                               if b == 0 { 
                                   Err(String::from("Divisione per zero")) 
                               } else { 
                                   Ok(a / b) 
                               } 
                           } 
         * mod    = moduli 
               rust puo organizzare il codice  in moduli  per suddividere il codice in  parte  
               piu piccolo e gestibili, possono definiti nello stesso file e su file diversi e
               resi pubblici mediante pub.
   
                   
                        mod calcoli { 
                            pub fn somma(a: i32, b: i32) -> i32 { 
                                a + b 
                            } 
                        } 
                         
                        fn main() { 
                            let risultato = calcoli::somma(5, 3); 
                            println!("Il risultato della somma è: {}", risultato); 
                        } 
         * use std::io 
               Partiamo con use std::io per importare le funzionalità di input/output. La funzione 
               main legge un numero dall'utente, lo converte da stringa a intero e lo valuta con 
               una struttura if. Il programma gestisce eventuali errori durante la lettura dell'input   
               o la conversione del numero, utilizzando .expect() per fornire messaggi di errore   
               chiari in caso di fallimento.
         * ownership (LA PROPRIETA DEL VALORE)
               sistema che gestisce la memoria in modo sicuro ogni valore spetta al proprietario 
               e quando esce dall'ambito (scope) viene automaticamente deallocato ed in questo modo
               elimina la garbace collection e previene i bug.
         * borrowing (PRESTITO DI UNA VARIABILE)
               permette di prestare una variabile senza trasferirne la proprieta sia con prestito
               MUTABILE che IMMUTABILE.
                  ESEMPIO:
                     fn main() { 
                         let s = String::from("ciao"); 
                         prendi_ownership(s); // Ownership trasferita 
                         // Non possiamo più usare `s` qui 
                      
                         let x = 10; 
                         prendi_in_prestito(&x); // `x` viene preso in prestito, ownership non trasferita 
                         // Possiamo ancora usare `x` qui 
                        } 
    
                     fn prendi_ownership(s: String) { 
                         println!("{}", s); 
                     } 
                      
                     fn prendi_in_prestito(y: &i32) { 
                         println!("{}", y); 
                     } 
         * use
            serve per importare  
               - i moduli                       = interi moduli o sottosezioni di essi;
               - le funzioni, strutture e tipi  = specifici elementi di un modulo  
               - Enum e varianti                = tipi di enum e loro varianti
               - Elementi di un modulo standard o di librerie esterne 
                     puoi accedere alle funzionalita delle librerie standard di terze parti.
                     ed un esempio la libreria st ricca di moduli.
               - Vedi esempio con l'utilizzo di std::io  che gestisce l'imputo e l'output  
                     della tastiera e lo schermo e fornisce gli strumenti per la gestione 
                     dei flussi stream i/o e gestione errori.
                 Vedi In io::stdin  legge  una  linea  di  input  dall'utente  
                        
               - Vedi  std::fs: fornisce funzionalità per lavorare con il file system,  
               - 
               - Vedi  std::collection: fornisce strutture dati utili come HashMap, Vec, BTreeMap  
                     strutture  dati  fondamentali  per  organizzare  e  gestire  le  collezioni  in  
                     modo efficiente:  
                     
               - Vedi  std::thread: gestisce la concorrenza con i thread
               - Vedi  std::async : per la sincronizzazione dei thread
               - Vedi  std::time  : per gestire il tempo e le durate temporali.
               - Vedi  std::env   : interagisco con l'ambiente di sistema come le variabili 
                                 - di ambiente
               - 
                     vedi i progetti  :
                     progetto ----> @std@io           = gestisce input output
                     progetto ----> @std@in           = in attesa di leggere linea input
                     progetto ----> @std@fs           = file system
                     progetto ----> @std@fcollection  = gestione strutture dati
                     progetto ----> @std@thread       = migliora le prestazioni con i thread
                     progetto ----> @std@async.04?    = sincronizzazione dei thread con Mutex e Arc
                     progetto ----> @std@time.05?     = misura la durata delle operazioni
                     progetto ----> @std@env.06?      = misura la durata delle operazioni
   
   
   
            use strumento performante
               La  dichiarazione  use  è  uno  strumento  performante  che  semplifica  l'accesso  alle 
               funzionalità  dei  moduli.  Che  si  tratti  di  leggere  input,  lavorare  con  file,  utilizzare 
               strutture  dati  avanzate  o  gestire  la  concorrenza,  Rust  offre  una  vasta  gamma  di 
               moduli standard accessibili tramite questa istruzione.
               Ciò permette di mantenere il codice organizzato e pulito, evitando ripetizioni inutili. 
   
               Questo è l'elenco degli strumenti  potenti  per  gestire  I/O,  dati,  concorrenza,  
               sincronizzazione,  e temporizzazione:
               +-------------------+-------------------------------+-------------------------------------------------------------+
               | Modulo            | Classi / Funzioni             | Descrizione                                                 |
               +-------------------+-------------------------------+-------------------------------------------------------------+
               | std::io           | stdin, stdout, Read, Write    | Gestisce l'I/O; lettura e scrittura da/verso flussi.       |
               | std::fs           | File, read_to_string, write   | Interazione con il file system; leggere/scrivere file.      |
               | std::collections  | HashMap, Vec, BTreeMap        | Strutture dati come mappe hash, vettori e alberi.           |
               | std::thread       | spawn, JoinHandle             | Concorrenza tramite thread per esecuzione parallela.        |
               | std::sync         | Mutex, Arc, RwLock            | Sincronizzazione e accesso sicuro a risorse condivise.      |
               | std::time         | Duration, Instant             | Tempo e durate temporali; utile per timeout e misurazioni.  |
               | std::env          | args, var, set_var            | Interazione con ambiente di sistema (argomenti/env).        |
               | std::net          | TcpListener, TcpStream,       | Funzionalità di rete per TCP e UDP.                         |
               |                   | UdpSocket                     |                                                             |
               | std::process      | Command, exit, Child          | Esecuzione di processi esterni e comandi di sistema.        |
               | std::cmp          | min, max, Ordering            | Funzioni e tipi per confronto e ordinamento.                |
               | std::option       | Option                        | Rappresenta un valore opzionale (presente o assente).       |
               | std::result       | Result                        | Gestione di successo o errore nelle operazioni.             |
               +-------------------+-------------------------------+-------------------------------------------------------------+
   
   
          Rust linguaggio sicuro 
            è progettato per il codice sicuro privo di errori nella gestione della memoria come ad
            esempio: dangling pointer
               (Un dangling pointer (in italiano: puntatore pendente o appeso) 
               è un puntatore che fa riferimento a una zona di memoria non 
               più valida o deallocata. Usarlo può causare crash, 
               comportamento indefinito o vulnerabilità di sicurezza.)
   
                  es.
   
                  int* p;
                     {
                         int x = 42;
                         p = &x; // p punta a x
                     } // x non esiste più: p ora è dangling!
                     printf("%d", *p); // ERRORE: accesso a memoria invalida
   
               🔥 Conclusione
                  Un dangling pointer è un pericolo nei linguaggi che non gestiscono la memoria in modo sicuro.
                  In Rust, non può succedere a meno che tu usi unsafe.
                  Il sistema di ownership + lifetimes lo impedisce a compile time, proteggendo il tuo programma.
            Quando si tratta di mutuo accesso ai dati in ambienti concorrenti, Rust introduce 
            un meccanismo di borrowing e reference counting che garantisce che due parti del 
            codice non possano accedere mutualmente e simultaneamente ai dati in modo che 
            possa  causare  conflitti
            Previene il race conditions,
                 linguaggio  assicura  che  un  dato  non  possa essere mutato mentre 
                 viene letto da altre parti del programma
         Protezione lifetime e delle reference
            cioè è il compilatore che individua e traccia chi sta accedendo a cosa e per quanto tempo.
            inoltre estende la protezione anche alle operazioni concorrenti e cioè eseguire delle 
            operazioni in parellelo senza  preoccupazione di deallocare la memoria oppure che si
            creano race conditions = dati lette mutato da altri.
            La programmazione asincrona è sicura in quanto perche utilizza
               Un task asincrono è un'unità di lavoro che può sospendersi e riprendere più tardi, senza bloccare il thread.
               Ti permette di scrivere codice che fa "altro" mentre aspetta (es. attende dati dalla rete, un file, un timer…).
               Viene gestito da una runtime asincrona (come tokio o async-std in Rust), che programma i task in modo efficiente.
               Cos'è un Future? = Un Future è un oggetto che rappresenta il risultato di un'operazione asincrona che 
               non è ancora disponibile, ma lo sarà in futuro.
   

</details>
<details>
<summary><h3>Le costanti e le variabili</h3></summary>

      Le costanti mantengono i loro valori immutabili. Le variabili consentono la  
      la memorizzazione e la manipolazione dei dati.  
      Pag. 38 - Sia le COSTANTI che le VARIABILI sfruttano la memoria RAM e cessano al termine  
      delle'esecuzione salvo in cui sono salvata su dispositivi esterni.  
      Let = parola chiave di dichiarazione e di default immutabili salvo se necesario per cambiare valore occorre  
      la parola chiame: mut  
            es. immutabile
               let x = 5; 
               println!("Il valore di x è: {}", x);  
            es. MUTABILE  
                   let mut y = 10; //DICHIARATA 10 MA MUTABILE
                   println!("Il valore iniziale di y è: {}", y); 
                   y = 20; //ASSEGNO NUOVO VALORE
                   println!("Il valore modificato di y è: {}", y);  
      Le costanti dichiarate con const sono empre immutabili. Rispetto alle variabili le costanti devono avere sempre  
         il tipo dichiarato  e possono essere utilizzate ovuncque nel programma; es   const MAX_PUNTI: u32 = 100_000;    
         MAX_PUNTI = valore massimo dichiarato come u32.   
      shadowing = consente di riutilizzare il nome della variabile in uno scope successivo quindi utile quando si ha  
         bisogno di trasformare il valore della variabile SENZA MODIFICARNE IL TIPO O LA MUTABILITA.  
         Con shadowing una nuova variabile con lo stesso nome viene creata nascondente la precedente es.  
               let z = 6; 
               let z = z + 1;       //shadowing della variabile con lo stesso nome aumenta di valore  
               let z = z * 2;       //shadowing della variabile con lo stesso nome aumenta di valore * 2 fino a 14
               Il compilatore capisce che z è dello stesso tipo.  
   
               ATTENZIONE che quando necessario è megli dichiare il tipo:   let a: f64 = 3.14;  // dichiarazione di una variabile di tipo floating point  
               la corretta tipizzazione delle variabili rende i codice Rust sicuro. 
</details>
<details>
<summary><h3>Tipologia delle variabili</h3></summary>

   PAG 40 - Le variabili sono contenitori che consentono ai programmatori di memorizzare e manipolare di dati durante l'esecuzione  
   del programma; ed in base al tipo associato, si stabilisce il valore che puo essere manipolato. In Rust le variabili sono divise  
   in tipologie per consentire al compilatore di eseguire controlli in sicurezza ed ottimizzazioni; le tipologie sono:  
      * __Tipi scalari__   : sono variabili che rappresentano il numero INTERO, VIRGOLA MOBILE, BOOLEANO e UN CARATTERE
         let intero: i32 = 42;         // i32, u32, i64  
         let float: f64 = 3.14;        // Virgola mobile f32 f64  
         let booleano: bool = true;  
         let carattere: char = 'R';  
      * __Tipi composti__  : sono variabili che che COMBINANO PIU VALORI in un **singolo tipo** cioè tuple ed array.  
            
            TUPLA  
               es:  let tupla: (i32, f64, char) = (42, 3.14, 'R');  
                let (x, y, z) = tupla;  
               questa tupla contiene un inter, un virgola mobile ed un carattere ed i valori possono essre destrutturati in x,y e z.  
            
            ARRAY  
                   let array: [i32; 3] = [1, 2, 3];  contiene 3 numeri interi e gli elementi sono accessibile con la funzione len:  
                        println!("L'array ha {} elementi.", array.len());    //len resituisce la lunghezza
            
            PATTERN MATCHING 
              PAG 41 = consente di verificare e destrutturare i valori in base al loro schema come ad esempio nelle tuple:  
   
                  fn main() { 
                      let tupla = (1, 0); 
                      match tupla { 
                           (0, y) => println!("Il primo valore è zero e il secondo è: {}", y), 
                           (x, 0) => println!("Il secondo valore è zero e il primo è: {}", x), 
                             _ => println!("Nessuno dei valori è zero"), 
                         } 
                     } 
                  con il match controlliamo la tupla e determiniamo il comportamento in base al valore.  
            
            PROPRIETA DELLE VARIABILI  
               quando una variabile viene assegnata ad un'altra variabile oppure viene passata ad una funzione la  proprieta viene  
               trasferita e la variabile originale non è piu utilizzabile, in questo modo gli errori legati alla gestione della  
               memoria vengono evitati es.  
   
                     let s1 = String::from("Ciao"); 
                      let s2 = s1;  // La proprietà di s1 è trasferita a s2 
                      
                      // println!("{}", s1); // Questo darà errore perché s1 non è più valido 
                      println!("{}", s2);  // Questo è valido
            BORROW - PRESTITO  
               PAG 42 - borrow è il prestito ossia la possibilita di accedere ai dati di una variabile SENZA TRASFERIRE LA PROPRIETA. 
                 - primo modo    = leggo i dati senza modifica (PRESTITO IMMUTABILE)
                 - secondo modo = leggo i dati senza modifica
   
      * __Le stringhe__    : sono piu complesse rispetto agli altri linguaggi per la sicurezza, e sono due tipi :  
            - &str   = stringhe slice di tipo immutabile  
                    let s_slice: &str = "Ciao, mondo!"; // stringa slice che punta a una stringa immutabile, utile  
                                       per operazioni leggere e quando non è necessario modificare la stringa   
            - String = stringhe allocate dinamicamente.  
                  -  let mut s_string = String::from("Ciao"); 
                     s_string.push_str(", mondo!"); 
                        Adatta per le modifiche durante l'esecuzione del programma, l'esempio inzia con Ciao e poi viene  
                        modificata con mondo.
            - 
   
      * __I booleani: s__  : sono i tipi bool con due valori false o true; e quindi utilizzati per il controllo di flusso  
            con strutture if ed while o nei pattern matching.  
            __Option<T>__  : Pag 45 - In rust non abbiamo i valori nulli (null pointer exceptions) ed i nulli vengono gestiti con  
                  con il tipo Option che rappresenta un valore  :
                     - Valore presente (Some(T))  
                     - Valore assente  (None)  
                  con questo meccanismo gestisco in MODO ESPLICITO ci casi di null es:  
   
                     fn main() { 
                         //2 variabile un valida 42 ed una nulla
                         let some_value: Option<i32> = Some(42); 
                         let no_value: Option<i32> = None; 
                      
                         match some_value { 
                             Some(x) => println!("Il valore è: {}", x), 
                             None => println!("Non c'è valore."), 
                         } 
                           //CON IL MATH viene gestito sia il valore valido   e sia il null con un msg
                           match no_value { 
                                Some(x) => println!("Il valore è: {}", x),    //esiste il valore
                                None => println!("Non c'è valore."),          //valore null msg
                            } 
                      }
                     - 
   
   
            *__tipi unitari__  
                PAG 45 -rappresentati da () = che è un tipo speciale  utilizzato quando non è necessario restituire  
                     alcune valore da una funzione o espressione è simile al void di C++ anche se viene trattato  
                     come un vero dato; con () il ritorno è implicito ed è utilie quanto una funzione esegue una  
                     azione come LA STAMPA DI UN MESSAGGIO SULLA CONSOLE.  
                        ESEMPIO  
                              fn main() { 
                                  stampa_messaggio(); 
                              } 
                               
                              fn stampa_messaggio() { 
                                  println!("Questa funzione non restituisce nulla."); 
                              } 
   
   
   
            é__Variabili immutabili e mutabili__
                  Le variabili sono immutabili di default e quindi il valore assegnato non puo essere  
                  cambiato in modo da scrivere codice sicuro e privo di errori.
                  Le variabili MUTABILI hanno la parola chiave mut
   
                     fn main() { 
                         let x = 5; // Immutabile 
                         // x = 6; // Questo darà un errore 
                          
                         let mut y = 5; // Mutabile 
                         y = 6; // Questo è valido 
                         println!("Il valore di y è: {}", y); 
                     } 
                      
   
            *__Shadowing  (Ombreggiatura)__  
               E' possibile dichiarare una nuova variabile con lo stesso nome di un'altra mettendo  
               in ombra la precedente e permettendo la ridefinizione se dover utilizzare la mutabilita.
               E' utile quando  si vuole modificare il tipo di dato di una variabile o applicare una  
               trasformazione dei suoi valori. 
                     fn main() { 
                         let x = 5; 
                         let x = x + 1; 
                         let x = x * 2; 
                      
                         println!("Il valore di x è: {}", x); // Stampa 12 
                     } 
   
   
   
            *__Enumerazioni (enum)__ 
                  Sono un tipo personalizzato che permette di definire una variabile   
                  che puo essere uno tra i valori possibilie e quindi rappresenta diverse
                  varieta di stati definiti:
   
                     enum Direzione { 
                         Su, 
                         Giù, 
                         Sinistra, 
                         Destra, 
                     } 
                      
                     fn main() { 
                         let movimento = Direzione::Su; 
                         match movimento { 
                             Direzione::Su => println!("Stai andando su!"), 
                             Direzione::Giù => println!("Stai andando giù!"), 
                             Direzione::Sinistra => println!("Stai andando a sinistra!"), 
                             Direzione::Destra => println!("Stai andando a destra!"), 
                         } 
                     } 
   
            *__Strutture  (struct)__
               simile a classi di altri linguaggi permettono di RAGGRUPPARE  diversi tipi  
               di variabili sono un unico TIPO DI DATO e quindi per rappresentare oggetti 
               complessi con molte proprieta:
   
                     struct Punto { 
                            x: i32, 
                            y: i32, 
                        } 
                         fn main() { 
                            let punto = Punto { x: 10, y: 20 }; 
                            println!("Punto: ({}, {})", punto.x, punto.y); 
                        } 
   
   
   
   
            *__Smart Pointers (Puntatori intelligenti)__
               I puntatore dei linguaggi C e C++ puntano solo ad una posizione della memoria
               senza la gestione della memoria che  è manuale.
               Mentre in rust in puntatori  Box, Rc, e RefCell forniscono le funzionalita
               di conteggio automatico dei riferimenti e la gestione della proprieta e 
               della  memoria garantendo che venga liberata in modo sicuro grazie alle regole
               della ownership di rust.
               Possono anche gestire situazioni condivise grazie al prestito borrowing.
   
                  use std::rc::Rc; 
    
                     fn main() { 
                         let valore = Rc::new(5); 
                         let clone = Rc::clone(&valore); 
                         println!("Valore: {}, Cloni: {}", valore, Rc::strong_count(&valore)); 
                     } 
   
</details>
<details>
<summary><h3>Scope delle variabili</h3></summary>


   SCOPE PAG 49 
      lo scope l'ambito di visibilita della variabile che inizia del blocco di codice   
      e finisce al fine blocco codice. Lo scopo riguarda quindi la visibilita della variabile  che quando esce dal blocco di codice viene distrutta in memoria ed  
      la memoria viene liberata con ownership (proprieta) e borrow (prestito)
   DIFFERENZE TRA SCOPE E LIFTIME      
            +-----------------+------------------------------------+------------------------------------+  
            |Concetto         | Scope                              | lifetime                           |  
            |-----------------|------------------------------------|------------------------------------|  
            |Cos'è?           | Ambito di visibilità di una        | Durata della validità di           |  
            |                 | variabile nel codice               | un riferimento                     |  
            |Quando si        |                                    |                                    |  
            |applica?         | A tutte le variabili               | Solo ai riferimenti (&T, &mut T)   |  
            |Gestito da       | Il compilatore in base alla        | Il borrow checker, in base al      |  
            |                 | posizione del codice               | flusso di vita dei dati            |  
            |Durata           | Determinata dalla struttura {}     | Determinata dalle regole del       |  
            |                 | del codice                         | borrow checker                     |  
            |                 |                                    |                                    |  
            +-----------------+------------------------------------+------------------------------------+  

      ESEMPIO DI CODICE
         Ecco un semplice diagramma visivo che illustra la differenza tra scope e lifetime.
                  fn main() {
                      let r;           // r è dichiarato qui (scope inizia)

                      {
                          let x = 5;   // x esiste solo in questo blocco
                          r = &x;      // ERRORE: r vuole riferirsi a x, ma x muore alla fine del blocco
                      }

                      println!("{}", r); // r sarebbe un dangling reference
                  }



         Timeline →
               ┌─────────────────────────────┐
               | main()                      |
               |                             |
               |  let r;                     |   r ----------┐
               |                             |               │
               |  ┌───────────────┐          |               │
               |  | {             |          |               │
               |  |   let x = 5;  |          |               │
               |  |   r = &x;     |          |  &x ----------┘ (qui si rompe!)
               |  | }             |          |
               |  └───────────────┘          |
               |                             |
               |  println!("{}", r); ← 💥 ERRORE: r contiene &x ma x è morto |
               └─────────────────────────────┘
         🔑 Conclusione
                  Lo scope di x è il blocco { ... }: fuori da lì, x non esiste più.
                  
                  Il lifetime del riferimento &x termina con lo scope di x, ma r vuole ancora usarlo dopo.
                  
                  Rust impedisce questo errore a compilazione, grazie al borrow checker.
                  




   REFERENCES  
         Il life time è collegato al references che rappresenta il modo sicuro per l'accesso  
         alla variabile senza cedere la proprieta.
         FAQ
            Come si puo accedere alla variabile ? = tramite il referece

      Il references consente di prendere in prestito una variabile, leggerla e modificarla.
      ✅ In sintesi: pag. 49

         le reference ti permettono di leggere o modificare una variabile a seconda del tipo di prestito:
            
            &T → solo lettura (immutabile).
            
            &mut T → lettura e scrittura (mutabile).
         
         Ma non puoi farlo liberamente: Rust impone regole di sicurezza ferree.
         Schema: Reference in Rust
            Tipo di reference   | Sintassi   | Cosa consente       | Regole
            --------------------|------------|---------------------|-------------------------------------------------------------
            Immutabile          | &x         | Solo lettura        | Puoi avere più reference immutabili contemporaneamente
            Mutabile            | &mut x     | Lettura e scrittura | Puoi avere solo una reference mutabile alla volta
                                |            |                     | Non puoi avere una &mut e & attive nello stesso scope

     In rust ogni reference ha un life time con cui rast stabilisci i tempi di validita di una variabile mediante il compilatore 
     affiche la variabile non sopravviva oltre il tempo in cui i suoi dati sono validi.
         Esempio:

            fn prendi_riferimento<'a>(s: &'a str) -> &'a str { 
                s 
            } 
             
            fn main() { 
                let stringa = String::from("ciao"); 
                let riferimento = prendi_riferimento(&stringa); 
                println!("{}", riferimento); // valido perché stringa vive più a lungo di riferimento 
            } 


               La  funzione  prendi_riferimento  accetta  una  reference  con  un  lifetime  'a  e  ne 
               restituisce un’altra con lo stesso lifetime.
               Questo vincolo assicura che il riferimento 
               restituito non possa essere utilizzato oltre il tempo in cui stringa è valida ossia previene
               il dangling pointers. (puntatori penzolanti):

                     Un puntatore penzolante (dangling pointer in inglese) è un riferimento a un'area di memoria 
                     che non è più valida. Significa che stai cercando di accedere a dei dati che:  
                        sono stati deallocati (liberati dalla memoria), oppure
                        non esistono più perché erano temporanei (es. una variabile locale uscita dallo scope).

                         Esempio in C (che permette errori del genere)
                              int* getPointer() {
                                  int x = 42;
                                  return &x; // ❌ ERRORE: x è locale e verrà distrutto alla fine della funzione
                              }   // qui viene distrutta la x
      
                              int main() {
                                  int* p = getPointer();
                                  printf("%d\n", *p); // comportamento indefinito!
                              }
      
                              p è un puntatore penzolante: punta a una variabile (x) che non esiste più.


                         Esempio in Rust
                            fn get_ref() -> &i32 {
                               let x = 10;
                               &x // ❌ Errore: x verrà eliminata alla fine della funzione
                              }

                              Rust evita i puntatori penzolanti a compile-time, grazie al sistema di ownership e borrowing.
                              Rust ti blocca con un errore tipo:
                                 error[E0515]: cannot return reference to local variable `x`
</details>
<details>
<summary><h3>Let e altre dichiarazioni</h3></summary>
   Immutabilita
      let è l'istruzione fondamentale per la dichiarazine e l'inizializzazione della variabile e per una  
      questione di sicurezza una volta assegnato il valore esso è immutabile.  
            Esempio  
               fn main() { 
                   let x = 5; 
                   // x = 6; // Questo genererà un errore di compilazione  
                   println!("Il valore di x è: {}", x); 
               } 
   Modificabilita  
      se assegni Let  mut alla variabile si permette la modifica del suo valore e mantenendo la proprieta  
         es,

               fn main() { 
                   let mut y = 5; 
                   y = 6; // Questo è valido 
                   println!("Il valore di y è: {}", y); 
               } 
                  
   SHADOWING  
         L'istruzione let supporta anche il cosiddetto shadowing, che consente di dichiarare  
         una  nuova  variabile  con  lo  stesso  nome di  una  già  esistente,  "oscurando"  quella  precedente.  
          Questo  può  essere  utile  quando  si  desidera  riutilizzare  un  nome  di 
            variabile per un valore calcolato o trasformato senza doverlo mutare: 
   
               //uso shadowing = per ricalcolare z
               fn main() { 
                      let z = 5; 
                      let z = z + 1; // Shadowing: z assume un nuovo valore 
                      let z = z * 2; 
                      println!("Il valore di z è: {}", z); // Stampa 12 
                  } 

   OMISSIONE TIPO DI DATO
      Let permette di omettere il tipo di dato della variabile che sara dedotto dal compilatore.
         es. 
            fn main() { 
                     let a = 10; // Il compilatore deduce che a è un intero (i32)   
                    let b = 3.14; // Il compilatore deduce che b è un numero a virgola mobile (f64)  
                             println!("a: {}, b: {}", a, b);  
                        } 
   LET DESTRUTTURARE I DATI COMPLESSI  
      con let puoi destrutturari dati complessi o tuple o array:
            fn main() { 
                   let (c, d) = (30, 40); // Assegna 30 a c e 40 a d 
                   println!("c: {}, d: {}", c, d); 
               }

   LET IGNORA I VALORI
      con l'operatore _ let ignora il valore es.
         fn main() { 
                let (_, e) = (50, 60); // Ignora il primo valore e assegna 60 a e   
                println!("Il valore di e è: {}", e); 
            } 
             

       Let è quindi un costrutto semplice sicuro e versatile che permette di scrivere codice chiaro ed   
       efficiente senza errori.

   ALTRE PAROLE CHIAVE (const , pub, priv )
      Rust non ha parole chiave come in C e C# ref, out, readonly, o public  ma ha dei concetti simili.  
       - const
            valore delle costanti sempre immutabili utilizzate per definire array statici.


               const MAX_VALORE: i32 = 100; 
                
               fn main() { 
                   println!("Il valore massimo è: {}", MAX_VALORE); 
               }   
       - visibilita
            con le parole  chiave  pub  e  priv. 
            Per default i moduli, le strutture e le funzioni sono private visibili solo all'interno del modulo
            in cui sono definiti.
            Se una strttura una funzion o un campo deve essere dichiarato pubblico occorre utilizzare la parola
            chiave pub:
                  //la gestione della visibilita è importante in quanto mantieni l'incapsulamentoe e limiti 
                  //l'accesso non autorizzato.
                  mod mio_modulo { 
                         
                         //definisco una struttura pubblica
                         pub struct MiaStruttura { 
                             //campo pubblico
                             pub campo_pubblico: i32, 
                             //campo privato perche non definito e quindi di default priv.
                             campo_privato: i32, 
                         } 
                        
                        //implemento la struttura pubblica
                         impl MiaStruttura { 
                             pub fn nuova(campo_pubblico: i32, campo_privato: i32) -> MiaStruttura { 
                                 MiaStruttura { campo_pubblico, campo_privato } 
                             } 
                         } 
                     } 
                      
                     fn main() { 
                         let struttura = mio_modulo::MiaStruttura::nuova(10, 20); 
                         println!("Campo pubblico: {}", struttura.campo_pubblico); 
                         // println!("Campo privato: {}", struttura.campo_privato); // Questo darà errore perché il 
                         campo è privato 
                  } 

       - readonly
            non esiste tale parola chiave in quanto la natura delle variabili è gia gestita di default essendo
            immutabili di default. Infatti per la variabile non dichiarata mut è immutabile e non occorre
            la parola chiave readonly.  
       - gestione puntatori
            In rust viene usato per la gestione dei puntatori il presito - borrowing - e il - reference -
            &     = creo riferimento immutabili;
            &mut  = creo riferimento mutabili;
            e quindi con il borrowing passo i dati senza trasferire la proprieta.

               fn main() { 
                      let mut x = 5; 
                      let y = &x; // riferimento immutabile 
                      let z = &mut x; // riferimento mutabile 
                      println!("y: {}", y); 
                      *z += 1; 
                      println!("z: {}", z); 
                  } 


  
  
       - la parola chiave out non esiste  

            in rust non esiste la parola chiae out come in C# perche se la FUNZIONE deve  
            modificare un valore si passa una variabile con riferimento mut.
</details>
<details>
<summary><h3>Operatori e metodi</h3></summary>
   
   Gli operatori aritmetici pag 55  
      - addizione +  
      - sottrazione -  
      - moltiplicazione *  
      - divisione /  
      - modulo %  

   Gli operatori logici pag 55  
      
      che restituiscono condizioni booleane true o false : AND &&; OR ||; NOT !
   
   Gli operatori di confronto pag 55  
      
         utilizzati per confrontare i valori e determinare le ralazioni tra di essi: 
            uguale ==; diverso !=; maggiore >; minore <;  maggiore e uguale >=; minore e uguale <=;  

   Gli operatori di assegnazione pag 55  
      
         servono per assegnare i valori alle variabili 
          come (=) e gli operatori composti come +=, -=, *=,  /=  e  %=,  servono  per  assegnare  valori  a  variabili.  

</details>
</details>
</details>