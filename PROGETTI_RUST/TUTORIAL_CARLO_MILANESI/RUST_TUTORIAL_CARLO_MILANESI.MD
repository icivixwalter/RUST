RUST_TUTORIAL_CARLO_MILANESI
	Note
		questo è un tutorial in italiano di carlo milanensi e 
		si trova qui:


	CREARE_FILE_PROGETTO
		rust è create per Tier o 3 livelli con diverse garanzie.
		Le piattaforme sono identificate con la target-triple = ossia la stringa che (terna-bersalio) che informa il compilatore su quale output deve essere prodotto.

		LIVELLO 1
			Le piattaforme di livello 1 possono essere pensate come "con garanzia di compilazione e funzionamento".
		LIVELLO 2
			Le piattaforme di livello 2 possono essere pensate come "con compilazione garantita". I collaudi automatizzati non vengono eseguiti, e quindi non è garantito che si producano eseguibili che funzionino, ma queste piattaforme spesso funzionano abbastanza bene e le migliorie sono sempre benvenute! 

		LIVELLO 3
			Le piattaforme di livello 3 sono quelle per cui Rust ha supporto, ma per applicare delle modifiche non è necessario né eseguire la compilazione, né passare i testi di collaudo. Gli eseguibili funzionanti su tali piattaforme possono essere lacunosi, dato che la loro affidabilità è spesso definita in termini di contributi dalla comunità. In aggiunta, gli artefatti di rilascio e gli installatori non sono forniti, ma ci possono essere infrastrutture comunitarie che li producono in luoghi non ufficiali.

	INSTALLAZIONE
		su linux, windows ecc..

	NUMERO_DI_VERSIONE
		Aprire il terminale e controllare il numero di @versione di @rust:
			rustc --version

		Per windows controllare la variabile di sistema con 
			echo %PATH%.
		se non esiste impostarla manualmente in quanto  gli eseguibili di Rust si trovano in una directory come "C:\Program Files\Rust stable GNU 1.x\bin".

		... RUST non fa i link

	CREARE DIRECTORY PER I PROGETTI
		per iniziare creare una directory di progetto e con la funzione main. es.
			fn main() {
		main senza parametri () vuote, e con una macro
			println!("hello word");
		infatti println senza 	(!) = è una funzione	
		infatti println! con 	(!) = è una MACRO
		{};							= le parentesi racchiudono tutte le funzioni e la riga finisce con 
									il ; perchè Rust è un linguaggio orientato alle ESPRESSIONI infatti
									la maggior parte delle cose sono espressioni e con il ; si dichiara
									la fine della espressione.

	COMPILARE ED ESEGUIRE SONO PASSI SEPARATI
		Rust è un linguaggio compilato in anticipo tipo il c++ perchè si ha il sorgente ed un eseguibile
		sulle piattaform a differenza degli  dei linguaggi dinamici tipo Python dove occore consegnare
		il sorgente .py ma occorre anche la macchina virtuale per l'esecuzione, cià che non serve in rust.
		La compilazione con rust c = va bene per i programmi semplici altrimenti occorre usare cargo.

		CARGO
			È un sistema di builde e di gestore di pacchetti Rust  per i progetti ed esegue 3 cose:
				- compila il codice;
				- scarica le librerie;
				- e costruisce le librerie;

			la versione di cargo è con : cargo --version

			Cargo utilizza un sistema ad ordine, la directory principale per README + LA LICENZA + I LINK quindi il sorgente va nella directory 
				src

			nella src ci sono i sorgenti tipo main.src oppure anche librerie lib.rs

			Per gestire con cargo occorre creare una direcotry madre
			una directory src dove inseire i file sorgenti menetre nella directory principale il Cargo.toml
			il toml = Questo file è nel formato TOML (Tom's Obvious Minimal Language). TOML è simile al formato INI, ma ha alcuni vantaggi, e viene usato come formato di configurazione di Cargo.
			Dentro questo file, si digiti la seguente informazione:

					[package]				= 	le seguenti direttive costruiscono un pacchetto

					name = "hello_world"	= il nome del programma
					version = "0.0.1"		= la versione
					authors = [ "Il tuo nome <tu@esempio.it>" ] = chi lo ha scritto
				
				hello_word_cargo
					Cargo.toml
					src
						main.rs


			Cargo viene utilizzato per grandi progetti, perchè  si iniziano ad usare i "crate"; questi, in altri linguaggi di programmazione, sono chiamati ‘librerie’ o ‘pacchetti’. 

			Con cargo per rilasciare il progetto finale occorre questo comando:
				cargo build --release
				 
			con questa secondo profilo la compilazione è piu lenta ma viene ottimimizzata e velocizzato
			il file finale .exe

			CARGO LOCK
				è il file in cui cargo tiene conto delle dipendenze dell'applicazione.


			CARGO CREA PROGETTO
				per creare un nuovo progetto occorre eseguire questo comando
					cargo new nome_progetto --bin
					 						--bin = opzione per creare direttamente un eseguibile e non
					 						una libreria

	GIOCO-INDOVINA
		Impostazione
			imposto il nuovo progetto nella directory con il comando
				cargo new gioco_indovina --bin
				                         --bin = creato un nuovo progetto eseguibile non una libreria.

         Elaborare un tentativo
         	Elaboraiamo il file main per consentire al giocatore di inserire i tentativi:

         		use std::io;		= libreria per lo standard io
         		fn main() { 		= e () indicano che non ci sono argomenti,
         		let 				= istruzione let, che viene usata per creare ‘legami a variabili’ 
         							  [variable bindings].

				  esempio
				  	let foo = bar;
					  	Questa istruzione crea un nuovo legame chiamato foo, e lo lega al valore bar. In molti linguaggi, questo si chiama ‘variabile’, ma i legami di variabile di Rust hanno alcuni assi nella manica.

					  	l'istruzione senza mut rende immutabile il legame in quanto di default

					  		let foo = 5; // immutabile.
							let mut bar = 5; // mutabile

						con l'istruzione 
							    let mut tentativo = String::new();
					    viene creata una variabile mutevole di tipo Stringa associata ad un tipo String
					    e non un nuova istanza di String e quindi è un METODO DI CLASSE o un METODO STATICO.

				    	con l'istruzione
				    		io::stdin().read_line(&mut tentativo).expect("non si riesce a leggere la riga");
				    		utilizza lo std::io
				    			std::io::stdin   = legge l'imput da linea di comando
		    		    		(&mut tentativo) = imposta la variabile tentativo mutevole con il 					risultato
		    		    		.expect("non ..) = visto che anche un Result (io::Result) come primo
		    		    		caso generico e anche sottolibreria ma viene utilizzato per il messaggio di errore se si va in panic.

    		    		le {} nella macro sono segna posto
    		    		Questa riga stampa la stringa dove abbiamo salvato il nostro input. Le parentesi graffe {} sono dei segna-posto per passare tentativo come argomento. Se avessimo scritto diversi {}, avremmo passato diversi argomenti:

									let x = 5;
									let y = 10;

									println!("x e y: {} e {}", x, y);

	GENERARE UN NUMERO SEGRETO 
		crate rand
			cassone o scatola oppure libreria che permette per i numeri casuali, nel frattempo viene
			creato un nuovo progetto per i numero segreto:
				mkdir genera_numero_segreto

			con 
				cargo new numero_segreto --bin 

			dopo aver creato il progetto per utilizzare la libreria rand occore indicare la dipendenza
			nel toml
				rand="0.3.0"

			dependencies per sapere quali dipendenze ci sono da crate esterni, e quali versioni di essi sono richieste. In questo caso, abbiamo specificato la versione 0.3.0, che Cargo capisce essere qualunque rilascio che è compatibile con questa specifica versione.
			Cargo va a prendere le versioni più recenti di ogni cosa dal registry, che è una copia di dati presi da Crates.io. Crates.io è il posto dove la gente nell'ecosistema di Rust invia i suoi progetti open source in Rust per farli usare ad altri.

			Dopo aver aggiornato il registry, Cargo verifica la nostra sezione [dependencies] e scarica tutti i pacchetti che non abbiamo ancora. In questo caso, mentre abbiamo detto soltanto che volevamo dipendere da rand, ci siamo presi anche una copia di libc. Questo perché rand dipende da libc per funzionare. Dopo averli scaricati, li compila, e poi compila il nostro progetto.
			Quando costruiamo un progetto da zero ed impostiamo la dipendenza ad es. rand=0.3.0, cargo
			imposta il Cargo Lock a questa versione e se uscira una successiva tipo  rand=0.3.9 il 
			cargo lock impedisce di scacarica la nuova versione per il pericolo della incompatibilita
			retroattiva a meno che non impostiamo a mano il 0.3.9 NEL TOML
			ALTERNATIVA per FORZARE IL CARGO.LOCK possiamo usare
				cargo update
			che significa ignora il cargo.lock e cerchera di aggiornare le versioni > 0.3.0 and < 0.3.9
			ma non la versione 0.4.0 che deve essere esplicitamente impostata nel .toml

			per utilizzare rand dopo averlo dichiarato nelle dipendence del .toml
			occore inserire nel file la dichiarazione di utilizzo:
				extern crate rand;


			viene inserito 
				use std::cmp::Ordering;

			STD::CMP = è un metodo che restituisce un ORDERIN cioè un enum  < = >.
			cmp = viene chiamato per qualunque oggetto da confrontare con riferimento allo
			stesso oggetto da confrontare  e restisce un Ordering.
			Ordering = è un enum con 3 POSSIBILI VARIANTI
				LESS, EQUAL, GREATER

			serve per confrontare i numeri  e deve essere integrato con un match
				
				match tentativo.cmp(&numero_segreto) {
				    Ordering::Less    => println!("Troppo piccolo!"),
				    Ordering::Greater => println!("Troppo grande!"),
				    Ordering::Equal   => println!("Hai vinto!"),
				}


			
			CONVERSIONE DEL TIPO STRINGA CON PARSE	

				Il metodo parse() applicato a una stringa analizza la stringa estraendone un numero di qualche tipo. Dato che tale metodo può riconoscere vari tipi di numeri, dobbiamo suggerire a Rust il tipo esatto del numero che vogliamo. Pertanto, scriviamo let tentativo: u32. I due-punti (:) dopo tentativo dicono a Rust che stiamo annotando il tipo del legame. u32 è il tipo intero senza segno a trentadue bit. 
					fn main() {
				        let tentativo: u32 = tentativo.trim().parse()
				        .expect("Prego, digita un numero!");
				}


			LOOP
				Viene creato un loop
				ed inserito la gestione del panic

	LEGAMI DI VARIABILE - 								4.1
			
			Per introdurre un legame, si usa la parola-chiave let, così:
					let x = 5;

			I PATTERN - 4.1.1
				negli altri linguaggi si chiama variabile, ma i legami di variabile di Rust hanno alcuni assi nella manica. Per esempio, il lato sinistro di un'istruzione let è un ‘pattern’, non un semplice nome di variabile

						let (x, y) = (1, 2);
						Dopo che questa istruzione viene eseguita, x varrà uno, e y varrà due. (vedere i pattern)

			ANNOTAZIONI DI TIPO - 4.1.2
				Rust è un linguaggio tipizzato staticamente, il che significa che specifichiamo subito i tipi, e questi vengono verificati in fase di compilazione.
				Ma rust ha anche la 
					INFERENZA DI TIPO. Se riesce a desumere qual'è il tipo di qualche 
										dato, Rust non costringe a digitarlo splicitamente.

				Se vogliamo, possiamo aggiungere il tipo di dato.
					let x: i32 = 5;

				Rust ha molti tipi interi primitivi. I loro nomi cominciano con i per gli interi con segno, e con u per gli interi senza segno (unsigned). Le dimensioni intere possibili sono 8, 16, 32, e 64 bit.

			MUTABILITÀ
				Di default, i legami sono immutabili. Questo codice non compilerà:
					let x = 5;
					x = 10;
				Se si vuole che un legame sia mutabile, si deve usare mut:
					let mut x = 5; // mut x: i32
					x = 10;

				LA RAGIONE PRINCIPALE DELLA IMMUTABILITA DEI LEGAMI E' LA SICUREZZA.
				Se ci si dimentica di dire mut, il compilatore se ne accorgerà, e farà sapere che si ha mutato qualcosa che si potrebbe non aver inteso mutare. Se i legami fossero mutabili di default, il compilatore non sarebbe in grado di dirlo. Se si intendesse proprio la mutazione, allora la 
				soluzione è facilissima: aggiungere mut.

			INIZIALIZZARE I LEGAMI
				I legami di variabile in Rust hanno un altro aspetto che differisce da altri linguaggi: I LEGAMI DEVONO ESSERE INIZIALIZZATI CON UN VALORE PRIMA DI POTERLI USARE.
				Se utilizziamo ugualmente una variabile senza inizializzabile Rust ci da errore; Rust non ci permetterà di usare un valore che non è stato inizializzato.
					es.
						fn main() {
						    let x: i32;

						   !=macro
						   				   1°               2°
						   2 argomenti |------------------||--|
						    println!("Il valore di x è: {}", x);
				
					Rust non ci permetterà di usare un valore che non è stato inizializzato.
			INTERPOLAZIONE STRINGA
					@interpolazione_(la @interporabilita della stringa è l'inserimento
						di una stringa all'interno di un'altra stringa,)
					@{}_(le @grasse sono dei @segnaposto per dire dopo la virgola interpoliamo il valore)

				println!("Il valore di x è: {}", x); utilizza l'interpolazione stringa ossia l'inserimento di una stringa all'interno di un'altra stringa al posto di altrettanti segnaposto. Infatti le {} sono dei segnaposto per cui viene detto a rust dopo la stringa tipo :
					"Il valore di x è: {}",  = indichiamo a rust che dopo la virgola (,)
					interpoliamo il valore di x e la virgola serve a separare gli argomenti che passiamo alle funzioni e alle macro.


			AMBITO ED OSCURAMENTO
				I legami di variabile hanno un ambito - ossia sono vincolati a risiedere nel blocco in cui sono stati definiti. 
				Un blocco è una collezione di istruzioni racchiuse da { e }. Anche le definizioni di funzione sono blocchi! 

				Nell'esempio seguente definiamo due legami di variabile, x e y, che risiedono in blocchi diversi. Si può accedere a x da tutto il blocco fn main() {}, mentre si può accedere a y solamente dal blocco più interno:

						fn main() { //I° BLOCCO GRAFFE
						    let x: i32 = 17;

						    //II° BLOCCO GRAFFE SEPARATO
						    {
						        let y: i32 = 3;
						        println!("Il valore di x è {} e il valore di y è {}", x, y);
						    }

						    //non puoi stampare x
						    println!("Il valore di x è {} e il valore di y è {}", x, y); // Questo non funziona Y non accessibile
						}
				La prima println! stamperebbe "Il valore di x è 17 e il valore di y è 3", ma questo esempio non può essere compilato con successo, perché la seconda println! non può accedere al valore di y, dato che non è più nel suo ambito. Otteniamo invece questo errore:




				OSCURAMENTO - SHADOWED
				 	Inoltre, i legami di variabile possono venire oscurati ("shadowed"). Ciò significa che un successivo legame di variabile con il medesimo nome di un legame attualmente nel suo ambito scavalcherà il legame precedente.

					let x: i32 = 8;
					{
					    println!("{}", x); // Stampa "8"
					    let x = 12; 	   //OSCURA LA X =8 
					    println!("{}", x); // Stampa "12"
					}
					println!("{}", x); // Stampa "8"
					let x =  42;       //OSCURA LA X =8 
					println!("{}", x); // Stampa "42"

					ATTENZIONE l'ocsuramento lega un nome ad un valore diverso ed è ammissibile il mut (es. x = 5 poi shadowe = x="pippo" è un testo non un numero)
						@oscuramento_(è possibile il mutamento? si NON VIENE DISTRUTO MA NON E' ACCESSIBILE)
					l'oscuramento non alter o distrugge il valore ma continua ad esistere
					finche non esce dall'ambito MA NON E' ACCESSIBILE.
						ES.
							let mut x: i32 = 1;
							x = 7;
							let x = x; // x adesso è immutabile ed è legato a 7

							let y = 4;
							let y = "Posso anche essere legato a un testo!"; // y adesso è di un altro tipo

	FUNZIONI 											4.2
		MAIN
					@funzioni_(nelle funzioni i @parametri devono essere dichiarati )

			Ogni programma Rust ha almeno una funzione, la funzione main:

				fn main() {
				}

			NO ARGOMENTI
				Questo è la dichiarazione di funzione più semplice possibile. Come accennato prima, fn indica che ‘questa è una funzione’, ed è seguita dal nome della funzione, da due parentesi vuote perché questa funzione non prende argomenti, e poi da parentesi graffe che contengono il corpo della funzione. Ecco una funzione chiamata foo:

					fn foo() {
					}

			PARAMETRI
				fn main() {
						//chiamo la funzione e passo il parametro
					    stampa_numero(5);
					}

					//creato fuori main la funzione con 1 parametro
					fn stampa_numero(x: i32) {
					    println!("x is: {}", x);
					}

			NOME + TIPO ARGOMENTO
				Note
					Come si vede, gli argomenti delle funzioni funzionano in modo molto simile alle dichiarazioni let: si aggiunge un tipo al nome dell'argomento, dopo i due punti :.

					Ecco un programma completo che somma due numeri e stampa il risultato:

						fn main() {
						    stampa_somma(5, 6); //Virgola quando chiamo la funzione
						}
						
						//			la funzione è esterna a main
						//--------------------------------------------------------------//
						//x: i32 = NOME+TIPO ..., passo 2 argomenti numerici x,y
						fn stampa_somma(x: i32, y: i32) { //virgola quando dichiaro la funzione
													   //	
						    println!("la somma è: {}", x + y);
						}

				LA VIRGOLA
					Si separano gli argomenti usando una virgola, sia quando si chiama la funzione, che quando la si dichiara.

				OBBLIGATORIA LA DICHIARAZIONE DEI TIPI
					Diversamente dall'istruzione let, i tipi degli argomenti delle funzioni devono essere dichiarati. Pertanto questo non funziona:

							fn stampa_somma(x, y) {
							    println!("la somma è: {}", x + y);
							}
							Si ottiene l'errore:
								previsto 1 tra  ! o @ ..
								expected one of `!`, `:`, or `@`, found `)`
									fn print_sum(x, y) {

				INFERENZA
							@inferenza_(determinazione del tipo di dato non dichiarato)

						E' una attivita in cui il compilatore  usa l'inferenza dei tipi  per determinare i tipi di dati delle variabili locali dichiarate senza un Tipo. Il compilatore inferisce il tipo della variabile dal tipo dell'espressione di inizializzazione. Ciò consente di dichiarare le variabili senza specificare in modo esplicito un tipo, come illustrato nell'esempio seguente. Come risultato delle dichiarazioni, entrambe num1 e num2 sono fortemente tipizzata come interi.
							   ' //esempio IN VB
								    Dim num1 As Integer = 3

								    ' Using local type inference.
								    Dim num2 = 3  //inferenza il compilatore comprende che è un intero

								//il vb all'interno della funzione @INFERENZA    
								Public Sub inferenceExample()

								    ' Using explicit typing. - tipo dichiarato
								    Dim num1 As Integer = 3

								    ' Using local type inference. 
								    Dim num2 = 3  //imposta come intero simile al tipo dichiarato perchè all'interno della funzione

								End Sub

					    In Rust L'INFERENZA è applicata all'interno del { corpo delle
					    	funzioni }, mentre è OBBLIGATORIA LA DICHIARAZIONE DEI TIPI
					    	nella DEFINIZIONE DELLE FUNZIONI.

			    VALORE DI RITORNO DELLA FUNZIONE
			    			@funzioni_(il valore di @ritorno con @->)
			    		Le funzioni di Rust restituiscono esattamente un valore, e si dichiara il tipo dopo una ‘freccia’, che è un trattino (-) seguito da un segno di maggiore (>). L'ultima riga di una funzione determina che cosa restituisce. Qui si noterà la mancanza di un punto-e-virgola. Se l'avessimo aggiunto:

									fn somma_uno(x: i32) -> i32 {
									    x + 1 // senza il; permette il valore di ritorno altrimenti errore
									}

				ESPRESSIONI CONTRO ISTRUZIONI
						@istruzioni_(in rust son di due tipi istruzioni quelle che restiscono un valore e quelle no. )

						@istruzioni@espressioni_(in rust restituiscono una tupla)

				ESPRESSIONI ED ISTRUZIONI DIFFERENZA

					Rust è primariamente un linguaggio basato sulle espressioni. Ci sono solamente due tipi di istruzioni, e ogni altra cosa è un'espressione.

					la differenza è che:
						espressioni = restiscono un valore
						istruzioni	= non restituiscono valori
							l'istruzione x + 1; non restituisce un valore. 

				ISTRUZIONI
					ci sono due tipi di istruzioni:

						ISTRUZIONE DI DICHIARAZIONE = con Let 
						ISTRUZIONE DI ESPRESSIONE 	= trasforma qualsiasi istruzione in 
													una espressione; significa che rust si aspetta da ogni istruzione segua un'altra istruzione.
				ISTRUZIONE DI DICHIARAZIONE

					in altri linguaggi esempio in Ruby pui creare espressioni e fare il 
					legame delle variabili es.
						 x = y =5;  //Ruby si puo' creare una espressione

				    In Rust in non è ammesso il legame delle variabile con una espressione es:
				    	let x = (let y = 5); // atteso un identificatore, trovata la parola-chiave `let` - produce errore.

			    	Ma rust utilizza let per attivare una istruzione e quindi non si puo assegnare come negli altri linguaggi con let una espressione, perche a differenza degli altri linguaggi con con l'istruzione:
			    		x = y = 5
			    		x ha 5 assegnato da y
		    		nel caso di rust
		    			let x = (let y = 5); // da errore in quanto  x non ha il valore
		    			di y  perche in:
		    				Rust = si ha un solo possessore  e quindi 5 non puo essere
		    				assegnato sia a y e sia ad x quindi in questo caso viene dato un errore.
    				nel caso invece di una istruzione di dichiarazione del tipo
    					let mut y = 5
    					let x = (y=6);
    					    x = ha valore di tupla vuota non di 6 mentre y contiene il 6.

			    ISTRUZIONE DI ESPRESSIONE
			    	le istruzioni di espressione il II tipo di istruzione, servono a Rust per trasformare QUALUNQUE ESPRESSIONE IN UNA ISTRUZIONE.
			    	Perchè rust si aspetta che ad ogni istruzione segua un'altra istruzione sempre separata da un ;
			    	
			    	UNICA ECCEZIONE AL ; è quando costruiamo una funzione che restituisce
			    	un valore nel cui corpo viene omessa il punto e virgola (;) perche è
			    	un valore di ritorno es.
			    		fn somma_uno (x: i32) {
		    				x+1  //senza ; restituisce il valore incrementato
			    		}

		    				.
    				e quindi in rust L'ISTRUZIONE DI ESPRESSIONE è sempre una istruzione di assegnamento e non di espressione in quanto anche un codice di questo tipo:
    					let mut y = 5;  //creo y mutubile

						let x = (y = 6);  // x ha valore `()`, non `6`
    				 
    				 let x è un tupla vuota ma non 6 non è un risutato di una espression
    				 di y che assegna a x, MA UNA ISTRUZIONE CHE ASSEGNA A X UNA TUPLA VUOTA.
    				 mentre y è uguale a 6.

    				 conclusione in rust ci sono istruzioni di assegnazioni o istruzioni di espressione, mentre tutto il resto sono espressioni.


				USCITE PRECOCI
					con la parola chiave return
						fn foo(x: i32) -> i32 {
						    return x;

						    // non si eseguirà mai questo codice!
						    x + 1
						}

					oppure con stile elegante:
						fn foo(x: i32) -> i32 {
								    return x + 1;
								}

			FUNZIONI DIVERGENTI

				sono le funzioni che non restituiscono mai il controllo al chiamante:

					fn diverge() -> ! {
					    panic!("Questa funzione non restituisce mai il controllo al chiamante!");
					}

					panic! è una macro, come lo è println!() che abbiamo già visto. Diversamente da println!(), panic!() manda in crash il thread corrente, stampando il messaggio ricevuto come argomento. Dato che questa funzione provocherà un crash, non restituirà mai il controllo al chiamante, e quindi ha il tipo ‘!’, che si legge ‘diverge’


				Se si aggiunge una funzione main che chiama diverge() e la si esegue, si otterrà un output simile a questo:
					thread ‘main’ panicked at ‘Questa funzione non restituisce mai il controllo al chiamante!’, main.rs:2

				Una funzione divergente può essere usata dove ci si aspetta un'espressione di qualunque tipo:

						let x: i32 = diverge();
						let x: String = diverge();
			
			PUNTATORI DI FUNZIONE
					@puntatori_(a @funzione con @interferenza o senza)

				Possiamo anche creare legami di variabili che puntano a funzioni:
					
					let f: fn(i32) -> i32;

				f è un legame di variabile che punta a una funzione che prende un i32 come argomento e restituisce un i32. Per esempio:
					
					fn piu_uno(i: i32) -> i32 {
						    i + 1
						}

						// senza l'inferenza di tipo
						let f: fn(i32) -> i32 = piu_uno;

						// con l'inferenza di tipo
						let f = piu_uno;
							
				Poi possiamo usare f per chiamare la funzione:
					let sei = f(5);    //variabile che punta alla funzione f e la chiama 					e poi assegna il risultato a sei.

	TIPI PRIMITIVI 										4.3
		Note
				Il linguaggio Rust ha vari tipi che sono considerati ‘primitivi’. Ciò significa che fanno parte del linguaggio. Rust è strutturato in modo tale che la libreria standard fornisca anche vari altri tipi utili, costruiti basandosi su quelli primitivi, ma quelli che vedremo sono i più primitivi.

		Booleani 							4.3.1
			Rust ha un tipo booleano primitivo, chiamato bool. Ha solo due valori, true ("vero") e false ("falso"):

				let x = true;

				let y: bool = false;
				Nella documentazione della libreria standard si trova ulteriore documentazione sui bool.


				I booleani sono usati tipicamente nei costrutti if.
		char 								4.3.2
			Il tipo char rappresenta un singolo valore scalare Unicode. Si possono creare dei char racchiudendoli tra apici singoli: (')
					let x = 'x';
					let two_hearts = '💕';
					Diversamente da alcuni altri linguaggi, ciò significa che il char di Rust non è rappresentato con un singolo byte, ma da quattro byte.

		Tipi numerici						4.3.3

			Note
				Rust ha parecchi tipi numerici, appartenenti alle seguenti categorie: con segno e senza segno, fissi e variabili, a virgola mobile e interi.

				Questi tipi consistono di due parti: la categoria, e la dimensione. Per esempio, u16 è un tipo senza segno con una dimensione di sedici bit. Più bit consentono di rappresentare numeri più grandi.

				Se un letterale numerico non specifica il tipo esatto a cui appartiene, il suo tipo viene inferito nel seguente modo:

				Ecco una lista dei diversi tipi numerici, con dei link alla loro documentazione nella libreria standard:

					i8
					i16
					i32
					i64
					u8
					u16
					u32
					u64
					isize
					usize
					f32
					f64
					Esaminiamoli in base alla loro categoria:

		Con segno e senza segno 			4.3.4
			Ci sono due categorie di tipi interi: 
				- con segno 	= da -8 a + 7
				e 
				- senza segno 	= 0 a +15 (no rappresenta i negativi)
			Per comprendere la differenza, consideriamo un numero di quattro bit. 

			Un numero di quattro bit, con segno, consentirebbe di rappresentare i numeri da -8 a +7. 
			I numeri con segno usano la "rappresentazione in complemento a due". 

			Un numero di quattro bit, senza segno, dato che non ha bisogno di rappresentare valori negativi, può rappresentare valori da 0 a +15.

		Tipi a dimensione fissa 			4.3.5
			I tipi dimensione fissa hanno uno specifico numero di bit nella loro rappresentazione. Le dimensioni in bit valide sono :
				8, 16, 32, e 64. 
			Perciò, u32 è un intero senza segno, a 32 bit, e i64 è un intero con segno, a 64 bit.

		Tipi a dimensione variabile 		4.3.6
				
				@usize_{dimensione dipende dall'architettura}

			Rust fornisce anche dei tipi la cui effettiva dimensione dipende dall'architettura della macchina in questione. L'ampiezza di questi tipi è sufficiente ad esprimere la dimensione di qualunque collezione, perciò questi tipi appartengono alla categoria ‘size’ ('dimensione'). Anche loro hanno la versione con segno e quella senza segno, e quindi sono due: isize e usize.

		Tipi a virgola mobile 				4.3.7
				@float_(numeri a @virgola @mobile con @f32 e @f64 sono a @precisione @singola o @doppia)

			Rust ha anche due tipi a virgola mobile: f32 e f64. Questi corrispondono rispettivamente ai numeri a precisione singola e a precisione doppia secondo lo standard IEEE-754.

		Arrays 								4.3.8
				@array_(@array@fisso o schiera per seguenze di dati o lista  a lunghezza fissa, si distingue dal vettore perche esso è dinamico mentre il semplice @array è fisso = alloca nello stack)

			Come molti linguaggi di programmazione, Rust ha dei tipi compositi per rappresentare sequenze di dati. Il più basilare è il tipo array ("schiera"), una lista a lunghezza fissa di elementi dello stesso tipo. Di default, gli array sono @immutabili.
				let a = [1, 2, 3]; // a: [i32; 3]
				let mut m = [1, 2, 3]; // m: [i32; 3]

			Gli arrays hanno tipo [T; N]. 
			Parleremo di questa notazione T nella [sezione sulla genericità][generics]. La N è una costante nota in fase di compilazione, che indica il numero di oggetti contenuto nell'array.

			C'è un'abbreviazione per inizializzare ogni elemento di un array allo stesso valore. Ecco come inizializzare a 0 ognuno dei 20 elementi dell'array a:

				let a = [0; 20]; // a: [i32; 20] attenzione con il ; resetta tutti a 20 a 
												 zero

			Si può ottenere il numero di elementi di un array a con l'espressione a.len(): 
				let a = [1, 2, 3];

				println!("a ha {} elementi", a.len());

			Si può accedere a un particolare elemento di un array con la notazione a indice:
					@accesso_(@elemento di un @array con l)

				let nomi = ["Graydon", "Brian", "Niko"]; // nomi: [&str; 3]

				println!("Il secondo nome è: {}", nomi[1]);

			Gli indici partono da zero, come nella maggior parte dei linguaggi di programmazione, e perciò il primo nome è nomi[0] e il secondo nome è nomi[1]. L'esempio precedente stampa Il secondo nome è: Brian. Provando ad usare un indice non compreso nell'array, si ottiene un errore, perché per ogni accesso ad un array, in fase di esecuzione si verifica che l'indice sia compreso nei limiti. Accessi erronei di questo tipo causano molti bug in altri linguaggi di programmazione di sistema.

		Slices ("@Fette")					4.3.9

				@slice_(sono viste dentro una struttura come una parte di array ed un putnatore all'inizio dei dati)

			Le ‘slices’ (pronunciato "slaises") sono riferimenti a (o “viste" dentro) un'altra struttura dati. Servono a consentire un accesso sicuro ed efficiente ad una porzione di un array senza fare copie. Per esempio, si potrebbe voler far riferimento solamente ad una riga di un file letto in memoria. Per sua natura, una slice non viene creata direttamente, ma partendo da un legame di variabile esistente. Le slices hanno una lunghezza fissa, e il loro contenuto può essere mutabile o immutabile.

			Internamente, le slice sono rappresentate come un @puntatore all'inizio dei dati e una lunghezza.



			Sintassi delle @slices			4.4.9.1

				Per creare una slice da vari oggetti si può usare la combinazione del carattere @& e della coppia di caratteri @[]. Il carattere & indica che le slices sono simili ai [riferimenti], che tratteremo in dettaglio più avanti in questa sezione. La coppia di caratteri [], utilizzata con un range, permette di definire la lunghezza della slice:

					let a = [0, 1, 2, 3, 4];
					let completo = &a[..]; // Una slice contenente tutti gli elementi di a
					let mezzo = &a[1..4]; // Una slice contenente solo gli elementi 1, 2, e 3

				Le slice sono di tipo &[T]. Parleremo di quella T quando tratteremo la genericità.??


			str 							4.4.9.2
				Il tipo @str di Rust è il tipo di stringa più primitivo. Come tipo unsized, non è molto utile di suo, ma diventa utile quando è utilizzato come riferimento, come per esempio &str. Tratteremo l'argomento in maniera più approfondita quando tratteremo le Stringhe e i riferimenti.

		Ennuple 							4.3.10
			definizione
				Una ennupla è una lista ordinata di lunghezza fissa. Come questa:
					let x = (1, "ciao");
						     //lista ordinara 1, ciao nelle ennuple conta l'ordine e
						     possono essere eterogenee


					Le parentesi e le virgole formano questa ennupla di lunghezza due. Ecco lo stesso codice, ma con il tipo annotato:

						let x: (i32, &str) = (1, "ciao");

					Come si vede, il tipo di una ennupla somiglia all'ennupla, ma in ogni posizione c'è il tipo invece del valore. I lettori attenti noteranno anche che le ENNUPLE SONO ETEROGENEE: in questa ennupla c'è un i32 e un &str. Nei linguaggi di programmazione di sistema, le stringhe sono un pochino più complesse che negli altri linguaggi. Per adesso, si legga &str come slice di stringa, presto impareremo di più a riguardo.

					BIN FRA ENNUPLE
						È possibile assegnare un bind su una ennupla ad un'altra ennupla, se entrambe le ennuple contengono gli stessi tipi di dato e hanno la stessa arity (STESSO NUMERO DI ARGOMENTI E STESSA LUNGHEZZA). Le tuple hanno la stessa arity quando hanno la stessa lunghezza.

								//Bind tra ennuple con gli stessi argomenti e stessa lunghezza
								let mut x = (1, 2); // x: (i32, i32)
								let y = (2, 3); // y: (i32, i32)

						Si può accedere ai campi di una ennupla usando un let destrutturante. Ecco un esempio:

							//pattern dx = sx assegni piu legami
							let (x, y, z) = (1, 2, 3);

							println!("x is {}", x);

								x = y;

						Questo pattern è molto efficace, e lo ritroveremo ripetuto in seguito.

						Per discriminare una ennupla con un solo elemento da un valore tra parentesi, basta usare una virgola:

						(0,); // ennupla con un solo elemento
						(0); // zero tra parentesi

			Indicizzazione delle ennuple 	4.4.10.1
					I campi di una ennupla possono essere acceduti anche con la sintassi di indicizzazione:

						let ennupla = (1, 2, 3);

						let x = ennupla.0;
						let y = ennupla.1;
						let z = ennupla.2;

						println!("x contiene {}", x);

						@differenza@indicizazione_(nelle ennuple l'indicizzazione si fa con il . negli array con le [])
					L'indicizzazione, come per gli di array, parte da zero, ma diversamente dall'indicizzazione di array, usa un carattere ., invece della coppia di caratteri [].
		Funzioni
						@puntatore@a@funzione_(puntatore a funzione che restituisce un valore )
					Anche le funzioni hanno un tipo! Ecco un esempio:
						//creo la funzione foo
						fn foo(x: i32) -> i32 { x }

						//let x: fn(i32) = crea un puntatore a funzione che restituisce un i32
						let x: fn(i32) -> i32 = foo;

					In questo caso, x è un ‘puntatore a funzione’ che punta a una funzione che prende un i32 e restituisce un i32.

	COMMENTI 											4.4
		Note

				@commenti_(come annotazione e come @documentazione)

			I commenti sono annotazioni che si lasciano per gli altri programmatori, per aiutarli a spiegare il proprio codice. Il compilatore per lo più li ignora.

			Rust ha due tipi di commenti a cui si dovrebbe essere interessati: i commenti di riga e i commenti di documentazione ["doc comment"].
			// I commenti di riga sono i caratteri tra la coppia di caratteri ‘//’ e la fine della riga.


			let x = 5; // anche questo è un commento di riga

			// Se si ha una lunga spiegazione da scrivere, si possono mettere più
			// commenti di riga, uno dopo l'altro. Mettere uno spazio tra // e il testo
			// rende più leggibile il commento.

			L'altro genere di commenti è il commento di documentazione. I commenti di documentazione usano /// invece di //, e supportano la notazione Markdown al loro interno:
						/// Aggiunge uno al numero dato.
						///
						/// # Esempi
						///
						/// ```
						/// let cinque = 5;
						///
						/// assert_eq!(6, somma_uno(5));
						/// # fn somma_uno(x: i32) -> i32 {
						/// #     x + 1
						/// # }
						/// ```
						fn somma_uno(x: i32) -> i32 {
						x + 1
						}

			C'è un altro stile di commento di documentazione, //!, usato per commentare gli elementi (per es. crate, moduli, o funzioni) che contengono tali commenti, invece che per commentare gli elementi che li seguono. Sono usati tipicamente all'interno delle radici di crate (lib.rs) o delle radici di moduli (mod.rs):

				//! # The Rust Standard Library
				//!
				//! The Rust Standard Library provides the essential runtime
				//! functionality for building portable Rust software.

				Quando si scrivono commenti di documentazione, fornire degli esempi di utilizzo è di enorme aiuto. Si noterà che qui abbiamo usato una nuova macro: 
					@assert_eq!. 
				Questa macro confronta due valori, e va in panic! se non sono uguali tra di loro. È di grande aiuto nella documentazione. C'è un'altra macro, 
					assert!, 
				che va in panic! se il valore passatole vale false.

				Si può usare lo strumento @rustdoc per generare @documentazione @HTML da questi commenti di documentazione, e anche per eseguire gli esempi di codice come collaudo!

		CREARE LA LIBRERIA COMMENTI
			Attenzione per creare la libreria in html occorre la funzione sia pubblica,
				1) es. 
					//!main procedura prodotta con cargo.
					//! # The Rust Standard Library
					//!
					//! The Rust Standard Library provides the essential runtime
					//! functionality for building portable Rust software.

					pub fn main() {
						
						println!("hello word");
					}

				2) PER CREARE LA LIBRERIA DIGITARE
					cargo doc
						crea la libreria html in target/doc ... con html
					cargo doc --verbose = per visualizzarlo sul terminale

	IF 													4.5
		Nota
				@if_(la if e la @elseif )

			E semplice
			Nel caso dell'if, c'è un valore booleano che conduce a due strade:

				let x = 5;

				if x == 5 {
				    println!("x vale cinque!");
				}

			Se cambiamo il valore di x a qualcos'altro, questa riga non verrebbe stampata. Più specificamente, se l'espressione dopo l'if vale true, allora il blocco viene eseguito; se vale false, no.
			Se si vuole che accada qualcosa caso false, si usa una clausola else:

				let x = 5;

				if x == 5 {
				    println!("x vale cinque!");
				} else {
				    println!("x non vale cinque :(");
				}

			ELSEIF
				If c'è più di un caso, si usa un else if:

				let x = 5;

				if x == 5 {
				    println!("x vale cinque!");
				} else if x == 6 {
				    println!("x vale sei!");
				} else {
				    println!("x non vale né cinque né sei :(");
				}

				Ciò funziona perché l'if è un'espressione. Il valore di tale espressione è il valore dell'ultima espressione della diramazione scelta. Un if senza un else ha sempre il valore ().

	CICLI 												4.6.
		LOOP 								4.6.1
			Il costrutto @loop ("ciclo") è la forma più semplice di ciclo disponibile in Rust. Usando la parola-chiave loop, Rust fornisce un modo di ciclare indefinitamente finché si raggiunge qualche istruzione di terminazione. Il @ciclo @infinito di Rust è fatto così:
				//Loop è il costrutto adatto per i cicli infinti
				loop {
				    println!("Cicla per sempre!");
				}

		WHILE								4.6.2
			Rust ha anche un @ciclo @while ("fintanto che"). È fatto così:

				let mut x = 5; // mut x: i32
				let mut fatto = false; // mut fatto: bool

				while !fatto {
				    x += x - 3;

				    println!("{}", x);

				    if x % 5 == 0 {
				        fatto = true;
				    }
				}

			I cicli while sono la scelta appropriata quando non si è sicuri di quante volte si dovrà ciclare.

			Se serve un ciclo infinito, si può essere tentati di scrivere:

				while true {Run
				Tuttavia, il costrutto loop è molto più adatto per gestire questo caso:

				loop {


			L'analisi del flusso di costrutto di Rust tratta questo costrutto diversamente da un while true, dato che sappiamo che ciclerà per sempre. In generale, più informazione possiamo dare al compilatore, meglio può fare con la sicurezza e la generazione del codice, e perciò si dovrebbe sempre preferire loop quando si intende ciclare indefinitamente.

		FOR 								4.6.3
			Il ciclo for viene usato per ciclare un particolare numero di volte. Però i cicli for di Rust funzionano un po' diversamente dagli altri linguaggi di sistema. Il ciclo for di Rust non somiglia al ciclo for del linguaggio C:
				//ciclo for in c
				for (x = 0; x < 10; x++) {
					    printf( "%d\n", x );
					}

			Invece, è fatto così:
				//@ciclo@for_(in rut con 0..10 con posizioni 0 fino a 9 l'ultimo escluso)
				
				for x in 0..10 {
				    println!("{}", x); // x: i32
				}

				o, in termini più astratti,

						for var in expression {
						    code
						}
				L'espressione è un elemento che può essere convertito in un [iteratore] usando IntoIterator. L'iteratore rende una serie di elementi. Ogni elemento è un'iterazione del ciclo. Tale valore viene poi associato al nome var, che è valido solo nel corpo del ciclo. Una volta che il corpo è finito, il prossimo valore viene preso dall'iteratore, e si esegue un'altra iterazione. Quando non ci sono più valori, il ciclo for è finito.

			POSIZIONI INIZIO E FINE: 
				Nel nostro esempio, 0..10 è un'espressione che prende una posizione di inizio e una di fine, e dà un iteratore su quei valori. Tuttavia, il limite superiore è escluso, così che questo ciclo stamperà i numeri da 0 a 9, e non il 10.

				Rust non ha il ciclo for in "stile C" di proposito. Controllare manualmente ogni elemento del ciclo è complicato e soggetto a errori, anche per sviluppatori esperti nel linguaggio C.


			Enumerazione
					@for@controllo@ciclo_(per tener traccia dei @cicli usare @enumerate)
				Quando c'è bisogno di tener traccia di quante volte si ha già ciclato, si può usare la funzione .enumerate().

				for (i, j) in (5..10).enumerate() { //for con enumerate per contare i cicli
					    println!("i = {} e j = {}", i, j);
					}
					Emette:

					i = 0 e j = 5
					i = 1 e j = 6
					i = 2 e j = 7
					i = 3 e j = 8
					i = 4 e j = 9
					In questo caso si devono aggiungere le parentesi intorno al range.

			Sugli @iteratori:


				//for su due linee di una stringa
				let linee = "ciao\nmondo".lines();

					for (numerolinea, linea) in linee.enumerate() {
					    println!("{}: {}", numerolinea, linea);
					}
					Emette:

					0: ciao
					1: mondo

			Terminare precocemente l'iterazione
					Diamo un'occhiata a quel ciclo while di prima:

						let mut x = 5;
						let mut fatto = false;

						while !fatto {
						    x += x - 3;

						    println!("{}", x);

						    if x % 5 == 0 {
						        fatto = true;
						    }
						}

					Abbiamo dovuto tenere apposta una variabile booleana, fatto, per sapere quando dovremmo uscire dal ciclo. Rust ha due parole-chiave per aiutarci a modificare le iterazioni: break ("interrompi") e continue ("continua").


					In questo caso, possiamo scrivere il ciclo in un modo migliore usando break:

						let mut x = 5;

						loop {//ciclo @loop@infinito_(con interuzione con break)
						    x += x - 3;

						    println!("{}", x);

						    if x % 5 == 0 { break; }
						}

					Adesso cicliamo per sempre usando loop e usiamo break per uscire precocemente. Anche eseguire un'istruzione return potrebbe servire a terminare il ciclo precocemente.

					continue è simile, ma invece di terminare il ciclo, passa alla prossima iterazione. Questo codice stamperà solamente i numeri dispari:

						for x in 0..10 {
							//stampa solo i dispari
						    if x % 2 == 0 { continue; }

						    println!("{}", x);
						}

			Etichette dei cicli
							@etichetta_(per i @cicli@esterni )

					Si potrebbero anche incontrare situazioni in cui ci sono @cicli @annidati si vuole specificare a quale ciclo si riferisce una particolare istruzione @break o @continue. Come nella maggior parte degli altri linguaggi, di default an'istruzione break o continue si applicheranno al ciclo più @interno. Dove si volesse applicare break o continue a uno dei @cicli @esterni, si possono usare delle etichette. Il seguente codice stamperà solamente quando sia x che y sono dispari:

						'esterno: for x in 0..10 {
						    'interno: for y in 0..10 {
						        if x % 2 == 0 { continue 'esterno; } // continua al ciclo su x
						        if y % 2 == 0 { continue 'interno; } // continua al ciclo su y
						        println!("x: {}, y: {}", x, y);
						    }

    VETTORI 											4.7
    	Definizione							4.7.1
    			@vettori_(il @vettore è un @array@dinamico ed estensibile con il tipo @Vec<T>)

    		Un ‘vettore’ è un @array @dinamico ossia ‘estendibile’, implementato dal tipo Vec<T> nella libreria standard. Il T significa che si possono avere vettori di ogni tipo (si veda il capitolo su [generici][generic] per avere maggiori informazioni). I VETTORI ALLOCANO SEMPRE I LORO DATI NELLO @HEAP. Possono essere creati con la macro vec!:
    			let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32> !=macro che usa [] ma anche ()

    			(Si noti che diversamente dalla macro println! che abbiamo usato in passato, con la macro vec! usiamo le parentesi quadre []. Rust permette di usare entrambi i tipi di parentesi in entrambe le situazioni, questo uso è solo una convenzione.)

			C'è una forma alternativa di vec! per ripetere un valore iniziale:
				let v = vec![0; 10]; // dieci zeri

			I vettori immagazzinano il loro contenuto sullo heap come array contigui di T. Ciò significa che devono essere capaci di sapere la dimensione di T in fase di compilazione (cioè, quanti byte servono per memorizzare un T?). La dimensione di alcuni oggetti non si può sapere in fase di compilazione. Per tali oggetti si dovrà immagazzinare un puntatore a quell'oggetto: fortunatamente, il tipo Box funziona perfettamente a questo scopo.

		Accedere agli elementi				4.7.2
			Per ottenere il valore a un particolare indice nel vettore, si usano le []:
				let v = vec![1, 2, 3, 4, 5];

				println!("Il terzo elemento di v è {}", v[2]);

			Gli indici contano da 0, e perciò il terzo elemento è v[2].

			È anche importante notare che si deve indicizzare con il tipo usize:

				let v = vec![1, 2, 3, 4, 5];

				let i: usize = 0;	//il vettore si deve indicizzare come tipo usize
				let j: i32 = 0;		

				// funziona
				v[i];

				// non funziona da errore non puoi indicizzarlo come i32 ma deve essere usize
				v[j];

		Accesso fuori dai limiti			4.7.3
			Se si prova ad accedere un indice che non esiste:
				let v = vec![1, 2, 3];
				println!("L'elemento 7 è {}", v[7]);

			allora il thread attuale andrà in [panico] con un messaggio come questo:
				thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 7'


			Se si vuole gestire gli errori di accesso fuori dai limiti senza andare in panico, si possono usare metodi come get o get_mut, che restituiscono None quando gli viene dato un indice invalido:

				let v = vec![1, 2, 3];
					match v.get(7) {  //gestione fuori indice con il @match
					    Some(x) => println!("Item 7 is {}", x),
					    None    => println!("Spiacente, questo vettore è troppo corto.")
					}
		
		Iterare 							4.7.4
			Una volta che si ha un vettore, si può iterare sui suoi elementi usando for. Ce ne sono tre versioni:

				let mut v = vec![1, 2, 3, 4, 5];

					for i in &v {
					    println!("Un riferimento a {}", i);
					}

					for i in &mut v {
					    println!("Un riferimento mutabile a {}", i);
					}

					for i in v {
					    println!("Prendi il possesso del vettore e del suo elemento {}", i);

		    CON RIFERIMENTO & PUOI ITERARE PIU VOLTE IL VETTORE 
		    	Nota: Non si può usare ancora il vettore dopo averlo iterato prendendone il possesso. Invece, si può iterare il vettore più volte se quando lo si itera se ne prende un riferimento. Per esempio, il seguente codice non compila.


		    	let v = vec![1, 2, 3, 4, 5];

					//I° ITERAZIONE con presa in possesso
					for i in v {
					    println!("Prendi possesso del vettore e del suo elemento {}", i);
					}

					//ERRORE = la II° iterazione con presa di posse non compila occorre il riferimento &
					for i in v {
					    println!("Prendi possesso del vettore e del suo elemento {}", i);
					}
				
				Mentre il seguente funziona perfettamente:

					let v = vec![1, 2, 3, 4, 5];

					//I iterazione con riferimento
					for i in &v {
					    println!("Questo è un riferimento a {}", i);
					}

					//II iterazione con riferimento
					for i in &v {
					    println!("Questo è un riferimento a {}", i);
					}

	POSSESSO 											4.8
		Note
			Questa è la prima delle tre sezioni che presentano il sistema di possesso di Rust. Questa è una delle caratteristiche più distintive e avvincenti di Rust, con la quale gli sviluppatori Rust dovrebbero diventare familiari. Il possesso è il modo in cui Rust raggiunge il suo maggior obiettivo, la sicurezza di accesso alla memoria. Ci sono alcuni concetti distinti, ognuno descritto in una sezione distinta:

			il possesso, che è la sezione attuale
			i prestiti, e le caratteristiche a loro associate, i ‘riferimenti’
			i tempi di vita, un avanzato concetto di prestito
			Queste tre sezioni sono correlate, e seguono un ordine. Bisognerà leggerli tutti e tre per capire pienamente il sistema di possesso.

		
		Meta								4.8.1
			Rust ha un'attenzione particolare sulla sicurezza e sulla velocità. Raggiunge questi obiettivi tramite molte ‘ASTRAZIONI A COSTO ZERO’, il che significa che in Rust, le astrazioni costano il meno possibile al fine di farle funzionare. Il sistema di possesso è un esempio primario di astrazione a costo zero. Tutta l'analisi di cui parleremo in questa guida viene fatta in fase di compilazione. Non si paga nessun costo in fase di esecuzione per queste funzionalità.
			Il costo sono i tempi di apprendimento in quanto si combatte con il verificatore di possesso perché il modello mentale del programmatore di come il possesso dovrebbe funzionare non combacia con le regole effettivamente implementate da Rust ma col tempo si migliora.


		Possesso							4.8.2
			I legami di variabili hanno una proprietà in Rust: ‘possiedono’ quello a cui sono legati. Ciò significa che quando un legame esce di ambito, Rust libererà le risorse legate. Per esempio:

				fn foo() {
				    let v = vec![1, 2, 3];  //vettore che viene messo sull'heap gli elementi + nello stack il vettore

				}

			Quando v viene nell'ambito, 
				viene creato un nuovo vettore sullo STACK, 
				e alloca spazio sullo HEAP per i suoi elementi. 
			Quando v esce di ambito alla fine di foo(), Rust ripulirà ogni cosa correlata al vettore, anche la memoria allocata sullo heap. Questo avviene deterministicamente alla fine dell'ambito.

		Semantica di spostamento			4.8.3
			Però qui c'è qualche 
				altra sottigliezza: 
					RUST ASSICURA CHE CI SIA ESATTAMENTE UN LEGAME A OGNI DATA RISORSA. Per esempio, se abbiamo un vettore, possiamo assegnarlo a un altro legame:

						let v = vec![1, 2, 3];
						let v2 = v;  //vettore v assegnato a v2 ma v perde il legame
					
					Ma, se dopo proviamo a usare v, otteniamo un errore:
					L'errore si presenta così:
						let v = vec![1, 2, 3];
							let v2 = v;   //il legame v viene assegnato a v2
							println!("v[0] vale: {}", v[0]);  //ma v perde il legame

						error: use of moved value: `v`
						println!("v[0] vale: {}", v[0]);

			ANCHE LA FUNZIONE PERDE IL LEGAME DELL'ARGOMENTO
				Una cosa simile accade se definiamo una funzione che prende possesso dell'argomento, e proviamo a usare qualcosa dopo che l'abbiamo passato come argomento:
					//la funnzione prende il possesso dell'argomento
					fn prendi(v: Vec<i32>) {

					    // ciò che accade qui dentro non è importante.
					}

					let v = vec![1, 2, 3]; //Legame v che viene passato alla funzione 
											prendi
					prendi(v);  //passo il legame alla funzione

					println!("v[0] vale: {}", v[0]); //legame perso con la fine della
					 								funzione.

				Stesso errore: ‘use of moved value’. Quando si trasferisce il possesso di un oggetto da un legame a un altro, si dice che l'oggetto a cui si fa riferimento è stato ‘spostato’. Qui non ci vuole qualche sorta di annotazione speciale, è il normale comportamento di Rust.


		I dettagli							4.8.4
			Dopo lo spostamento dell'oggetto il legame non puo essere piu utilizzato. 
			Quando scriviamo del codice come questo:

				let x = 10;  //x = oggetto x va nello stack mentre per 10; il riferimento del valore 10 nell'heap

			Rust alloca sullo stack della memoria per un intero i32, copia i bit che rappresentano il valore 10 alla memoria allocata, e lega il nome della variabile x a questa regione di memoria per poterna riferire in seguito.

			Adesso consideriamo il seguente frammento di codice:
				//l'oggetto let v = memorizzato nello statck
				let v = vec![1, 2, 3];  // per i valori [1, 2, 3]; vie copiato il 
										   riferimento nell'heap al primo numero
				let mut v2 = v;

			Ma in aggiunta a ciò, alloca anche della memoria sullo heap per i dati effettivi ([1, 2, 3]). 
			Rust copia l'indirizzo di questa allocazione sullo heap al puntatore interno, che fa parte dell'oggetto vettore posto sullo stack (chiamiamolo "puntatore ai dati").

			VETTORE NELLO STACK - DATI NELL'HEAP = Vale la pena evidenziare (anche al rischio di affermare l'ovvio) che 
				l'oggetto vettore e i suoi dati 
			vivono in regioni di memoria separate, invece di essere un'unica allocazione di memoria contigua (a causa di ragioni che non approfondiremo in questo momento). Queste due parti del vettore (quella sullo stack e quella sullo heap) devono accordarsi l'un l'altra in ogni momento riguardo a cose come la lunghezza, la capacità, ecc.

			Quando si sposta v in v2, Rust effettivamente fa una copia bit-a-bit dell'oggetto vettore v nell'allocazione sullo stack rappresentata da v2. Questa copia superficiale non crea una copia dell'allocazione sullo heap contenente i dati effettivi. Il che significa che ci sarebbero due puntatori al contenuto del vettore entrambi che puntano alla stessa allocazione di memoria sullo heap. Se si potesse accedere sia a v che a v2 nello stesso tempo, si violerebbe la garanzia di sicurezza di Rust, introducendo un'accesso concorrente ai dati.

		I tipi Copy							4.8.5
			I tipi primativi tipo bool, i32 ecc utilizzano copy e quindi quando 
			assegni u i32 con 
				let v1=1;
				let v2=v1;
				println!("v vale: {}", v); //i32 non ha puntatori ai dati ma utilizza
											Copy e quindi si fa una copia integrale dei dati.
		Oltre al possesso					4.8.6
			al posto della restituzione attivo il prestito

	RIFERIMENTI E PRESTITI 								4.9
	
		Borrowing (@PRESTITO)				4.9.1 
			Prendere in prestito
				Alla fine della sezione sul possesso, avevamo una brutta funzione che si presentava così:

					fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
					    // fa' qualcosa con v1 e con v2

					    // restituisci il possesso di v1 e v2, e restituisci anche
					    // il risultato della nostra funzione
					    (v1, v2, 42)
					}

					let v1 = vec![1, 2, 3];
					let v2 = vec![1, 2, 3];
					let (v1, v2, risposta) = foo(v1, v2);


				Però questo non è tipico di Rust, dato che non sfrutta i prestiti. Ecco il primo passo:

						fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
						    // fa' qualcosa con v1 e con v2

						    // restituisci la risposta
						    42
						}

						let v1 = vec![1, 2, 3];
						let v2 = vec![1, 2, 3];

						let risposta = foo(&v1, &v2);

				Un esempio più concreto:

					fn main() {
						//vedi il codice qui: @esempio_(@prestito valori)
					    // Non importa se non si capisce cosa fa `fold`, quello che importa
					    // qui è che un riferimento immutabile viene preso in prestito.
					    fn somma_vec(v: &Vec<i32>) -> i32 {
					        return v.iter().fold(0, |a, &b| a + b);
					    }
					    // Prendi in prestito due vettori e sommane gli elementi.
					    // Questo tipo di prestito non permette che gli oggetti siano mutati.
					    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
					        // fa' qualcosa con v1 e con v2
					        let s1 = somma_vec(v1); //s1 e s2 non possono essere mutati
					        let s2 = somma_vec(v2);
					        // restituisci la risposta senza modifiche
					        s1 + s2
					    }

					    let v1 = vec![1, 2, 3];
					    let v2 = vec![4, 5, 6];

					    let risposta = foo(&v1, &v2);
					    println!("{}", risposta);
					}

					Invece di prendere dei Vec<i32> come argomenti, prendiamo dei riferimenti: &Vec<i32>. 
					E invece di passare v1 e v2 direttamente, passiamo &v1 e &v2. 

					Il tipo &T viene chiamato ‘riferimento’, e invece di possedere la risorsa, ne prende in prestito il possesso. 

					Un legame che prende in prestito qualche oggetto non dealloca quella risorsa quando esce dall'ambito. Ciò significa che dopo la chiamata a foo(), possiamo usare ancora i nostri legami originali.

					ATTENZIONE RIFERIMENTI IMMUTABILI
						I riferimenti sono immutabile, come i legami. Ciò significa che dentro foo(), i due vettori non possono affatto essere modificati:

							fn foo(v: &Vec<i32>) {
							     v.push(5);  //SE MODIFICHI IL VETTORE ERRORE PERCHE IL
							     			//PRESTITO E' IMMUTABILE
							}

							let v = vec![];

							foo(&v);
							ci darà questo errore:

							error: cannot borrow immutable borrowed content `*v` as mutable
							v.push(5);
							^
							Aggiungere un valore (chiamando push) muterebbe il vettore, e quindi non ci viene permesso.

		i riferimenti &mut 					4.9.2
				@come@modificare_(la risorsa presa in prestito)
			C'è un altro tipo di riferimenti: &mut T. Un ‘riferimento mutabile’ permette di mutare la risorsa che viene presa in prestito. Per esempio:
				let mut x = 5;    //x legame mut
					{
					    let y = &mut x; //y= riferimento mutabile a x perche &mut x
					    				//preso in prestito da x e così puo essere incrementato
					    // *y = asterico prima di y per accesso al riferimento sia  se
					    //      mutabile o non è mutabile
					    *y += 1;	//incrementi perchè marcato con &mut ..
					}
					println!("{}", x);  //stampa 6

			Abbiamo creato y come riferimento mutabile a x, e poi abbiamo incrementato l'oggetto a cui y punta. Si noterà che abbiamo dovuto marcare anche x come mut. Se non l'avessimo fatto, non avremmo potuto prendere in prestito mutabile un valore immutabile.

			PERCHE' * = ACCESSO AL RIFERIMENTO MUTABILE O NON MUTABILE
				Si noterà anche che abbiamo aggiunto un asterisco (*) prima di y, rendendolo *y. Questo è necessario perché y è un riferimento. SI deve usare un asterisco per accedere al contenuto di un riferimento, che sia mutabile o immutabile.

		le regole		 					4.9.2
			Ecco le regole per prendere a prestito in Rust:
				AMBITO DEL PRESTITO <= A QUELLO DEL POSSESSORE = Primo, ogni prestito deve durare per un ambito non più esteso di quello del possessore.
					@prestito_(in Rust deve rimanere nell'ambito per cui è valido
					e per fare un prestito occorre &T = prestito non mutabile)
				

				Secondo, si può avere uno o l'altro dei due seguenti generi di prestiti, ma non entrambi allo stesso tempo:
						@riferimenti_@immutabili_o_@mutabili_(per fare un prestito in rust occorre &T e se mutabile &mut T:
							 @&T= prestito immutabile; 
							 @&mut T =  prestito @mutabile )

				A) uno o più riferimenti non mutabili (&T) a un oggetto,
				B) esattamente un riferimento mutabile (&mut T) a un oggetto.

				CORSA AI DATI =  C'è una ‘@corsa @ai @dati’ @data@race quando due o più puntatori accedono alla medesima posizione di memoria nello stesso tempo, e per almeno uno di essi tale accesso è in scrittura, e tali accessi non sono sincronizzati.

				TANTI RIFERIMENTI MUTABILE SOLO UNO MUTABILE ALLA VOLTA = Per quanto riguarda i riferimenti immutabili, se ne possono avere quanti se ne vogliono, dato che nessuno di essi sta scrivendo. Però, dato possiamo avere solamente un riferimenti mutabili per volta, è impossibile avere una corsa ai dati. Questa tecnica consente a RUST IN FASE DI COMPILAZIONE DI PREVENIRE LE CORSE AI DATI: otterremmo degli errori se violiamo le regole.
				Tenendo questo a mente, consideriamo ancora il nostro esempio.

		Pensare secondo gli ambiti			4.9.3
			
			//il prestito mutabile valido e parte da x e finisce alla graffa finale,
			// l'ambita parte da --> { .... e finisce con ---> }
			fn main() {
					'//prestito mutabile parte qui'
				    let mut x = 5;		//&mut T = riferimento MUTABILE di x
				    let y = &mut x;     //&mut T = faccio il prestito mutabile di x
				    					// a y

				    *y += 1;			//prendo il riferimento con *Y = puntatore e lo
				    					// incremento
				    					//fino qui il prestito è mutabile e x non è piu
				    					// possessore

				    println!("{}", x);	//da errore il prestito è sempre nell'ambito di y
				    					//fino nell'ambito delle graffe e quindi mutabile e non è tornato a x come immutabile

				} //prestito mutabile valido fino a fine programma 

					Questo codice ci dà questo errore:

					error: cannot borrow `x` as immutable because it is also borrowed as mutable
					Errore: Impossibile prendere in prestito 'X' come immutabile perché è anche preso in prestito come mutabile

					    println!("{}", x);
					                   ^

                   Questo perché abbiamo violate le regole: abbiamo un &mut T (mutabile) che punta a x, e così non ci è permesso creare dei &T (immutabili) che puntino al medesimo oggetto. È l'uno o l'altro. L'annotazione suggerisce come pensare a questo problema:

						note: previous borrow ends here
						Nota: il prestito precedente termina qui
						fn main() {

						}
						^

					IL PRESTITO MUTABILE VIENE TENUTO PER TUTTO IL PROGRAMMA: In altre parole, il prestito mutabile viene tenuto per tutto il resto del programma. 

					Ciò che vogliamo è che il prestito mutabile a y finisca, così che la risorsa possa essere restituita al possessore, x. 

					Poi x può fornire un prestito immutabile a println!. 

					In Rust, prendere a prestito è legato all'ambito per cui il prestito è valido. E il nostro ambito si presenta così:


						fn main() {
						    let mut x = 5;     //rende mutabile x

						    let y = &mut x;    // -+ qui inizia il prestito mutabile di 
						    						x 
						                       //  |
						    *y += 1;           //  |
						                       //  |
						    println!("{}", x); // -+ - qui prova a prendere a prestito 
						    					     immutabile x
						}                      // -+ qui finisce il prestito mutabile di 
													 x
						                       
						 Gli ambiti sono in conflitto: non possiamo fare un &x mentre y è nell'ambito.


					 	AMBITO CHIAVE DEL PRESTITO
						 Perciò quando aggiungiamo le graffe:

							let mut x = 5;

							{
							    let y = &mut x; // -+ qui inizia il prestito mutabile
							    *y += 1;        //  |
							}                   // -+ ... e qui finisce

							println!("{}", x);  // <- qui prova a prendere a prestito immutabile x
							Non c'è problema. Il nostro prestito mutabile esce di ambito prima che venga creato quello immutabile. Perciò l'ambito è la chiave per vedere quanto dura un prestito.
							vedi --->  @esempio@di_(area scope di y)

		Difetti prevenuti dai prestiti		4.9.4
					Perché ci sono queste regole restrittive? Beh, come abbiamo detto, queste regole prevengono le corse ai dati. Le corse ai dati che genere di difetti provocano? Ecconi alcuni.

			Invalidazione degli iteratori		4.9.4.1
				Un esempio è l'‘invalidazione degli iteratori’, che avviene quando si prova a mutare una collezione su cui si sta iterando. Il verificatore dei prestiti di Rust previene che accada:

					let mut v = vec![1, 2, 3];

						for i in &v {
						    println!("{}", i);
						}

				Questo codice stampa i numeri da uno a tre. Mentre iteriamo lungo il vettore, ci vengono dati solamente dei riferimenti agli elementi. E v è esso stesso preso in prestito come immutabile, il che significa che non possiamo cambiarlo mentre stiamo iterando:

				let mut v = vec![1, 2, 3];

					for i in &v {
					    println!("{}", i);  //fino qui &v = immutabile
					    v.push(34);			//no push perchè È IMMUTABILE
					}


				cannot borrow `v` as mutable because it is also borrowed as immutable
    			v.push(34);

    			 non può prendere in prestito 'v' come mutevole perché è anche preso in prestito come immutabile v.push(34);

    			note: previous borrow of `v` occurs here; the immutable borrow prevents
					subsequent moves or mutable borrows of `v` until the borrow ends
					for i in &v {
					          ^

	          	Nota: qui si verifica il precedente prestito di 'V'; Il prestito immutabile impedisce
					Mosse successive o prestiti mutevoli di 'V' fino alla fine del prestito
					per i in &v {
					          ^

	          	Non possiamo modificare v perché è preso in prestito dal ciclo.

		Uso dopo il rilascio				4.9.5

			RIFERIMENTO NON PIU LUNGI DELLA VITA DELL'OGGETTO DI RIFERIMENTO
			I riferimenti non devono vivere più a lungo dell'oggetto a cui fanno riferimento. Rust verificherà gli ambiti dei riferimenti per assicurare che sia così.

			Se Rust non verificasse questa proprietà, potremmo accidentalmente usare un oggetto che è diventato invalido. Per esempio:


				let y: &i32;
					{
					    let x = 5;  //creo il legame con x
					    y = &x;		//faccio un riferimento a x per assegnarlo a Y 
					    			//che ha valore finche esiste x
					}

					//fuori dell'ambito di x e quindi y non è valido
					println!("{}", y);
			

			Otteniamo questo errore:

				error: `x` does not live long enough
				    y = &x;
				         ^
				note: reference must be valid for the block suffix following statement 0 at
				2:16...
				let y: &i32;
				{
				    let x = 5;
				    y = &x;
				}

				note: ...but borrowed value is only valid for the block suffix following
				statement 0 at 4:18
				    let x = 5;
				    y = &x;
				}

			In altre parole, il valore di y è valido solamente per l'ambito dove x esiste. Non appena x se ne va, diventa invalido fare riferimento ad esso. Come tale, l'errore dice che il prestito ‘non vive abbastanza a lungo’ perché non è valido per la giusta quantità di tempo.

			STESSO ERRORE Y VIVE PIU A LUNGO DI X PERCHE DICHIARATO PRIMA ES.
				Il medesimo problema avviene quando il riferimento è dichiarato prima della variabile a cui si riferisce. Questo è devuto al fatto che le risorse entro lo stesso ambito vengono rilasciate nell'ordine inverso di quello con cui sono state acquisite:

					let y: &i32;  	//Y creata prima di x 
					let x = 5;		//x muore prima di y
					y = &x;         

					println!("{}", y); //da errore su y perche x è cessata prima e 
										quindi viene usata il legame di x quando è già stata rilasciata e non esiste piu mentre Y vive piu a lungo

				Otteniamo questo errore:

					error: `x` does not live long enough
					y = &x;
					Errore: 'X' non vive abbastanza a lungo
					y = &x;


				Nell'esempio qui sopra, y è dichiarato prima di x, il che comporta che y vive (leggermente) più a lungo di x, il che non è consentito.

 	TEMPO DI VITA										4.10.
 		Note
 			Questa è l'ultima delle tre sezioni che presentano il sistema di possesso di Rust. Qui si assume che siano già state lette le altre due:

				Il possesso, il concetto chiave
				I prestiti, e le loro caratteristiche associate, i ‘riferimenti’
 		I tempi di vita					4.10.1
 				@lifetime_(@tempo@di@vita il sistema di possesso di Rust che imposta il
 					momento in cui deve essere rilasciata la risorsa)


	 		Prestare un riferimento a una risorsa posseduta da qualcun altro può essere complicato. Per esempio, immaginiamo questa sequenza di operazioni:

	 			1 Acquisisco un riferimento a una risorsa di qualche tipo.
				2 Ti presto un riferimento a tale risorsa.
				3 Decido di aver finito di lavorare con quella risorsa, e quindi la
				  rilascio, mentre tu hai ancora il tuo riferimento a tale risorsa.
				4 Tu decidi di usare quella risorsa.

						@puntatore@penzolante_(un riferimento ad un risorsa non valida)
			Ahi, ahi! Il tuo riferimento sta puntando a una risorsa non più valida. Questo difetto si chiama ‘puntatore penzolante‘ o ‘utilizzo dopo il rilascio’.

						@life@time_(tempo di vita del riferimento fino al rilascio della risorsa.)
			Per correggerlo, dobbiamo assicurarci che il passo 4 non avvenga mai dopo il passo 3. Il sistema di possesso in Rust lo fa tramite un concetto chiamato "tempo di vita" ["lifetime"], che descrive l'ambito in cui un riferimento è valido. Nel nostro caso, o decidiamo che il tempo di vita vale solamente fino al passo 3, e in tal caso il passo 4 darà errore di compilazione, o decidiamo che il tempo di vita vale fino al passo 4, e in tal caso la risorsa dovrà essere rilasciata al passo 5.

			RIFERIMENTI IMPLICITI O ESPLICITI
			Quando abbiamo una funzione che prende un argomento per riferimento, possiamo essere impliciti o espliciti riguardo al tempo di vita di tale riferimento:
				
				// implicito
					fn foo(x: &i32) {
					}

					// esplicito : bar ha un solo tempo di vita <'a>
					fn bar<'a>(x: &'a i32) {  //
					}
			L'a si legge ‘il tempo di vita a’.
			Tecnicamente, ogni riferimento ha qualche tempo di vita associato ad esso, ma il compilatore consente di eliderlo (cioè ometterlo, si veda la sezione ["@Elisione del tempo di vita"] [elisione del tempo di vita] più avanti) nei casi più tipici.
			erò, prima di arrivarci, scomponiamo l'esempio esplicito:

				fn bar<'a>(...) = angolari  <> dopo il nome della funzione per dichiarare i tempi di 
											   vita personali


			Precedentemente abbiamo parlato un po' della sintassi delle funzioni, ma non abbiamo discusso dei <> dopo il nome della funzione. Una funzione può avere dei ‘parametri generici’ fra le <>, dei quali i tempi di vita sono un tipo. Discuteremo altri tipi di generici più avanti nel libro, ma per adesso, focalizziamoci sull'aspetto dei tempi di vita.

			Usiamo le <> per dichiarare i nostri tempi di vita. Questo dice che bar ha un solo tempo di vita, 'a. Se avessimo dei parametri riferimento, si presenterebbe così:

				fn bar<'a, 'b>(...)

			Poi nel nostro elenco di argomenti, usiamo i tempi di vita che abbiamo nominato:

				...(x: &'a i32)  

			Se avessimo voluto un riferimento &mut, avremmo scritto:

				...(x: &'a mut i32)  //riferimento mutabile i  32 con tempo di vita di a

			Confrontando &mut i32 con &'a mut i32, si nota che 
				l'unica differenza è che il tempo di vita 'a si è intrufolato fra il & il mut i32. 

			La clausola &mut i32 va letta come 
				‘un riferimento mutabile a un i32’, mentre 
				la clausola &'a mut i32 va letta come ‘un riferimento mutabile a un i32 con tempo di vita 'a’.
 		Nelle struct					4.10.2
 				Note
 					creo un programma per l'esempio tempo_di_vita_struct vedi
 					esempio:
 					 	@esempio@tempo_(di vita foo @NELLA @STRUCT o @struttura)

 			C'è bisogno dei tempi di vita espliciti anche quando si lavora come le struct che contengono riferimenti:


 				//TEMPI DI VITA ESPLICITI = riferimento a Foo con tempo di vita = a i32 che lo contiene e non superiore
 				struct Foo<'a> {  //Foo<'a> = strutture con tempi di vita espliciti
					    x: &'a i32,
					}

					fn main() {
					    let y = &5; // questo è lo stesso che `let _y = 5; let y = &_y;`
					    let f = Foo { x: y };   //chiami foo e passi x:Y ossia il valore di Y a x
					    //f
					    println!("{}", f.x);
					}


					Come si vede, anche le struct possono avere tempi di vita. In modo simile alle funzioni, truct Foo<'a> { = dichiara un tempo di vita, e x: &'a i32, lo usa. 

					Allora perché qui ci serve un tempo di vita? Ci serve per assicurare che ogni : riferimento a un Foo non possa sopravvivere al 
					riferimento a un i32 che contiene.
		I blocchi impl					4.10.3
 			Implementiamo un metodo su Foo:
 					@tempi@di@vita_(nella struct e nella implementazione che deve esser ripetuto 
 						due volte il tempo di vita)

 			//01 la struttura Foo con tempo di vita
 			struct Foo<'a> {
    			x: &'a i32,
				}
			//02 implementi foo ma con doppia ripetizione del tempo di vita,
			//impl<'a> 	= definisce il tempo di vita
			//Foo<'a> 	= utilizza il tempo di vita
				impl<'a> Foo<'a> {
				    fn x(&self) -> &'a i32 { self.x }
				}

				fn main() {
				    let y = &5; // questo è lo stesso che `let _y = 5; let y = &_y;`
				    let f = Foo { x: y };
				    println!("x is: {}", f.x());
				}


			Come si vede, dobbiamo dichiarare un tempo di vita per Foo nella riga di impl. 'a viene ripetuto, come per le funzioni: impl<'a> definisce un tempo di vita 'a, e Foo<'a> lo usa.
		Tempi di vita multipli			4.10.4
	 		Se si hanno riferimenti multipli, si può usare lo stesso tempo di vita più volte:

	 			//tempi di vita uguali tra x e y
	 			fn x_o_y<'a>(x: &'a str, 
	 						 y: &'a str) -> &'a str {

		 	Questo dice che sia x che y sono vivi per lo stesso ambito, e che anche il valore reso è vivo per lo stesso ambito. 

		 	Se si voless che x e y avessero tempi di vita diversi, si possono usare più parametri di tempo di vita:

		 		//TEMPO DI VITA DIVERSI ma con con valore reso uguale a quello di x
		 		fn x_o_y<'a, 'b>(x: &'a str,  //ritorna il risultato con il tempo di vita di x   
		 		                 y: &'b str) -> &'a str { ...

	     	In questo esempio, x e y hanno diversi ambiti validi, ma il valore reso ha lo stesso tempo di vita di x.
 		Pensare agli ambiti				4.10.5
 			Un modo di pensare ai tempi di vita è visualizzare l'ambito per cui un riferimento rimane valido. Per esempio:
				 	fn main() {
				    let y = &5;    			// -+ y entra nell'ambito
				                   			//  |
				    // scrivi coidice       //  |
				                            //  |
				}                  			// -+ y esce dall'ambito (RAPPRESENTATAO DALLA GRAFFE)
			
			Aggiungendo il nostro Foo:

				//aggiunto il Foo con ambito di vita della x
				struct Foo<'a> {
				    x: &'a i32,
				}

				fn main() {
				    let y = &5;           // -+ y entra nell'ambito y=riferimento a 5
				    let f = Foo { x: y }; // -+ f entra nell'ambito - chiama foo e gli passa y
				    // roba               //  |
				                          //  |
				}                         // -+ prima f e poi y escono dall'ambito
										  //alla fine delle graffe sia y che f ossia x hanno lo stesso
										  //tempo di vita e quindi vengono distrutte in modo uguale	


			   Il nostro f vive entro l'ambito di y, perciò tutto funziona. E non fosse così? Questo codice non funziona:

					struct Foo<'a> {
					    x: &'a i32,
					}

					fn main() {
					    let x;                    // -+ x entra nell'ambito ma fuori le graffe
					                              //  |
					    {                         //  |
					        let y = &5;           // ---+ y entra nell'ambito piu piccolo le graffe interne
					        let f = Foo { x: y }; // ---+ f entra nell'ambito
					        x = &f.x;             //  | | errore qui
					    }                         // ---+ prima f e poi y escono dall'ambito
					                              //  |
					    println!("{}", x);        //  |  quindi x non ha piu riferimento perche distrutta la x insieme a y
					}                             // -+ x esce dall'ambito


				Come si vede, gli ambiti di f e y sono più piccoli dell'ambito di x. 
				Ma quando facciamo x = &f.x, rendiamo x un riferimento a qualcosa che sta per uscire dal suo ambito.

				I tempi di vita con nome sono un modo di dare un nome a questi ambiti. 
				Dare un nome a qualcosa è il primo passo verso l'essere capaci di parlarne.

 		static							4.10.5
 			@static_(tempo di vita dell'intero programma)
	 		Il tempo di vita chiamato ‘static’ è un tempo di vita speciale. Segnala che qualcosa ha il tempo di vita dell'intero programma. La maggior parte dei programmatori Rust si imbattono per la prima volta in 'static quando trattano le stringhe:
	 			
	 			//stringa static che viene salvata alla fine del file binario e vale per tutto il programma
	 			let x: &'static str = "Ciao, mondo.";

			Le lettere della stringa sono di tipo &'static str perché il riferimento è sempre vivo: vengono depositati nel segmento dati del file binario finale. Un altro esempio sono i globali:

				//variabile globale, i32 inserita nel segmento dati del file binario
				static FOO: i32 = 5;
				let x: &'static i32 = &FOO;

			Questo aggiunge un i32 al segmento dati del file binario, e x è un riferimento a esso.

 		Elisione del tempo di vita		4.10.6

 			INFERENZA LOCALE SOLO NEI CORPI DELLE FUNZIONI - NESSUNA INFERENZA NELLE FIRME -
 			Rust supporta una potente inferenza di tipo locale nei corpi delle funzioni ma non nelle firme dei loro elementi. È vietato consentire di ragionare sui tipi a seconda della sola firma degli elementi. 
 			Però, per ragioni di comodità, un algoritmo di INFERENZA SECONDARIA molto ristretto chiamato “ELISIONE DEL TEMPO DI VITA” si applica quando si giudicano i tempi di vita. 

 			L'elisione dei tempi di vita viene considerata solamente per inferire i parametri del tempo di vita usando tre regole facilmente memorizzabili e non ambigue. 
 			
 				Ciò significa che 
 					l'elisione del tempo di vita agisce da abbreviazione per scrivere una firma di un elemento, 
				mentre non nasconde i tipi effettivamente coinvolti, come avverrebbe se fosse applicata una una completa inferenza locale. 	
		
			ELISIONE DI TEMPO DI VITA DI IMPUT E DI OUTPUT
			Quando si parla dell'elisione del tempo di vita, usiamo i termini tempo di vita di input e tempo di vita di output. Un tempo di vita di input è un tempo di vita associato a un argomento di una funzione, mentre un tempo di vita di output è un tempo di vita associato a un valore reso da una funzione. Per esempio, questa funzione ha un tempo di vita di input:
				//TEMPO DI VITA DI INPUT
				fn foo<'a>(bar: &'a str)

				Quest'altra ha un tempo di vita di output:

				//attenzione () = non c'è parametro
				fn foo<'a>() -> &'a str   //restituzione del'outuput


				E questa ha un tempo di vita in entrambe le posizioni:

				fn foo<'a>(bar: &'a str) -> &'a str  //tempo di vita di imput e di output


				Ecco le tre regole: (?? non chiare??)

					Ogni tempo di vita eliso tra gli argomenti di una funzione diventa un un distinto parametro tempo di vita.

					Se c'è esattamente un tempo di vita di input, eliso o no, quel tempo di vita è assegnato a tutti i tempi di vita elisi nei valori resi di quella funzione.

					Se ci sono più tempo di vita di input, ma uno di essi è 
						&self o 
						&mut self, 
					il tempo di vita di self viene assegnato a tutti i tempi di vita di output elisi.

					Altrimenti, è un errore elidere un tempo di vita di output.

 		Esempi							4.10.7
	 		Ecco alcuni esempi di funzioni con tempi di vita elisi. Abbiamo accoppiato ogni esempio di un tempo di vita eliso con la sua forma espansa.
	 			fn stampa(s: &str); // eliso
				fn stampa<'a>(s: &'a str); // espanso

					//lvl: u32 = non ha tempo di vita
				fn debug(lvl: u32, s: &str); // eliso
				fn debug<'a>(lvl: u32, s: &'a str); // espanso

			Nell'esempio precedente, lvl non ha bisogno di un tempo di vita, perché non è un riferimento (&). 

			Solamente oggetti riferiti a riferimenti (come uno struct che contiene un riferimento) hanno bisogno di tempi di vita.
					//@molto@difficili_(non si comprendono ??? @tempi@di@vita @incomprensibili)

					fn substr(s: &str, until: u32) -> &str; // eliso
					fn substr<'a>(s: &'a str, until: u32) -> &'a str; // espanso

					fn get_str() -> &str; // ILLEGALE, nessun input

					fn frob(s: &str, t: &str) -> &str; // ILLEGALE, due input
					// espanso: il tempo di vita di output è ambiguo
					fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str;

					fn get_mut(&mut self) -> &mut T; // eliso
					fn get_mut<'a>(&'a mut self) -> &'a mut T; // espanso

					fn argomenti<T: ToCStr>(&mut self, args: &[T]) -> &mut Command; // eliso
					fn argomenti<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T])
					-> &'a mut Command; // espanso

					fn new(buf: &mut [u8]) -> BufWriter; // eliso
					fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // espanso

 IMMUTABILITA											4.11.
 	NON DI DEFAULT
	 	La mutabilità, ossia la capacità di cambiare qualcosa, funziona in Rust un po' diversamente che in altri linguaggi. Il primo aspetto della mutabilità in Rust è il fatto di non esserci per default:
	 		let x = 5;
			x = 6; // errore!
		Si può introdurre la mutabilità con la parola-chiave mut:
			let mut x = 5;
			x = 6; // non c'è problema!

			Questo è un legame di variabile mutabile. Quando un legame è mutabile, significa che si può cambiare ciò a cui quel legame punta. Perciò nell'esempio sopra, non è tanto che il valore alla posizione x cambia, quanto che il legame passa dal puntare un i32 al puntarne un altro.

			
			Si può anche creare un riferimento ad esso, usando &x, ma se si vuole usare il riferimento per cambiarlo, ci vorrà un riferimento mutabile:
				let mut x = 5;
				let y = &mut x;  //y legame immutabile di default che non si lega ad altro valore ma
								 //viene utilizzato y per legare x (o riferimento a x mutevole)

				y è un legame immutabile a un riferimento mutabile, il che significa che non si può legare 'y' a qualcos'altro (y = &mut z), ma y può essere usato per legare x a qualcos'altro (*y = 5). Una distinzione sottile.

		Naturalmente, se servono entrambi:
			let mut x = 5;
			let mut y = &mut x;

			Adesso y può essere legato a un altro valore, e inoltre il valore che sta referenziando può essere cambiato.

		È importante notare che mut fa parte di un pattern, e perciò si possono fare cose come questa:

			let (mut x, y) = (5, 6);

			fn foo(mut x: i32) {

		Si noti che qui la x è mutabile, mentre la y non lo è.

 	
 	Mutabilità interiore contro mutabilità esteriore						4.11.1 	
 		MUTABILITA ESTERIORE
	 		Però, quando diciamo che qualcosa è ‘immutabile’ in Rust, non intendiamo che è impossibile cambiarla: ci stiamo riferendo alla sua ‘immutabilità esteriore’. Si consideri, per esempio, 
	 		Arc<T>:
	 			use std::sync::Arc;  //OGGETTO Arc<T>
				let x = Arc::new(5); //X legame immutabile
				let y = x.clone();

			Quando chiamiamo clone(), l'oggetto di tipo Arc<T> deve aggiornare il conteggio dei riferimenti. Però qui non abbiamo usato nessun mut, x è un legame immutabile, e non abbiamo preso il valore &mut 5 né altri valori. E allora?

			Per capirlo, dobbiamo tornare al nucleo della filosofia guida di Rust, che è la sicurezza di memoria, e al meccanismo col quale Rust la garantisce, che è il sistema di [possesso][possesso], e più specificamente, il [prestito][prestito]:

				Si possono avere l'uno o l'altro di questi due tipi di prestiti, ma non entrambi allo stesso tempo:

					uno o più riferimenti immutabili (&T) a una risorsa,
					esattamente un riferimento mutabile (&mut T).

			Perciò, questa è la vera definizione di ‘immutabilità’: è sicuro che ci siano due puntatori a questo oggetto? Nel caso di Arc<T>, sì: la mutazione è contenuta interamente dentro la struttura stessa. Non si presenta all'utente. Per questa ragione, passa un &T a clone(). Se gli avesse passato un &mut T, però, sarebbe un errore.

		MUTABILITA INTERIORE
			Altri tipi, come quelli nel modulo std::cell, sono nella situazione opposta: mutabilità interiore. Per esempio:
				use std::cell::RefCell;

				let x = RefCell::new(42);  //RefCel pasa dei riferimenti &mut

				let y = x.borrow_mut();    //lo passa al suo metodo borrow_mut


			RefCell passa dei riferimenti &mut a ciò che c'è al suo interno usando il metodo borrow_mut(). Non è pericoloso? Che succede se facciamo:

				use std::cell::RefCell;

				let x = RefCell::new(42);

				let y = x.borrow_mut();
				let z = x.borrow_mut();

			Di fatto questo andrà in panico, in fase di esecuzione. Questo è quello che fa RefCell: forza le regole di prestito di Rust in fase di esecuzione, e va in panic! se sono violate. Questo ci consente di aggirare un altro aspetto delle regole di mutabilità di Rust. Prima parliamone.


	Mutabilità interiore contro mutabilità esteriore						4.11.2
	Mutabilità a livello di campo											4.11.3
		La mutabilità è una proprietà o di un prestito (&mut) o di un legame (let mut). Ciò significa che, per esempio, non si può avere una struct con alcuni campi mutabili e altri immutabili:

		struct Punto {
		    x: i32,			//questo non è mutabile 
		    mut y: i32, // non si può  -- solo questo è mutabile --
		}


		La mutabilità di una struct sta nel suo legame:

			struct Punto {
			    x: i32,
			    y: i32,
			}
			
			let mut a = Punto { x: 5, y: 6 };

			a.x = 10;   //immutabile

			let b = Punto { x: 5, y: 6};

			b.x = 10; // errore: non si può assegnare al campo immutabile `b.x`


		Però, usando Cell<T>, si può emulare la mutabilità a livello di campo:


				use std::cell::Cell;

				struct Punto {
				    x: i32,
				    y: Cell<i32>,
				}

				let punto = Punto { x: 5, y: Cell::new(6) };

				punto.y.set(7);

				println!("y: {:?}", punto.y);
				Questo stamperà y: Cell { value: 7 }. Abbiamo aggiornato y con successo.

 STRUTTURE 												4.12.
 		COMBINA DATI COMPLESSI IN UN SINGOLO DATO
		 	Le struct sono un modo di creare tipi di dati più complessi. Per esempio, se stessimo facendo calcoli che coinvolgono coordinate nello spazio 2D, cioè nel piano cartesiano ci servirebbero sia un valore x che un valore y:
		 		let origine_x = 0;
				let origine_y = 0;
			Una struct ci permette di combinare questi due oggetti in UN SINGOLO TIPO DI DATI UNIFICATO, i cui campi sono etichettati x e y:

					struct Punto { //STRUTTURA CHE COMBINA 2 DATI IN UN SINGOLO TIPO Punto
					    x: i32,
					    y: i32,
					}


					fn main() {
					    let origine = Punto { x: 0, y: 0 }; // origine: Punto

					    println!("L'origine è in ({}, {})", origine.x, origine.y);
					}
		PAROLA CHIAVE STRUCT
				
				@struct@convenzione_(la convenzione nel dichiarare la struct è Camel Case o stile @Pascal)

			Qui ci sono molte cose, analizziamole. Una struct si dichiara con la parola-chiave struct, e poi con un nome. Per convenzione, le struct hanno la MAIUSCOLIZZAZIONE DEL PASCAL: 	
				PuntoNelloSpazio, 
			non Punto_Nello_Spazio, né punto_nello_spazio.


			LET = Possiamo creare un'istanza della nostra struct usando let, come al solito, ma per impostare ogni campo usiamo una sintassi con lo stile chiave: valore. L'ordine non dev'essere il medesimo della dichiarazione originale.

			ACCESSO AI CAMPI = Infine, siccome i campi hanno un nome, possiamo accedervi tramite la notazione a punto: origine.x


			VALORI IMMUTABILI DI DEFAULT = I valori nelle struct sono immutabili di default, come gli altri legami in Rust. Si deve usare mut per renderli mutabili:

				//struttura con con stile pascal
				struct Punto { //valori immutabili  NON SI PUO' USARE MUT nella struttura solo &
					    x: i32,
					    y: i32,
					}

					fn main() {
						//vengono resi mutabili i valori della struct con let mut punto
					    let mut punto = Punto { x: 0, y: 0 };

					    //accesso ai campi con origine.campo
					    punto.x = 5;

					    println!("Il punto è in ({}, {})", punto.x, punto.y);
					}
						Questo stamperà Il punto è in (5, 0).


			NO MUTABILITA DEI CAMPI DELLA STRUCT = Rust non supporta la mutabilità dei campi a livello del linguaggio, quindi non si può scrivere qualcosa così:

					struct Point {
					    mut x: i32,  //ERRORE non si puo rendere mutabile un campo della struct
					    y: i32,
					}


			MUTABILITA SOLO DEGLI OGGETTI IN MODO TEMPORANEO = La mutabilità è una proprietà del legame, non della struttura stessa. Chi fosse abituato all mutabilità a livello di campo, lo può trovare strano dapprima, ma semplifica parecchio le cose. Consente perfino di rendere temporaneamente mutabili degli oggetti:

					//@Struttura_(@struct con @campi @immutabili di default)
					struct Punto {
					    x: i32,
					    y: i32,
					}

					fn main() {
						
						//@struct@mut_(viene reso @mutabile l'oggetto della struttura)
					    let mut punto = Punto { x: 0, y: 0 };

					    punto.x = 5; //qui assegni perchè è mutabile

					    let punto = punto; // con il nuovo let adesso è immutabile

					    punto.y = 6; // questo provoca un errore non puo' essere modificato
					}

			STRUTTURA CON PUNTATORI = Però una struttura può contenere dei puntatori &mut, che consentono di applicare qualche tipo di mutazione:


				//STRUTTURA
				struct Punto {
					    x: i32,
					    y: i32,
					}

					//STRUTTURA CON PUNTATORI CON &mut
					struct RifPunto<'a> {
					    x: &'a mut i32,   //puntatori con &mut
					    y: &'a mut i32,
					}

					fn main() {
					    let mut punto = Punto { x: 0, y: 0 };  //assegno a variabile mutabile

					    {
					        //ASSEGNO a r i riferimenti mutabili di punto.x e punto.y
					        let r = RifPunto { x: &mut punto.x, y: &mut punto.y };

					        *r.x = 5; //assegno al campo r.x, r.y i valori
					        *r.y = 6;
					    }

			

					    assert_eq!(5, punto.x);
					    
					    		//stampo dopo assert x
					    		println!("stampo dopo il codice assert_eq!(5, punto.x) il valore di x che è = {:?}", punto.x);
					    assert_eq!(6, punto.y);

					    		//stampo dopo assert y
					            println!("stampo dopo il codice assert_eq!(5, punto.y) il valore di y che è = {:?}", punto.y);
			
					}

	Strutture ennuple									4.12.1 
		IBRIDO TRA ENNUPLA E UNA STRUCT
			Rust ha un altro tipo di dati che è come un ibrido fra una ennupla e una struct, e si chiama ‘struttura ennupla’. Le strutture ennuple hanno un nome, ma i loro campi no. Sono dichiarate con la parola-chiave struct, e poi con un nome seguito da un'ennupla:

				struct Colore(i32, i32, i32);
				struct Punto(i32, i32, i32);

				let nero = Colore(0, 0, 0);
				let origine = Punto(0, 0, 0);


			Qui, nero e origine non sono dello stesso tipo, anche se contengono campi degli stessi tipi.

			Si può accedere ai membri di una struttura ennupla tramite la notazione a punto o il let destrutturante, proprio come le normali ennuple:

				let nero_r = nero.0;
				let Punto(_, origine_y, origine_z) = origine;



			I pattern come Punto(_, origine_y, origine_z) sono usati anche nelle espressioni match.

		UTILITA DELLA STRUTTURA ENNUPLA CON UN SOLO ELEMENTO
			Un caso in cui una struttura ennupla è molto utile è quando ha un solo elemento. Questo viene chiamato il pattern ‘newtype’, perché consente di creare un nuovo tipo che è distinto da quello del suo valore contenuto ed esprime anche un suo significato semantico:

				//STRUTTURA ENNUPLA CON 1 ELEMENTO PATTERN newtype
				struct Pollici(i32);				//tipo pollici	

					let lunghezza = Pollici(10);   //assegno a lunghezza pollici 10 e creo un nuovo tipo
												   //lunghezza	diverso da pollici

					let Pollici(lunghezza_intera) = lunghezza;   //assegno a pollici lunghezza
					println!("la lunghezza è {} pollici", lunghezza_intera);  //posso stampare lunghezza

			Come sopra, si può estrarre il tipo intero interno tramite un let destrutturante. In questo caso, il 
				let Pollici(lunghezza_intera) 
			assegna 10 a lunghezza_intera. 

			Avremmo potuto usare la notazione a punto per fare la stessa cosa:
				let lunghezza_intera = lunghezza.0;


		ALTERNATIVA AD UNA ENNUPLA USARE UNA STRUCT
			È sempre possibile usare una struct invece di una struttura ennupla, e può essere più chiara. Avremmo potuto scrivere Colore e Punto anche così:

				//Creo una struttura al posto di una ennupla = struct Pollici(i32);
				struct Colore {
				    rosso: i32,
				    blu: i32,
				    verde: i32,
				}

				//nella struttura ci sono nomi effettivi non le posizioni come la ennupla
				//e puoi utilizzare la notazione a punto es. Punto.x
				struct Punto {
				    x: i32,
				    y: i32,
				    z: i32,
				}

			I buoni nomi sono importanti, e mentre si può fare riferimento ai valori in una struttura ennupla anche con la notazione a punto, una struct ci dà dei nomi effettivi piuttosto che delle posizioni.
	Struct simili a unità 								4.12.2
		Note
			Si può anche definire una struct senza nessun membro:

				//STRUTTURE SENZA MEMBRO CON GRAFFE O SENZA 
				struct Elettrone {} // si usano le graffe vuote...
				struct Protone;     // ...o solo un punto-e-virgola

				// che la struct sia stata dichiarata con le graffe oppure no,
				// si deve fare lo stesso quando se ne istanzia una
				let x = Elettrone {};
				let y = Protone;


				Una tale struct è chiamata ‘simile a unità’ perché somiglia alla ennupla vuota, (), che talvolta è chiamata ‘unità’. Come una struttura ennupla, definisce un nuovo tipo.

				Questo tipo è usato raramente da solo (sebbene talvolta può servire come tipo marcatore), ma in combinazione con altre caratteristiche, può diventare utile. Per esempio, una libreria può chiedere di creare una struttura che implementi un certo tratto per gestire eventi. Se non si hanno dati da mettere nella struttura, si può creare struct simile a unità.
 		
 ENUMERAZIONI											4.13.
		Note
				 @enumerazioni_(puo essere chiamata @tipo @somma e quindi un tipo @chiave @valore e puo
				 				contenere valori diversi)

			Una enumerazione in Rust è un tipo, dichiarato usando la parola-chiave enum, che rappresenta un dato che ha un valore scelto in un elenco di varianti. Ogni variante dell'enumerazione può avere dei dati associati ad essa. Per esempio:
				
				//enum con dati associati o no  puo contenere dati non definiti, i32 o String
				enum Message {

					//solo chiave
				    Abbandona,						//variante senza dati (sono simili a strutture senza
				    								// unita)
				    //chiave e valore				
				    CambiaColore(i32, i32, i32),	//variante con i dati senza nome
				    Muovi { x: i32, y: i32 },		//variante con i dati con nome
				    Scrivi(String),
				}

			La sintassi per definire le varianti somiglia alle sintassi usate per definire le strutture: si possono avere 
				varianti senza dati (come le strutture simili a unità), 
				varianti con dati con nome, e 
				varianti con dati senza nome (come le strutture ennupla). 
			
			Diversamente dalle definizioni di strutture distinte, però, un enum è un singolo tipo. 
			Un valore di un'enumerazione può corrispondere a una qualunque delle sue varianti. 

			Per questa ragione, un'enumerazione è talvolta chiamata un 
				‘tipo somma’: l'insieme dei possibili valori dell'enumerazione è la somma degli insiemi dei possibili valori di ogni variante.

		SINTASSI ::
			Usiamo la sintassi :: per usare il nome di ogni variante: il loro nome può essere usato solo se qualificato dal nome dell'enum stesso. Ciò consente di avere le seguenti dichiarazioni di x e y:
				let x: Messaggio = Messaggio::Mossa { x: 3, y: 4 };

					enum TurnoGiocoDaTavolo {
						//etichetta dei dati Mossa e Passo
					    Mossa { quadrati: i32 },	//VARIANTE CON DATI E NOME
					    Passo,						//VARIANTE SOLO NOME SENZA DATI
					}


					let y: TurnoGiocoDaTavolo = TurnoGiocoDaTavolo::Mossa { quadrato: 1 };


					Entrambe le varianti si chiamano Mossa, ma dato che sono qualificate dal nome dell'enumerazione, possono essere usate entrambe senza creare ambiguità.??


			VALORE ENUM COSA CONTIENE
					@unione@etichettata(significato delle varianti enum)_@tagged @union

				Un valore di un tipo enum contiene 
					l'informazione su quale variante è, 
					oltre a ogni dato associato a quella variante. 

				Questo fatto è talvolta indicato come ‘unione etichettata’ ["tagged union"], dato che i 
					dati comprendono un'’etichetta’ ["tag"], che indica qual'è il tipo. 

				Il compilatore usa questa informazione per garantire che si acceda in modo sicuro ai dati dell'enumerazione. 

				Per esempio, non si può semplicemente provare a destrutturare un valore come se fosse una delle possibile varianti:

					fn elabora_cambio_colore(mess: Messaggio) {
						    let Messaggio::CambiaColore(r, g, b) = mess; // errore di compilazione
						}



				COME SUPERI LA LIMITAZIONE DELLA MANCANZA DI SUPPORTO DELLE OPERAZIONI: Non supportare queste operazioni può sembrare piuttosto limitativo, ma è una limitazione che possiamo superare. 
				Ci sono due modi: o 
					implementando l'uguaglianza noi stessi, 
						o 
					eseguendo un pattern matching delle varianti con espressioni match, come vedremo nella prossima sezione. 

				Non abbiamo ancora visto abbastanza Rust per implementare l'uguaglianza, ma lo vedremo nella sezione sui [tratti][tratti]. ??


 	Costruttori come funzioni							4.13.1
 		Note

			Un costruttore di un enum può anche essere usato come una funzione. Per esempio:

				let m = Messaggio::Scrivi("Ciao, mondo".to_string());

			è lo stesso che
				//costruisco la funzione con restituzione 
				fn foo(x: String) -> Messaggio 
					{
					    Messaggio::Scrivi(x)
					}

				//assegno a x la stringa e chiamo la funzione
				let x = foo("Ciao, mondo".to_string());



			Questo non ci è immediatamente utile, ma quando arriveremo alle chiusure, parleremo di come 	passare le funzioni come argomenti ad altre funzioni. 

			Per esempio, 
				con gli iteratori, 

			possiamo fare questo per convertire un vettore di String in un vettore di Messaggio::Scrivi:

					//vettore di string convertito in vettore di messaggio

					let v = vec!["Ciao".to_string(), "Mondo".to_string()];   //vettore di stringhe

					//conversione vettore di stringhe in vettore di messaggio
					let v1: Vec<Messaggio> = v.into_iter().map(Messaggio::Scrivi).collect();

 MATCH													4.14.
 		Note
 			Spesso, un semplice if/else non basta, perché ci sono più di due opzioni possibili. Inoltre, le condizioni possono diventare parecchio complesse. Rust ha una parola-chiave, match ("combacia"), che consente di sostituire dei complicati raggruppamenti di if/else con qualcosa di più potente. Ecco qua:

 				let x = 5;

					match x {
						//braccio valore espressione
					    1 => println!("uno"),
					    2 => println!("due"),
					    3 => println!("tre"),
					    4 => println!("quattro"),
					    5 => println!("cinque"),
					      => println!("qualcos'altro"),
					      _  //qui il compilatore inferisce come i32					}

			match prende un'espressione ed esegue una diramazione in base al suo valore. Ogni ‘braccio’ della diramazione ha la forma valore => espressione. 

			Quando il valore combacia, l'espressione di quel braccio viene valutata. Viene chiamata match a causa del concetto di ‘pattern matching’, di cui match è un'implementazione. C'è un separate section on patterns che tratta di tutti i pattern che sono ammessi qui.

			Uno dei molti vantaggi di match è che impone la ‘verifica di esaustività’. Per esempio, se si toglie l'ultimo braccio, quello con il carattere _, il compilatore darà l'errore:
					error: non-exhaustive patterns: `_` not covered

			INFERIMENTO DI X = Rust ci dice che abbiamo dimenticato qualche valore. Il compilatore inferisce dall'x che può avere qualunque valore a 32 bit, da -2.147.483.648 a 2.147.483.647. Il carattere _ agisce da 'prendi-tutto', e prenderà tutti i possibili valori che non sono specificati in un braccio dello stesso match. Come si vede nell'esempio precedente, al match vengono forniti bracci per gli interi da 1 a 5, se x vale 6 o qualunque altro valore, viene preso dal caso _.

			ESPRESSIONE SU LATO DESTRO = Il costrutto match è anche un'espressione, il che significa che lo si può usare al lato destro di un'istruzione let o direttamente dove è ammessa una espressione:

				let x = 5;
					//Lato destro di =; e gli interi vengono convertiti in stringhe 
					let numero = match x {
					    1 => "uno",
					    2 => "due",
					    3 => "tre",
					    4 => "quattro",
					    5 => "cinque",
					    _ => "qualcos'altro",
					};


					Talvolta è un modo carino di convertire qualcosa da un tipo a un altro; in questo esempio gli interi vengono convertiti in String.


 	
	Combaciare le enumerazioni							4.14.1
 			Un altro impiego importante della parola-chiave match sta nell'elaborare le possibili varianti di un'enumerazione:

 				enum Messaggio {
				    Abbandona,
				    CambiaColore(i32, i32, i32),
				    Sposta { x: i32, y: i32 },
				    Scrivi(String),
				}

				fn abbandona() { /* ... */ }
				fn cambia_colore(r: i32, g: i32, b: i32) { /* ... */ }
				fn sposta_cursore(x: i32, y: i32) { /* ... */ }

				fn elabora_messaggio(msg: Messaggio) {
				    match msg {
				        Messaggio::Abbandona => abbandona(),
				        Messaggio::CambiaColore(r, g, b) => cambia_colorw(r, g, b),
				        Messaggio::Sposta { x: x, y: y } => sposta_cursore(x, y),
				        Messaggio::Scrivi(s) => println!("{}", s),
				    };
				}


					Ancora, il compilatore Rust verifica l'esaustività, e richiede di avere un braccio combaciante per ogni variante dell'enum. Se ne manca qualcuno, darà un errore di compilazione, a meno che si usi il braccio _.

					Diversamente dai precedenti utilizzi di match, questo caso non è sostituibile da un semplice uso del costrutto if. Si può però usare il costrutto if let, che può essere visto come una forma abbreviata di match.

 PATTERN												4.15.
 	Note
 			I @pattern sono molto comuni in Rust. 
 			Li usiamo nei 
 				legami di variabile, 
 				nelle espressioni match, 
			e anche in altri posti. Facciamo una carrellata di tutte le cose che i pattern possono fare!
			Un rapido ripasso: si può far combaciare direttamente con letterali, e il carattere _ agisce come caso ‘qualunque’:

				let x = 1;

				match x {
				    1 => println!("uno"),
				    2 => println!("due"),
				    3 => println!("tre"),
				     => println!("qualunque cosa"),
				    _                        //_ = significa @caso @qualunque oppure @non@identificato
				
				}


				Questo stampa uno.

			OSCURAMENTO
						@oscuramento_(esiste anche l'oscuramento tra i @pattern)
				C'è un trabocchetto con i pattern: come ogni cosa che introduce un nuovo legame, anche i pattern possono introdurre l'oscuramento. Per esempio:

						let x = 1;			//I° legame di x
						let c = 'c';

						match c {
						x => println!("x: {} c: {}", x, c),  //II° legame di x che vale c
						}

						println!("x: {}", x)  //x esterna oscura la x interna al match stampa 1

					Questo stampa:

						x: c c: c
						x: 1


				In altre parole, x => combacia con il valore di c e introduce un nuovo legame avente nome x. 
				Questo nuovo legame è ha come ambito il braccio di match e prende il valore di c. 

				Si noti che il valore di x all'esterno dell'ambito di match è ininfluente sul valore x al suo interno. Siccome avevamo già un legame chiamato x, questo nuovo x lo oscura.

 	Pattern multipli									4.15.1
 		Nota
 			Possiamo far combaciare più pattern usando |:
 				let x = 1;

					match x {
						//due pattern 1 | 2
					    1 | 2 => println!("uno o due"), //@pattern @multiplo con |
					    3 => println!("tre"),
					    _ => println!("qualunque cosa"),
					}
					Questo stampa uno o due.

 	Destrutturazione									4.15.2	
 		Nota
 			Se si ha un tipo di dati composito, come una struct, lo si può destrutturare 
 			dentro un pattern:
 				
 				struct Punto {
					    x: i32,
					    y: i32,
					}

					let origine = Punto { x: 0, y: 0 };   //nome base x e y

					match origine {
						//qua destrutturi in x,y??
					    Punto { x, y } => println!("({},{})", x, y),
					}


			Possiamo usare : per dare un altro nome a un valore.

				struct Punto {  //struttura
				    x: i32,
				    y: i32,
				}

				let origine = Punto { x: 0, y: 0 };   //nome base x,y

				match origine {
					//qui dai altro nome, x a x1 e y a y1
				    Punto { x: x1, y: y1 } => println!("({},{})", x1, y1),
				}

			Se ci interessano solamente alcuni valori, non dobbiamo dare dei nomi a tutti:

				struct Punto {
				    x: i32,
				    y: i32,
				}

				let origine = Punto { x: 0, y: 0 };

				match origine {
					//forse qui solo la x??
				    Punto { x, .. } => println!("x is {}", x),
				}

				
				Questo stampa x è 0.



				si può fare questo genere di match su qualunque membro, non solamente il primo:

					struct Punto {
					    x: i32,
					    y: i32,
					}

					let origine = Punto { x: 0, y: 0 };

					match origine {
					    Punto { y, .. } => println!("y is {}", y),
					}
					Questo stampa y è 0.

					Questo comportamento ‘destrutturante’ funziona su qualunque tipo di dati composito, come le ennuple o le enum.

 	Ignorare i legami									4.15.3
 		Note
 			Si può usare _ in un pattern per non tener conto del tipo e del valore. Per esempio, ecco un match con un Result<T, E>:



 			match qualche_valore {
				    Ok(valore) => println!("preso un valore: {}", valore),
				    Err(_) => println!("è avvenuto un errore"),
				}

			Nel primo braccio, leghiamo il valore dentro la variante Ok a valore. 
			Ma nel braccio Err, usiamo _ per non tener conto dello specifico errore, e stampiamo un messaggio d'errore generico.

			COME IGNORARE UN ELEMENTO
				_ è valido in qualunque pattern che crea un legame. Ciò può essere utile per ignorare parti di una struttura più grande:

				fn coordinate() -> (i32, i32, i32) {
					    // genera e restituisci una terna
					}

					let (x, _, z) = coordinate();

				Qui, leghiamo il primo e l'ultimo elemento dell'ennupla a x e a z, ma ignoriamo l'elemento di mezzo.


				Vale la pena notare che usando _ il valore combaciante non viene affatto legato, il che comporta che tale valore non viene spostato:

				let ennupla: (u32, String) = (5, String::from("cinque"));

				// Qui, ennupla viene spostata, perché l'oggetto String è stato spostato:
				let (x, _s) = ennupla;

				// La prossima riga darebbe "error: use of partially moved value: `ennupla`"
				// println!("L'ennupla è: {:?}", ennupla);

				// Però,



				let ennupla = (5, String::from("five"));

				// Qui, ennupla _non_ vien spostata, dato che l'oggetto String non è
				// mai stato spostato, e l'oggetto u32 è Copy:
				let (x, _) = ennupla;

				// Ciò comporta che questo funziona:
				println!("L'ennupla è: {:?}", ennupla);


				Ciò comporta anche che ogni variabile temporanea verrà distrutta alla fine dell'istruzione:

				// Qui, la String creata verrà distrutta immediatamente, dato che non è legata:
				let _ = String::from("  hello  ").trim();



				Si può usare anche .. in un pattern per ignorare più valori:

				enum EnnuplaOpzionale {
				    Valore(i32, i32, i32),
				    Mancante,
				}

				let x = EnnuplaOpzionale::Valore(5, -2, 3);

				match x {
				    EnnuplaOpzionale::Valore(..) => println!("Ho un'ennupla!"),
				    EnnuplaOpzionale::Mancante => println!("Non ho tale fortuna."),
				}
				Questo stampa Ho un'ennupla!

 	ref e ref mut										4.15.4

 			@riferimento_(con let crea un riferimento e mut @riferimento@mutabile)

 		Se si vuole ottenere un riferimento, si usi la parola-chiave ref:
 			let x = 5;

				match x {
					//ref = parola chiave per ottenere un @riferimento
				    ref r => println!("Ho un riferimento a {}", r),
				}

			Questo stampa Ho un riferimento a 5.

			Qui, la r dentro il match ha il tipo &i32. In altre parole, la parola-chiave ref crea un riferimento, da usare nel pattern. Se serve un @riferimento @mutabile, ref mut funzionerà allo stesso modo:

				let mut x = 5;

				match x {
					//ref mut = riferimento mutabile
				    ref mut mr => println!("Ho un riferimento mutabile a {}", mr),
				}

 	Gamme												4.15.5
 			
 			@gamme_(@gamma di @valori es da 1 a 5 con 1 ...5)

 		Si può far combaciare una gamma di valori usando ...:

 		let x = 1;

			match x {
			    1 ... 5 => println!("da uno a cinque"),
			    _ => println!("qualunque cosa"),
			}

		Questo stampa da uno a cinque.
			@utilizzo@gamme_(la gamma di valore vengono usati per gli interi ed i chart)

		Le gamme sono usate per lo più con gli interi e i char:

			@esempio@gamma_(di @char)

			let x = '💅';

			match x {
			    'a' ... 'j' => println!("lettera precoce"),
			    'k' ... 'z' => println!("lettera tardiva"),
			    _ => println!("qualcos'altro"),
			}

			Questo stampa qualcos'altro.

 	Legami												4.15.6
 		Si possono legare valori a nomi usando @:

 			@legami_(si possono legare i valori ai @nome con la chiocciola @)

 			let x = 1;

				match x {
					//lega @ al valore 1 
				    e @ 1 ... 5 => println!("ho un elemento della gamma: {}", e),
				    _ => println!("qualunque cosa"),
				}

				@operatore@chiocciola_(la@ puo essere utilizzata per le estrazioni complicate)

			Questo stampa ho un elemento della gamma: 1. Questo operatore serve anche quando si vuole estrarre una parte di una struttura dati complicata:

					#[derive(Debug)]
						//creo la struttura persona
						struct Persona {
						    nome: Option<String>,
						}

						//assegno il valore di stringa a nome
						let nome = "Steve".to_string();
						//assegno a x ??
						let x: Option<Persona> = Some(Persona { nome: Some(nome) });
						match x {
							//lego il nome ad a 	??? cosa fa Some  todo: spiegare meglio ?? @non@è@chiaro @devi@studiare@meglio											
						    Some(Persona { nome: ref a @ Some(_), .. }) => println!("{:?}", a),
						    _ => {}
						}

						Questo stampa Some("Steve"): abbiamo legato il nome interno ad a.

						Se si usa @ con |, bisogna assicurarsi che il nome sia legato in ogni parte del pattern:

						let x = 5;

						match x {
							//utilizzo @| = @= 1 e |@ = 8 il nome è legato nel lato dx e sx del pattern
						    e @ 1 ... 5 | e @ 8 ... 10 => println!("ho un elemento della gamma: {}", e),
						    _ => println!("qualunque cosa"),
						}

 	Guardie												4.15.7
 		
 			@if@nel@match_(la if applicata al match sia con pattern solo dx che dx e sx)

 		Si possono introdurre le ‘guardie di match’ usando if:
 			//CREO UN ENUM 
 			enum OptionalInt {
			    Valore(i32),
			    Mancante,
			}
			//assegno a x 5
			let x = OptionalInt::Valore(5);



			match x {
			    //metto la if con il Pattern dx che valuta valore(i)= 5
			    OptionalInt::Valore(i) if i > 5 => println!("Ho un int maggiore di cinque!"),
			    OptionalInt::Valore(..) => println!("Ho un int!"),
			    OptionalInt::Mancante => println!("Non ho tale fortuna."),
			}

				Questo stampa Ho un int!.......

			Se si sta usando if con più pattern, la if si applica a entrambi i lati (dx e sx):

			let x = 4;
			let y = false;

			match x {
				//viene valutata l'espressione 4|5 e quindi stampa si
				//la if si applica al pattern dx e a quello sx
			    4 | 5 if y => println!("sì"),
			    _ => println!("no"),
			}

				Questo stampa no, perché la if si applica a tutta l'espressione 4 | 5, e non solamente al 5. In altre parole, la precedenza di if si comporta così:

					(4 | 5) if y => ...

					non così:

					4 | (5 if y) => ...

 	Mescolare e abbinare								4.15.8
 		Urca! Ci sono molti modi diversi di far combaciare le cose, e tutti quanti possono essere mescolati e abbinati, a seconda di ciò che si sta facendo:

			//match con due @pattern @mescolati ed @abbinati  todo: ???spiegare meglio 
 			match x {
 				//pattern dx =x  e pattern sx = y		
			    Foo { x: Some(ref nome), y: None } => ...
			}	

			I pattern sono molto potenti. Facciamone buon uso.

 SINTASSI DEI METODI									4.16.
 	
	
	Sintassi dei metodi									4.16.1
		Le funzioni sono ottime, ma se si vuole chiamarne un po' su alcuni dati, può diventare scomodo. Si consideri questo codice:

		//ORDINE DI CHIAMATA E INVERSO, prima 1) foo, 2) bar, 3) baz
		baz(bar(foo(x)));

			Normalmente leggiamo questo codice da sinistra a destra, e quindi diciamo ‘baz di bar di foo di x’. Ma questo non è l'ordine con cui le funzioni verrebbero chiamate; l'ordine di chiamata è invece il contrario: ‘applica a x prima foo, poi bar, e poi baz’. Non sarebbe carino se potessimo scrivere il seguente codice?

		x.foo().bar().baz();
			Fortunatamente, come si potrebbe immaginare, si può! Rust fornisce la capacità di usare questa ‘sintassi di chiamata di metodo’ tramite la parola-chiave impl.

	Chiamate di metodo									4.16.2
		Ecco come funziona:

			//creo una truttura e sua implementazione di cerchio
			//======================================================================//
					
				//STRUTTURA CERCHIO
				struct Cerchio {
				    x: f64,
				    y: f64,
				    raggio: f64,
				}

				//blocco impl con il metodo area
				impl Cerchio {
					//metodo area; con 1 argomento speciale con &self = riferimento immutabile
				    fn area(&self) -> f64 {    //CON &SELF = Prendi a prestito regola base al posto del possesso
				    	//calcolo del cerchio con la libreria std f64 const pi = raggio x raggio
				        std::f64::consts::PI * (self.raggio * self.raggio)
				    }
				}
			//======================================================================//

			//chiamo il cerchio con main
			fn main() {
					//assegno a c
				    let c = Cerchio { x: 0.0, y: 0.0, raggio: 2.0 };
				    println!("\n eseguo il calcolo Cerchio passando c con i 3 parametri x, y, raggio : {}", c.area());
				}


				Questo stamperà 12.566371.


				Abbiamo definito una struct che rappresenta un cerchio. Poi abbiamo scritto un blocco impl, e al suo interno abbiamo definito un metodo, area

				I metodi prendono un primo argomento speciale, di cui ci sono tre varianti: 
						self, 		= valore stack
						&self, e 	= riferimento (NON MUTABILE)
						&mut self  	= riferimento MUTABILE

				 Si può pensare a questo primo argomento come se fosse il foo in foo.bar(). 

				 Le tre varianti corrispondono ai tre tipi di cose che foo potrebbe essere: 
				 	self se è un valore sullo stack, 
				 	&self se è un riferimento, e 
				 	&mut self se è un riferimento mutabile. 

			 	Siccome abbiamo preso l'argomento &self da area, possiamo usarlo come qualunque altro argomento. 
			 	Siccome sappiamo che tale argomento è di tipo Cerchio, possiamo accedere al suo membro raggio come faremmo con qualunque altra struct.


			 	USARE &SELF COME REGOLA: 
				 	Di regola dovremmo usare &self, dato che dovremmo preferire prendere a prestito rispetto a prendere il possesso, e pure dovremmo preferire prendere un riferimenti immutabili rispetto a qulli mutabili. Ecco un esempio di tutte e tre le varianti:


				 		struct Cerchio {
							    x: f64,
							    y: f64,
							    raggio: f64,
							}

							impl Cerchio {
							    fn riferimento(&self) {  //&self = prendi a prestito ed è un riferimento 
							       println!("presa di sé per riferimento!");
							    }

							    fn riferimento_mutabile(&mut self) {  //&mut self = prendi a mutabile (è prestito o posesso ??)
							       println!("presa di sé per riferimento mutabile!");
							    }

						    fn prendi_possesso(self) {  //self nello stack e poi finisce con l'ambito delle {}
							}


						Si possono usare tanti blocchi impl quanti se ne vuole. L'esempio precedente poteva anche essere scritto così:

							//esempio di implementazione con 3 blocchi
							struct Cerchio {
							    x: f64,
							    y: f64,
							    raggio: f64,
							}
							//blocco cerchio con &self
							impl Cerchio {
							    fn riferimento(&self) {
							       println!("presa di sé per riferimento!");
							    }
							}

							//blocco cerchio con  &mut self = nella memoria heap ed è mutabile
							impl Cerchio {
							    fn riferimento_mutabile(&mut self) {  
							       println!("presa di sé per riferimento mutabile!");
							    }
							}

							//blocco cerchio con  solocon self = nello stack
							impl Cerchio {
							    fn prendi_possesso(self) {
							       println!("presa di possesso di sé!");
							    }
							}

	Concatenamento di chiamate di metodi				4.16.3
		Perciò, adesso sappiamo come chiamare un metodo, come foo.bar(). 
		E che dire del nostro esempio originale, x.foo().bar().baz()? 
		Questo è chiamato ‘concatenamento di metodi’. Vediamo un esempio:
			
			//CONCATENAMENTO DI CHIAMATE DI METODI
			 	//=====================================================================================//

					//faccio la struttura cerchio
						#[derive(Debug)]        //macro di debug per evitare errori
						#[allow(dead_code)]   //elimina il warning sul codice non usato
						struct Cerchio {
						    x: f64,
						    y: f64,
						    raggio: f64,
						}

						//implemento il cerchio con una funzione new (3 parametri x, y,raggio) --> restituisce Cerchio
						impl Cerchio {
						    fn new(x: f64, y: f64, raggio: f64) -> Cerchio {
						        Cerchio {
						            x: x,           //assegna valore 1 parametro
						            y: y,           //assegna valore 2 parametro
						            raggio: raggio, //assegna valore 3 parametro
						        }
						    }
						}

						fn main() {
						    //chiamo il metodo o la funzione statica con Cerchio::new che prende un argomento self (immutabile)
						    let c = Cerchio::new(0.0, 0.0, 2.0);
						    println!("ESEMPIO di concatenamento di metodi, area = {:?}", c);
						}

			 	//=====================================================================================//


			 	Verifica il tipo del valore restituito:
			 	//posso inserire un'altra funzione chiamata grow ...
				fn grow(&self, increment: f64) -> Cerchio {
				Diciamo che stiamo restituendo un Cerchio. Con questo metodo, possiamo far crescere un nuovo Cerchio a qualunque dimensione.




	Funzioni associate									4.16.4
		
			@metodi@di@classe_(sono funzione associate in una struct dette @funzioni@statiche )
		Si possono anche definire funzioni associate che non prendono un argomento self. 
		Ecco un pattern molto comune nel codice Rust:

			//faccio la struttura cerchio
			struct Cerchio {
				    x: f64,
				    y: f64,
				    raggio: f64,
				}

				//implemento il cerchio con una funzione new (3 parametri x, y,raggio) --> restituisce Cerchio
				impl Cerchio {
				    fn new(x: f64, y: f64, raggio: f64) -> Cerchio {
				        Cerchio {
				            x: x,				//assegna valore 1 parametro
				            y: y,				//assegna valore 2 parametro
				            raggio: raggio,		//assegna valore 3 parametro
				        }
				    }
				}

				fn main() {
							//chiamo il metodo o la funzione statica con Cerchio::new che prende un argomento self (immutabile)
				    let c = Cerchio::new(0.0, 0.0, 2.0);
				}

				Questa ‘funzione associata’ ci costruisce un nuovo Cerchio. 

				Si noti che le funzioni associate vengono chiamate usando la sintassi Struct::function(), invece che con la sintassi ref.method(). 
						@funzioni@statiche_(sono le funzione associate dette metodi di classe o @funzioni@associate)

				In alcuni altri linguaggi, le funzioni associate sono chiamate ‘funzioni membro statiche’ o ‘metodi statici’ o ‘metodi di classe’.


	Il pattern del costruttore							4.16.5
			Diciamo che vogliamo che i nostri utenti possano creare delle istanze di Cerchio, 
			ma permetteremo loro di impostare solamente le proprietà a cui sono interessati. 

			Se non specificati, gli attributi x e y varranno 0.0, e l'attributo raggio varrà 1.0. 

			Rust non ha il sovraccaricamento dei metodi, né gli argomenti con nome, né un numero variabile di argomenti. 

			Invece si impiega il pattern del costruttore. Si presenta così:


			//COME FARE UN COSTRUTTORE IN RUST
			 	//=====================================================================================//


					//struttura
					struct Cerchio {
						    x: f64,
						    y: f64,
						    raggio: f64,
						}


						//implementazione
						impl Cerchio {
							//calcolo AREA cerchio
						    fn area(&self) -> f64 {
						        std::f64::consts::PI * (self.raggio * self.raggio)
						    }
						}

						//costruttore di cerchi con 3 parametri x,y,raggio
						struct CostruttoreDiCerchi {
						    x: f64,
						    y: f64,
						    raggio: f64,
						}

						//CREO I METODI nel costruttore che permette di costruire il cerchio finale con la funzione finalizza
						impl CostruttoreDiCerchi {
						    fn new() -> CostruttoreDiCerchi {
						        CostruttoreDiCerchi { x: 0.0, y: 0.0, raggio: 1.0, }
						    }

						    // coordinata x 
						    fn x(&mut self, coordinata: f64) -> &mut CostruttoreDiCerchi {
						        self.x = coordinata;
						        self
						    }

						    // coordinata y

						    fn y(&mut self, coordinata: f64) -> &mut CostruttoreDiCerchi {
						        self.y = coordinata;
						        self
						    }

						    // I metodo il raggio
						    fn raggio(&mut self, raggio: f64) -> &mut CostruttoreDiCerchi {
						        self.raggio = raggio;
						        self
						    }

						    // II metodo finalizza
						    fn finalizza(&self) -> Cerchio {
						    	println!("\n 
						    mi trovo nel METODO FINALIZZA e chiamo la classe cerchio 
						    passando i valori di   x: {},  	\n
						    passando i valori di   y: {} e 	\n
						    passando il valore del raggio {}  \n", self.x, self.y, self.raggio);

						        Cerchio { x: self.x, y: self.y, raggio: self.raggio }  //chiamo cerchio passando 3 paramtri
						    }
						}

						fn main() {
							//Chiamo il costruttore dell'istanza di cerchi con new
						    let c = CostruttoreDiCerchi::new()		
						        .x(1.0)				//passo la coordinata x
						        .y(2.0)				//passo la coordinata y
						        .raggio(2.0)		//passo i valori del raggio
						        .finalizza();		//costruisco il cerchio

						    println!("
						    calcolo area: {}", c.area());     //CHIAMO il metodo area per il calcolo del cerchio raggio x raggio
						    println!("
						    valore di  x: {}", c.x);
						    println!("  
						    valore di  y: {}", c.y);
						    }

 					//=====================================================================================//

 		    STRUTTURA COSTRUTTORI CERCHI			
 			Ciò che abbiamo fatto qui è creare un'altra struct, CostruttoreDiCerchi. 

 					Su di essa abbiamo definito i nostri metodi di costruttore. 

 					Abbiamo anche definito il nostro metodo area() su Cerchio. 
 					Inoltre abbiamo creato un altro metodo su CostruttoreDiCerchi: 
 						finalizza(). 
					Questo metodo crea il nostro Cerchio finale dal costruttore. Adesso, abbiamo usato il sistema dei tipi per imporre le nostre intenzioni: possiamo usare i metodi su CostruttoreDiCerchi per vincolare la costruzione di istanze di Cerchio in qualunque modo desideriamo.


  STRINGHE												4.17.
  	Le stringhe											4.17.1
  		Le stringhe sono un concetto di cui è importante che ogni programmatore si impadronisca. Il sistema di gestione delle stringhe di Rust è un po' diverso da quello degli altri linguaggi, a causa del suo incentrarsi sulla programmazione di sistema. Ogni volta che c'è una struttura dati di dimensione variabile, le cose possono complicarsi, e le stringhe sono una struttura dati ridimensionabile. Detto questo, le stringhe di Rust funzionano diversamente anche da alcuni altri linguaggi di sistema, come il C.

		Scaviamo nei dettagli. Una ‘stringa’ è una sequenza di valori scalari Unicode codificati come flusso di byte UTF-8. Tutte le stringhe sono garantite essere una ecodifica valida di sequenze UTF-8. In aggiunta, diversamente da alcuni linguaggi di sistema, le stringhe non hanno un carattere terminatore e possono contenere il carattere NUL, rappresentato dal byte 0.
			@stringa_(sequenza di valori scalari Unicode UTF-8 senza terminatore ma hanno @nul)

		Rust ha due principali tipi di stringhe: 
			&str 		= chiamate slice di stringa (a dimensione fissa ed una sequenza di byte UTF-8)
				e 
			String. 	= stringa variabile sull'Heap

		Dapprima parliamo di &str. Queste sono chiamate ‘slice di stringa’. Una slice di stringa ha una dimensione fissa, e non può essere modificata. È un riferimento a una sequenza di byte UTF-8.

			let saluto = "Ciao là."; // saluto: &'static str

		"Ciao là." è un letterale di stringa il cui tipo è &'static str. 

				@&str_(slice di stringa allocato staticamente nel programma compilato)

		Un letterale di stringa è uno slice di stringa che è allocato staticamente, il che significa che è salvato dentro il nostro programma compilato, ed esiste per l'intera durata dell'esecuzione. Il legamo saluto è un riferimento a questa stringa staticamente allocata. Qualunque funzione che si aspetta una slice di stringa accetterà anche un letterale di stringa.


		SI ESTENDONO SU PIU RIGHE: I letterali di stringa possono estendersi su più righe. Ce ne sono due forme. La prima includerà i caratteri a-capo e gli spazi che li seguono:

			I° 	FORMA
				//qui si sono gli accapo e gli spazi
				let s = "foo
    				bar";

				assert_eq!("foo\n    bar", s);

			II° FORMA
				La seconda, con un \, rimuove gli a-capo e gli spazi che li seguono:

				// attenzione se metta la \ vengono rimossi gli spazi e gli accapo seguenti
				let s = "foo\
				    bar";

				assert_eq!("foobar", s);

			TIPO NON DIMENSIONATO : Si noti che normalmente non si può accedere direttamente a una str, ma solamente tramite un riferimento &str. Questo perché str è un tipo non dimensionato che richiede informazioni aggiuntive in fase di esecuzione per poter essere usata. Per avere maggiori informazioni, si veda il capitolo sui tipi non dimensionati.

			STRING MEMORIZZATO SULL'HEAP ED E' ESTENSIBILE = Però Rust ha di più oltre alle &str. Una String è una stringa allocata sullo heap. Questa string è estendibile, ed è anche garantita essere UTF-8. 
			
			METODO to_String CONVERTE UNA SLICE DI STRING = Le String tipicamente sono create convertendo una slice di stringa, usando il metodo to_string.
				@Metodo@to_String_(converte le slice di stringa da str a String)

			Le String vengono forzate ad essere un &str usando un &:

					//&str = Forzatura di String che viene riconvertita in str
				fn prendi_slice(slice: &str) {
				    println!("Preso: {}", slice);
				}

				fn main() {
				    let s = "Ciao".to_string();  //to_string converte una slice di stringa in String
				    prendi_slice(&s);
				}

			CONVERSIONE OBBLIGATORIA CON &* = Questa forzatura non avviene per le funzioni che accettano uno dei tratti di &str invece di &str stessa?. Per esempio, 
				TcpStream::connect ha un argomento di tipo ToSocketAddrs. 

				Una &str va bene, ma una String deve essere esplicitamente convertita usando &*.

				use std::net::TcpStream;

				TcpStream::connect("192.168.0.1:3000"); // argomento di tipo &str

				let stringa_indirizzo = "192.168.0.1:3000".to_string();
				TcpStream::connect(&*stringa_indirizzo); 

			COSTO DELLA CONVERSIONE = Vedere una String come una &str costa poco, ma convertire la &str in una String comporta allocare della memoria. Non c'è ragione di farlo, a meno che sia necessario!


 	Indicizzazione										4.17.2

 			@indice@stringa_(vietato perchè costoso e la stringa puo essere anche punti)

 		NO INDICIZZAZIONE = Siccome le stringhe sono UTF-8 valide, non supportano l'indicizzazione:
 			let s = "ciao";
			println!("La prima lettera di s è {}", s[0]); // ERRORE!!!		

		Solitamente, l'accesso a un vettore con [] è molto veloce. Ma, siccome ogni carattere una stringa codificata in UTF-8 può occupare più byte, si deve percorrere la stringa per trovare l'ennesima lettera di una stringa. Questa è un'operazione significativamente più costosa, e non si vuole essere fuorvianti. 
		Inoltre, il concetto di ‘lettera’ non è qualcosa di ben definito in Unicode. Possiamo scegliere di guardare una stringa come una sequenza di singoli byte, o come punti di codice ["codepoint"]:

		let hachiko = "忠犬ハチ公";

			for b in hachiko.as_bytes() {
			    print!("{}, ", b);
			}

			println!("");

			for c in hachiko.chars() {
			    print!("{}, ", c);
			}

			println!("");

			Questo stampa:

			229, 191, 160, 231, 138, 172, 227, 131, 143, 227, 131, 129, 229, 133, 172,
			忠, 犬, ハ, チ, 公,

			
			Si può ottenere qualcosa di simile a un indice in questo modo:

			let dog = hachiko.chars().nth(1); // un po' come hachiko[1]
			Questo evidenzia che dobbiamo percorrere la lista di char dall'inizio.





 	Affettatura ["slicing"]								4.17.3

 			@slicing_(come @affettare una stringa @offset in caratteri non in @byte)

 		Si può ottenere una slice di una stringa con la sintassi dell'affettatura:

 			fn main() {
			    let dog = "hachiko";    	//stringa
				let hachi = &dog[0..5];     //slicing stringa ma non andare fuori indice : NO 0...6 FUORI INDICE
				print!("slicing di string è ammesso da 0..5= {}, ", hachi);

				}

		Ma si noti che questi sono offset in byte, non offset in caratteri. Perciò questo fallirà in fase di esecuzione:
			@slicing@byte_(in byte non avviene lo slicing ma da errore)

			//questi sono  byte non stringhe 
			let dog = "忠犬ハチ公";
			let hachi = &dog[0..2];			//slicing errore perche in byte

		con questo errore:

			thread 'main' panicked at 'index 0 and/or 2 in `忠犬ハチ公` do not lie on
			character boundary'

 	Concatenazione										4.17.4

 		Avendo una String, si può concatenare una &str alla sua fine:

 			fn main() {
				//esempio CON UNA STRING
				let ciao = "Ciao ".to_string();   //la String che viene deferenziata in &string
				let mondo = "mondo!";

				//puoi concatenare le due string
				let ciao_mondo = ciao + mondo;
				print!("concateno ciao + mondo = {}, ", ciao_mondo);
				}



			Ma se avendo due String, serve un &:

				fn main() {
				    let ciao = "Ciao ".to_string();  //QUI ABBIAMO LA I° STRING 
				let mondo = "mondo!".to_string();     //QUI ABBIAMO LA  II° STRING 

				let ciao_mondo = ciao + &mondo;     //DEVI DEFERENZIARE LA STRING con &string
				print!("concateno ciao + mondo  = {}, ", ciao_mondo);
				}

			Questo perché &String può essere automaticamente forzata in un &str. 
			Questa caratteristica si chiama ‘forzatura Deref’.

 GENERICITÀ												4.18.
 	I generici											4.18.1

 		POLIMORFISMO PARAMETRICO
	 		Talvolta, quando si scrive una funzione o un tipo di dati, potremmo volere che funzioni per più tipi di argomenti. In Rust, lo possiamo fare usando i generici. Nella teoria dei tipi, i generici sono chiamati ‘polimorfismo parametrico’, che significa che sono tipi o funzioni che hanno più forme (in greco ‘poli’ significa ‘plurimo‘, e ‘morfo’ significa ‘forma‘) in base a un dato parametro (da cui ‘parametrico’).

	 		Comunque, basta così con la teoria dei tipi, guardiamo del codice generico. La libreria standard di Rust fornisce un tipo, Option<T>, che è generico:

	 			@Tipo@generico_(argomenti con @tipi@generici, @Option<T> quindu UN GENERICO CON UN SOLO TIPO)

	 			//TIPO GENERICO SINGOLO
	 			enum Option<T> {
					    Some(T),
					    None,
					}

			
					@tipo@generico_(viene indicata con <T> il tipo generico @singolo)

			La parte <T>, che abbiamo già visto alcune volte, indica che questo è un tipo di dati generico. Ogni volta che nel nostro codice usiamo questo enum, specifichiamo un tipo che sostituisce il parametro T ogni volta che compare nella dichiarazione generica. Ecco un esempio di uso di Option<T>, con un'annotazione di tipo aggiuntiva:


				//SX E DX LE dichiarazioni combaciano in quanto sul lato sx Option<T> = è di tipo i32
				//												sul lato sx Some = 5  indica che Some(T) è intero di tipo i32


				let x: Option<i32> = Some(5);

				Nella dichiarazione di tipo, diciamo Option<i32>. 
				Si noti quanto simile appaia a Option<T>. 

				Quindi, in questa particolare Option, T ha il valore di i32. Sul lato destro del legame, costruiamo un Some(T), dove l'oggetto di tipo T è 5. 

				Dato che si tratta di un i32, i due lati combaciano, e Rust è contento. 

				Se non combaciassero, otterremmo un errore:


				//errore perche due tipi diversi di <T>
				let x: Option<f64> = Some(5);

				// error: mismatched types: expected `core::option::Option<f64>`,
				// found `core::option::Option<_>` (expected f64 but found integral variable)



				ATTENZIONE UGUAGLIANZA SOLO NEI DUE LATI MA OTPION DIVERSI


				Ciò non significa che non si possono costruire degli Option<T> che tengono f64! Solamente i due lati dell'assegnamento devono avere lo stesso tipo:

					//OTPION <T> diversi ma con lati uguali

					let x: Option<i32> = Some(5);			//i32
					let y: Option<f64> = Some(5.0f64);		//f64


				Così va bene. Una sola definizione, utilizzi multipli.


				GENERICI CON DUE TIPI

					I generici non sono limitati ad essere parametrizzati da un solo tipo. Si consideri un altro tipo simile, fornito dalla liberia standard di Rust, Result<T, E>:

						//TIPO GENERICO CON DUE TIPI 
						enum Result<T, E> {
									    Ok(T),
									    Err(E),
									}


					Questo tipo è generico relativamente a due tipi: T ed E. Tra l'altro, le lettere maiuscole possono essere qualunque lettera si gradisca. Si sarebbe potuto definire Result<T, E> come:

							//TIPO GENERICO DOPPIO
							enum Result<A, Z> {
							    Ok(A),
							    Err(Z),
							}

					se si avesse voluto. 

					Per convenzione il primo parametro generico dovrebbe essere T, per ‘tipo’, e si dovrebbe usare E per ‘errore’. Però a Rust non importa.

						@TIPI@GENERICI@DOPPI_(come il Result per con due tipi T,E per restituire un errore in caso di fallimento dell'elaborazione)

					Il tipo Result<T, E> è pensato per essere usato come risultato di un'elaborazione, dando la possibilità di rendere un errore nel caso non si riuscisse a completare correttamente l'elaborazione.

 	Funzioni generiche									4.18.2
 		
 			@funzione@generica@un@tipo_(funzione con UN TIPO T)

 		Si possono scrivere funzioni che prendono tipi generici, usando una sintassi simile:

 			
 			//@funzione@generica
 			fn prende_qualunque_cosa<T>(x: T) {
 			   // fai qualcosa con x
				}

				La sintassi ha due parti: 
					la <T> dice “questa funzione è generica rispetto a un tipo, T”, e la 
					x: T dice “x è di tipo T.”


			@funzione@generica@DUE@STESSO@tipo_(funzione con DUE TIPO T O PIU ARGOMENTI)

				Più argomenti possono essere dello stesso tipo generico:

					//TIPO DI CON DUE PARAMETRI OGGETTI DELLO STESSO TIPO X,Y
					fn prende_due_oggetti_del_medesimo_tipo<T>(x: T, y: T) {
   						 // ...
							}



				Si può anche scrivere una versione che prende più tipi:
					
					@funzione@generica@DUE@tipI_(funzione con DUE @TIPO@T)

					@funzione@generica@DUE@OGGETTI@DIVERSI_(funzione con @DUE@OGGETTI@DIVERSI DI TIPO T,U)

				//DUE OGGETTI CON TIPO DIVERSO T,U
				fn prende_due_oggetti<T, U>(x: T, 
					                        y: U) {
						    // ...
						}

	Struct generiche									4.18.3

			@TIPI@GENERICI@struct_(I TIPI GENERICI SI APPLICANO  @campi della struct )

				Si può usare un tipo generico anche per i campi di una struct:

					@definizione@struttura@generica_(definita con parametro di tipo <T>)

					//NELLA DEFINIZIONE DELLA STRUTTURA GENERICA il parametro è di tipo T con due campi di tipo T

					struct Punto<T> {
					    x: T,
					    y: T,
					}

					// un tipo T = intero + secondo tipo T = virgola mobile
					let origine_intera = Punto { x: 0, y: 0 };
					let origine_a_virgola_mobile = Punto { x: 0.0, y: 0.0 };


				IMPLEMENTAZIONE DELLA STRUTTURA GENERICA
				Quando si vuole aggiungere un'implementazione per una struct generica, si dichiara il parametro di tipo subito dopo la impl:

				//Implementazione tipo T + parametro di tipo T (Punto <T>)
				impl<T> Punto<T> {
					    fn swap(&mut self) {
					        std::mem::swap(&mut self.x, &mut self.y);
					    }
					}


				Finora abbiamo visto dei generici che prendono assolutamente qualunque tipo. 

				Questi servono in molti casi: 
					abbiamo già visto Option<T>, 
					e poi incontreremo i tipi contenitore universali, come Vec<T>. 

				D'altra parte, spesso si vuole rinunciare a quella flessibilità per avere un maggior poter espressivo. Si legga la sezione sui legami di tratto per vedere come e perché.

 TRATTI													4.19.
 	Tratti ["trait"]									4.19.1
 		trait
 			Un tratto è una caratteristica del linguaggio che dice al compilatore Rust quali funzionalità un tipo deve fornire.
 					@tratto_(@trait= @classe@astratta o una @firma che permette di definire le funzionalita di un tipo)

 			Ricordiamo la parola-chiave impl, usata per chiamare una funzione con la sintassi dei metodi:

 				
 				//differenza tra la struttura implementata ed il tratto; questa è una struttura
 				//struttura
 				struct Cerchio {
					    x: f64,
					    y: f64,
					    raggio: f64,
					}

					//implementa la struttura
					//impl = parola chiave utilizzata per chiamare una funzione con sintassi dei metodi
					//impl contiene il metodo non la firma
					impl Cerchio {			//impl Item
					    fn area(&self) -> f64 {
					        std::f64::consts::PI * (self.raggio * self.raggio)
					    }
					}


				//qui abbiamo un tratto che si differenza dalla struttura perchè prima la firma dopo l'implemetazione

				//struttura cerchio uguale
					struct Cerchio {
					    x: f64,
					    y: f64,
					    raggio: f64,
					}


			//firma del tratto ed implementazione
			//---------------------------------------------------------------------------//
				//prima la firma = blocco trait
				trait HaArea {
					    fn area(&self) -> f64;		//firma del tipo senza implementazione
					}

					//implementazione del tratto  = blocco impl 
					impl HaArea for Cerchio {		//impl Trait for Iterm
					    fn area(&self) -> f64 {
					        std::f64::consts::PI * (self.raggio * self.raggio)
					    }
					}
			//---------------------------------------------------------------------------//

				Come si può vedere, 
						il blocco trait appare molto simile al 
						blocco impl, 
				ma non contiene il corpo della funzione, solamente una firma di tipo. 

				Quando si implementa un tratto, si usa la formula 
					impl Trait for Item, 
					invece della più semplice impl Item.








 	Legami del tratto sulle funzioni generiche			4.19.2
 	
 		IMPORTANZA DEI TRATTI
 			I tratti sono utili perché consentono a un tipo di fare certe PROMESSE SUL SUO COMPORTAMENTO. 

 			Le funzioni generiche possono sfruttare questo (I TRAIT) per vincolare, o legare, i tipi che accettano. 

 			Si consideri questa funzione, che non compila:

 			 	//FUNZIONE GENERICA che utilizza una figura generica ma non c'è il tipo generico
 			 	fn stampa_area<T>(figura: T) {  //rust da errore perche non c'è il tipo generico
					    println!("Questa figura ha un'area di {}", figura.area());
					}

			Siccome T può essere qualunque tipo, non possiamo essere sicuri che implementi il metodo area. Ma possiamo aggiungere un tratto legato al nostro T generico, assicurando che lo faccia:

			
			//tratto che definisce: il TIPO GENERICO = <T: HaArea>
			//+ LA FIGURA GENERICA --> (figura: T)
			fn stampa_area<T: HaArea>(figura: T) {
				    println!("Questa figura ha un'area di {}", figura.area());
				}


				La sintassi <T: HaArea> significa 
					“qualunque tipo che implementa il tratto HaArea.” 

				Siccome i tratti definiscono delle firme di tipo di funzione, possiamo star sicuri che qualunque tipo che implementa HaArea avrà un metodo .area().


			Ecco un esempio esteso di come funziona questa cosa:


				//TRATTO
				trait HaArea {
    					fn area(&self) -> f64;
					}


				//Struttura ed implementazione del Cerchio		
				//...........................................................................//	
					struct Cerchio {
					    x: f64,
					    y: f64,
					    raggio: f64,
					}

					impl HaArea for Cerchio {
					    fn area(&self) -> f64 {
					        std::f64::consts::PI * (self.raggio * self.raggio)
					    }
					}
				//...........................................................................//	



				//Struttura ed implementazione del Quadrato		
				//...........................................................................//	
					struct Quadrato {
					    x: f64,
					    y: f64,
					    lato: f64,
					}

					impl HaArea for Quadrato {
					    fn area(&self) -> f64 {
					        self.lato * self.lato
					    }
					}
				//...........................................................................//	
				
				//funzione stampa che puo usare DUE TIPI DIVERSI, stringo o f64 per la stampa
				//...........................................................................//	

					//funzione che permette di stapare tipi generici MA PER ORA CON GLI STESSI DATI
					//f64 altrimenti da errore
					fn stampa_area<T: HaArea>(figura: T) {
						    println!("Questa figura ha un'area di {}", figura.area());
						}
				//...........................................................................//


				fn main() {
				    let c = Cerchio {
				        x: 0.0f64,
				        y: 0.0f64,
				        raggio: 1.0f64,
				    };

				    let q = Quadrato {
				        x: 0.0f64,
				        y: 0.0f64,
				        lato: 1.0f64,
				    };


				Questo programma emette:

				Questa figura ha un'area di 3.141593
				Questa figura ha un'area di 1


				Da come si vede, stampa_area adesso è generica, ma assicura anche che le abbiamo passato i tipi corretti (solo f64). Se le passiamo un tipo scorretto:

				//parametro intero da errore
				stampa_area(5);
					Otteniamo un errore in fase di compilazione:

					error: the trait bound `_ : HasArea` is not satisfied [E0277]


 	
 		Finora, abbiamo aggiunto implementazioni di tratti solamente a delle struct, ma un tratto può essere implementato per qualunque tipo. 

 		Perciò tecnicamente, potremmo implementare HaArea anche per il tipo i32:

 			trait HaArea {
				    fn area(&self) -> f64;
				}

				impl HaArea for i32 {
				    fn area(&self) -> f64 {
				        println!("questo è sciocco")
				        *self as f64
				    }
				}

				5.area();

			È considerato stile scadente implementare dei metodi su tali tipi primitivi, anche se è possibile.

			Questo può sembrare come il Far West, ma ci sono due restrizioni riguardo l'implementazione dei tratti che prevengono che la cosa ci sfugga di mano. 
				La prima è che se il tratto non è definito nel nostro ambito, non si applica. 

			Ecco un esempio: la libreria standard fornisce un tratto Write che aggiunge delle funzionalità ai File, per fare I/O su file. Di default, un File non avrà i suoi metodi:


					//qui write da errore perche non è importata la libreria
					let mut f = std::fs::File::open("foo.txt").expect("Fallita apertura di foo.txt");
					let buf = b"qualcosa"; // stringa letterale di byte. buf: &[u8; 8					let risultato = f.write(buf);
					Ecco l'errore:

					error: type `std::fs::File` does not implement any method in scope named `write`
					let result = f.write(buf);


			Dapprima dobbiamo importare il tratto Write con use:

				//qui funzione perche viene importata la libreria
				use std::io::Write;

					let mut f = std::fs::File::open("foo.txt").expect("Fallita apertura di foo.txt");
					let buf = b"qualcosa"; // stringa letterale di byte. buf: &[u8; 8]
					let risultato = f.write(buf);
					Questo compilerà senza errori.


			Ciò significa che anche se qualcuno fa qualcosa di male come implementare un tratto per i32, questo non ci toccherà, a meno che importiamo quel tratto.

			C'è un'altra restrizione sull'implementare i tratti: o il tratto o il tipo per cui lo stiamo implementando, devono essere definiti da noi. 
			O per meglio dire, almeno uno di essi deve essere definito nello stesso crate in cui si trova l'impl che stiamo scrivendo. Per saperne di più sul sistema dei moduli e dei pacchetti di Rust, si veda la sezione su crate e moduli.

			Perciò, potremmo implementare il tratto HasArea per i32, dato che abbiamo definito HaArea nel nostro codice. 
			Ma se provassimo a implementare ToString, un tratto fornito da Rust, per i32, non potremmo, perché né il tratto né il tipo sono definiti nel nostro crate.

			Un'ultima cosa sui tratti: le funzioni generiche con un legame di tratto usano la ‘monomorfizzazione’ (dal greco "mono"="uno" e "morfo"="forma"), e quindi sono smistati staticamente. Che significa? Si guardi la sezione sugli oggetti-tratto per avere maggiori dettagli.











   Regole per implementare i tratti				4.19.4
   	
	 		Finora, abbiamo aggiunto implementazioni di tratti solamente a delle struct, ma un tratto può essere implementato per qualunque tipo. 

			Perciò tecnicamente, potremmo implementare HaArea anche per il tipo i32:

				trait HaArea {
				    fn area(&self) -> f64;
				}

				impl HaArea for i32 {
				    fn area(&self) -> f64 {
				        println!("questo è sciocco")
				        *self as f64
				    }
				}

				5.area();

		È considerato stile scadente implementare dei metodi su tali tipi primitivi, anche se è possibile.

		Questo può sembrare come il Far West, ma ci sono due restrizioni riguardo l'implementazione dei tratti che prevengono che la cosa ci sfugga di mano. 
			La prima è che se il tratto non è definito nel nostro ambito, non si applica. 

		Ecco un esempio: la libreria standard fornisce un tratto Write che aggiunge delle funzionalità ai File, per fare I/O su file. Di default, un File non avrà i suoi metodi:


				//qui write da errore perche non è importata la libreria
				let mut f = std::fs::File::open("foo.txt").expect("Fallita apertura di foo.txt");
				let buf = b"qualcosa"; // stringa letterale di byte. buf: &[u8; 8					let risultato = f.write(buf);
				Ecco l'errore:

				error: type `std::fs::File` does not implement any method in scope named `write`
				let result = f.write(buf);


		Dapprima dobbiamo importare il tratto Write con use:

			//qui funzione perche viene importata la libreria
			use std::io::Write;

				let mut f = std::fs::File::open("foo.txt").expect("Fallita apertura di foo.txt");
				let buf = b"qualcosa"; // stringa letterale di byte. buf: &[u8; 8]
				let risultato = f.write(buf);
				Questo compilerà senza errori.


		Ciò significa che anche se qualcuno fa qualcosa di male come implementare un tratto per i32, questo non ci toccherà, a meno che importiamo quel tratto.

		C'è un'altra restrizione sull'implementare i tratti: o il tratto o il tipo per cui lo stiamo implementando, devono essere definiti da noi. 
		O per meglio dire, almeno uno di essi deve essere definito nello stesso crate in cui si trova l'impl che stiamo scrivendo. Per saperne di più sul sistema dei moduli e dei pacchetti di Rust, si veda la sezione su crate e moduli.

		Perciò, potremmo implementare il tratto HasArea per i32, dato che abbiamo definito HaArea nel nostro codice. 
		Ma se provassimo a implementare ToString, un tratto fornito da Rust, per i32, non potremmo, perché né il tratto né il tipo sono definiti nel nostro crate.

		Un'ultima cosa sui tratti: le funzioni generiche con un legame di tratto usano la ‘monomorfizzazione’ (dal greco "mono"="uno" e "morfo"="forma"), e quindi sono smistati staticamente. Che significa? Si guardi la sezione sugli oggetti-tratto per avere maggiori dettagli.


 	Legami di tratto multipli						4.19.5
 		Abbiamo visto che si può legare un parametro generico di tipo a un tratto:

			fn foo<T: Clone>(x: T) {
			    x.clone();
			}


		Se serve più di un legame, si può usare +:

			use std::fmt::Debug;

			//2 tratti 
			fn foo<T: Clone + Debug>(x: T) {
			    x.clone();
			    println!("{:?}", x);
			}
		T adesso ha bisogno di essere sia Clone che Debug.



 	La clausola Where									4.19.6
 		Scrivere funzioni con solamente alcuni tipi generici e un piccolo numero di legami di tratto non è malaccio, ma man mano che il loro numero si accresce, la sintassi divenga sempre più goffa:

 			use std::fmt::Debug;
 			//nome funzione 		= foo
 			//lisata argomenti 	= <T: Clone, K: Clone + Debug>(x: T, y: K)
				fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
				    x.clone();
				    y.clone();
				    println!("{:?}", y);
				}


			Rust ha una soluzione, e si chiama ‘clausola where’:

				use std::fmt::Debug;

						fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
						    x.clone();
						    y.clone();
						    println!("{:?}", y);
						}

						// uso la clausola where
						fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
						    x.clone();
						    y.clone();
						    println!("{:?}", y);
						}

						fn main() {
						    foo("Ciao", "mondo");
						    bar("Ciao", "mondo");
						}
			Si devono solo omettere i vincoli quando si definiscono i propri parametri di tipo, e poi aggiungere where dopo l'elenco degli argomenti. Per liste più lunghe, si possono aggiungere spaziature:

				use std::fmt::Debug;

					fn bar<T, K>(x: T, y: K)
					    where T: Clone,
					          K: Clone + Debug {

					    x.clone();
					    y.clone();
					    println!("{:?}", y);
					}

			Questa flessibilità può aumentare la chiarezza in situazioni complesse.

			where è anche più potente della sintassi più semplice. Per esempio:

				trait ConvertiIn<Output> {
					    fn converti(&self) -> Output;
					}

					impl ConvertiIn<i64> for i32 {
					    fn converti(&self) -> i64 { *self as i64 }
					}

					// si può chiamare con T == i32
					fn normale<T: ConvertiIn<i64>>(x: &T) -> i64 {
					    x.converti()
					}

					// si può chiamare con T == i64
					fn inversa<T>(x: i32) -> T
					        // sta usando ConvertiIn come se fosse "ConvertiIn<i64>"
					        where i32: ConvertiIn<T> {
					    x.converti()
					}

			Questo codice esibisce la caratteristica aggiuntiva delle clausole where: tali clausole consentono legami sul lato sinistro, non solamente dei parametri di tipo, ma anche dei tipi (in questo caso, il tipo i32). In questo esempio, i32 deve implementare ConvertiIn<T>. Invece di definire cos'è i32 (dato che è ovvio), qui la clausola where vincola T.





 	I metodi di default								4.19.7
 			
 			Un metodo di default può essere aggiunto a una definizione di tratto se è già noto come un implementatore tipico definirà un metodo. 

 			Per esempio, e_invalido() è definito come l'oppost di e_valido():

 					trait Foo {
					    fn e_valido(&self) -> bool;

					    fn e_invalido(&self) -> bool { !self.e_valido() }
					}

			Gli implementatori del tratto Foo devono implementare e_valido(), ma possono non implementare e_invalido(), dato che ha già un comportamento di default. Questo comportamento di default può sempre essere scavalcato, come in:

					
					struct UsaDefault;

						impl Foo for UsaDefault {
						    fn e_valido(&self) -> bool {
						        println!("Chiamato UsaDefault.e_valido.");
						        true
						    }
						}

						struct ScavalcaDefault;

						impl Foo for ScavalcaDefault {
						    fn e_valido(&self) -> bool {
						        println!("Chiamato ScavalcaDefault.e_valido.");
						        true
						    }

						    fn e_invalido(&self) -> bool {
						        println!("Chiamato ScavalcaDefault.e_invalido!");
						        true // Scavalca il valore atteso di e_invalido()
						    }
						}

						let default = UsaDefault;
						assert!(!default.e_invalido()); // stampa "Chiamato UsaDefault.e_valido."

						let scavalca = ScavalcaDefault;
						assert!(scavalca.e_invalido()); // stampa "Chiamato ScavalcaDefault.e_invalido!"																				
 	Ereditarietà										4.19.8

		Ereditarietà
			Talvolta, implementare un tratto richiede implentare un altro tratto:

				trait Foo {
				    fn foo(&self);
				}

				trait FooBar : Foo {  //FooBar implementa anche Foo = ereditarieta
				    fn foobar(&self);
				}


		Gli implementatori di FooBar devono implementare anche Foo, così:

			struct Baz;

				impl Foo for Baz {
				    fn foo(&self) { println!("foo"); }
				}

				impl FooBar for Baz {	//FooBar implementa solo Baz e non Foo per cui da errore
				    fn foobar(&self) { println!("foobar"); }
				}


				Se tralasciamo di implementare Foo, Rust ce lo dirà:
					error: the trait bound `main::Baz : main::Foo` is not satisfied [E0277]



 	Derivazione											4.19.9

 		ATTRIBUTO per l'implementazione automadica dei tratti = Implementare ripetutamente i tratti come Debug e Default può diventare parecchio noioso. Per tale ragione, Rust fornisce un attributo che consente di far implementare automaticamente dei tratti a Rust:
 				#[derive(Debug)]
					struct Foo;

					fn main() {
					    println!("{:?}", Foo);
					}

		DERIVAZIONE LIMITATA = Però, la derivazione è limitata a un certo insieme di tratti. Eccoli:
				@derivazione@tratti_(@attributi per l'implementazione atomatica dei tratti)

				Clone
				Copy
				Debug
				Default
				Eq
				Hash
				Ord
				PartialEq
				PartialOrd

 DROP													4.20.
 	Distruzione ["Drop"]								4.20.1

 			@tratto_@Drop_(tratto di rust che svolge da distruttore utilizzato nelle struct)

 		Adesso che abbiamo parlato dei tratti, parliamo di un particolare tratto fornito dalla libreria standard di Rust, Drop, che letteralmente significa "lasciar cadere", ma qui va inteso nel senso di "distruggere". Il tratto Drop svolge la funzione che in altri linguaggi di programmazione è svolta dai "distruttori"; infatti fornisce un modo di eseguire del codice quando un oggetto esce dal suo ambito.

 			Per esempio:

 				struct DaDistruggere;

					impl Drop for DaDistruggere {
					    fn drop(&mut self) {
					        println!("Distruzione!");
					    }
					}

					fn main() {
					    let x = DaDistruggere;
					    
					    // fai qualcosa

					} // qui x esce di ambito e stampa "Distruzione!"


			Quando x esce di ambito, alla fine della funzione main(), il metodo drop verrà eseguito. Tale metodo, che è l'unico di Drop, prende un riferimento mutabile a self.

			Ecco fatto! La meccanica di Drop è molto semplice, ma ci sono alcune sottigliezze. Per esempio, i valori vengono distrutti nell'ordine opposto a quello in cui sono stati dichiarati. Ecco un altro esempio:

				struct Esplosivo {
					    energia: i32,
					}

					impl Drop for Esplosivo {
					    fn drop(&mut self) {
					        println!("BUM per {}!", self.energia);
					    }
					}

					fn main() {
					    let petardo = Esplosivo { energia: 1 };
					    let granata = Esplosivo { energia: 100 };
					    let bomba = Esplosivo { energia: 1000 };
					}

				Questo programma stamperà:

					BUM per 1000!
					BUM per 100!
					BUM per 1! 	

				La bomba esplode prima della granata, che esplode prima del petardo, perché sono stati dichiarati in quell'ordine invertito. Ultimo dentro, primo fuori.

				Dunque a cosa serve il tratto Drop? In generale, Drop viene usato per distruggere le risorse associate a una struct. Per esempio, Arc<T> è un tipo che contiene un oggetto di tipo generico T e un conteggio di riferimenti. Quando viene chiamato il suo metodo drop, questo metodo decrementa il conteggio dei riferimenti, e se tale conteggio è diventato zero, dealloca l'oggetto contenuto.

 IF LET 												4.21.

 	COSTRUTTO IF LET 									4.21.1
		 	Il costrutto if let consente di combinare i costrutti if e let, per migliorare efficienza di certi tipi di pattern match.
		 	Per esempio, diciamo che abbiamo un certo Option<T>. Vogliamo chiamare una funzione su di esso, ma solo se vale Some<T>, e non fare niente se vale None. Si potrebbe fare così:

		 		//soluzione I non soddisfacente
		 		match opzione {
					    Some(x) => { foo(x) },	
					    None => {},
					}

				Qui non è necessario usare un match, perché potremmo anche usare un if:

		 			//soluzione II non soddisfacente
					if opzione.is_some() {  //se some è valido utilizzo if
					    foo(opzione.unwrap());


			    Però nessuna delle due soluzioni è particolarmente soddisfacente. Invece si può usare un if let per fare la stessa cosa in un modo più carino:

		 			//@if_(if che migliora l'efficienza di pattern le utilizzazione @some )
			    	if let Some(x) = opzione {
						    foo(x);
						}

					Se un pattern combacia con successo, lega tutte le parti appropriate del valore agli identificatori del pattern, e poi valuta l'espressione. Se il pattern non combacia, non succede niente.

							esempio completo


									fn main() {
									    let opzione = Some(5);
									fn foo(x: i32) { 
									         println!("\n
										     CHIAMATA FOO ATTIVA : esempio di if let che permette di utilizzare Option T 
										     solo se Some T è valido. E' combaciato il pattern let Some (x) = opzione
										     la funzione foo HA RICEVUTO X CHE VIENE STAMPATO = {}",x);
									}
									fn bar() { }
									if let Some(x) = opzione {
									     println!("\n\n
									    Se combacia il pattern: let Some (x) = opzione
									    allora viene chiamata la funzione foo (x)  = {}",x);
									    foo(x);
									} else {
									    bar(); //se non combacia il pattern viene chiamata elese 
									}
									}							 	
	while let											4.21.2
		Similmente, il costrutto while let può essere usato quando si vuole ciclare condizionalmente fin tanto che un valore combacia con un certo pattern. Trasforma il seguente codice:

			//esempio con il loop senza while let
			fn main() {
			    let mut v = vec![1, 3, 5, 7, 11];
			loop {
				 //devo usare match
			    match v.pop() {
			        Some(x) =>  println!("{}", x),
			        None => break,
			    }
			}
			}

			nel seguente codice:

					fn main() {
					    let mut v = vec![1, 3, 5, 7, 11];

					//qui uso while let e finche combacia il pattern Some(x) = v.pop() viene eseguito
					while let Some(x) = v.pop() {
					    println!("{}", x);
					}
					}

 OGGETTI-TRATTI											4.22.
 				@come@individuare_(la @versione del programma con il @disbrigo o @dispatch)
	 		Quando del codice coinvolge del polimorfismo, serve un meccanismo per determinare quale specifica versione viene effettivamente eseguita. Questo si chiama ‘dispatch’ (in italiano, "disbrigo"). Ci sono due forme principali di dispatch: il dispatch statico e il dispatch dinamico. Rust favorisce il dispatch statico, ma supporta anche il dispatch dinamico tramite un meccanismo chiamato ‘oggetti-tratto’ ["trait object"].
 	Premesse											4.22.1
 		Per il resto di questa sezione, ci serviranno un tratto e alcune implementazioni. Facciamone una semplice, Foo. Ha un metodo che ci si aspetta che restituisca una String.

 				trait Foo {
					    fn metodo(&self) -> String;  //metodo che restituisce una
					}


		Implementeremo anche questo tratto per u8 e String:

			impl Foo for u8 {
				    fn metodo(&self) -> String { format!("u8: {}", *self) }
				}

				impl Foo for String {
				    fn metodo(&self) -> String { format!("string: {}", *self) }
				}


 	Dispatch statico									4.22.2
 		Possiamo usare questo tratto per eseguire un dispatch static con i legami dei tratti:

 			//RICAPITOLIAMO:

 			//01 = crea il tratto
			trait Foo { fn metodo(&self) -> String; }

				//02 = CREA due metodi uno per u8 e uno per una string
				impl Foo for u8 { fn metodo(&self) -> String { format!("u8: {}", *self) } }
				impl Foo for String { fn metodo(&self) -> String { format!("string: {}", *self) } }

				//funzione per utilizzare il tratto
				fn fai_qualcosa<T: Foo>(x: T) {
				    x.metodo();
				}

				fn main() {
					//qui assegno x e y e chiama la funzione fai_qualcosa <  >... CHE UTILIZZA IL TRATTO
				    let x = 5u8;
				    let y = "Hello".to_string();

				    fai_qualcosa(x);
				    fai_qualcosa(y);
				}


		Qui Rust usa la ‘@monomorfizzazione’ per eseguire il dispatch statico. Questo significa che Rust creerà una versione speciale di fai_qualcosa() sia per u8 che per String, e poi sostituirà i punti di chiamata con chiamate a queste funzioni specializzate. In altre parole, Rust genera qualcosa così:

				//RUST CREA QUESTO
					fn fai_qualcosa_u8(x: u8) {
					    x.metodo();
					}

					fn fai_qualcosa_string(x: String) {
					    x.metodo();
					}

					fn main() {
					    let x = 5u8;
					    let y = "Hello".to_string();

					    fai_qualcosa_u8(x);
					    fai_qualcosa_string(y);
					}

		VANTAGGI DEI DISPATCH STATICO = Questo ha un aspetto molto positivo: il dispatch statico consente che le chiamate di funzione siano espanse in linea, perché il chiamato è noto in fase di compilazione, e l'espansione in linea è la chiave per una buona ottimizzazione. Il dispatch statico è veloce, ma si paga: il gonfiore del codice ["‘code bloat’"], dovuto alle numerose copie della stessa funzione esistente nel binario, uno per ogni tipo.
					@dispach@statico_(piu efficiente rispetto al dinamico)
		COMBINAZIONE DISPACH STATICO + DISPACH DINAMICO - NO IL CONTRARIO = Però, il caso più tipico è quello che è più efficiente usare il dispatch statico, e si può sempre avere una piccola funzione chiamata con dispatch statico che esegue il dispatch dinamico, mentre non è possibile il contrario, ossia trasformate il dispatch dinamico in statico, il che significa che le chiamate statiche sono più flessibili. Per questo ragione, la libreria standard cerca di usare il dispatch statico il più possibile.



		FUNZIONI ESPANSE 

 	Dispatch dinamico									4.22.3
 			
 			@trasformazione@tratto_(in oggetto tratto tramite @forzatura o @conversione)
 			
 			@oggetti@tratto_(@oggetto@tratto che immaganizzano valori di qualsiasi tipo noto solo in fase di esecuzione)

 			@ottenere@tratto_(il tratto puo essere ottenuto da un @puntatore@tipo)

 		Rust fornisce il DISPATCH DINAMICO tramite una caratteristica chiamata ‘oggetti-tratto’. Gli oggetti-tratto, come &Foo o Box<Foo>, sono valori normali che immagazzinano un valore di qualunque tipo che implementa il dato tratto, dove il preciso tipo può essere noto solo in fase di esecuzione.

		COME SI PUO OTTENERE UN OGGETTO TRATTO - FORZATURA E CONVERSIONE DEGLL'OGGETTO TRATTO= Un oggetto-tratto può essere ottenuto da un puntatore a un tipo concreto che implementa il tratto, 
			convertendolo (per es. &x as &Foo) o 
			forzandolo (per es. usando &x come argomento a una funzione che prende un &Foo).

				@tratto@forzato@tratto@convertito_(il tratto puo essere convertito o @forzato)

		FORZATURE E CONVERSIONI ANCHE PER I PUNTATORI E BOX <T>
			
			Queste forzature e conversioni di oggetti-tratto funzionano anche per i :
				
				puntatori come &mut T convertito in &mut Foo, 
					e per 
				Box<T> convertito in Box<Foo>, 

				e per il momento per nient'altro. Le forzature e le conversioni sono identiche.

		CANCELLARE IL PUNTATORE  DEL TIPO = Questa operazione può essere vista come un ‘cancellare’ la conoscenza che il compilatore ha sullo specifico tipo del puntatore, e quindi talvolta si fa riferimento agli oggetti-tratto come a ’cancellazioni di tipo’.

			Tornando all'esempio di prima, possiamo usare il medesimo tratto per eseguire un dispatch dinamico con gli oggetti-tratto, sia tramite la conversione:

							//DISPATCH DINAMICO 
							fn fai_qualcosa(x: &Foo) {  //riceve un tratto Foo
							    x.metodo();
							}

							fn main() {
							    let x = 5u8;      		//qui invio un 5 u8
							    fai_qualcosa(&x as &Foo);  //puntatore al tipo &x + &Foo
							}

			che tramite la forzatura:


				//@funzione_(che prende un @oggetto@tratto non è specializzata)
				fn fai_qualcosa(x: &Foo) {
					    x.metodo();
					}

					fn main() {
					    let x = "Hello".to_string();
					    fai_qualcosa(&x);
					}

			DISPACHT DINAMICO RENDE PIU LENTE LE FUNZIONI VIRTUALI: Una funzione che prende un oggetto-tratto non è specializzata per ognuno dei tipi che implementano Foo: ne viene generata solamente una copia, con la conseguenza che spesso (ma non sempre) si riduce il gonfiore di codice. 

			Però, ciò comporta il costo di richiedere le più lente chiamate di funzioni virtuali, e di inibire ogni possibilità di espandere in linea e di applicare le relative ottimizzazioni.





 	Perché i puntatori?									4.22.4
			
			@tratti@puntatori_(nei tratti non ci sono puntatori default ma puntatore che rappresentato tipo e dimensione) 	

 		RUST IL PUNTATORE DEVE CONOSCERE LA DIMENSIONE DEI TIPI = Rust non mette le cose dietro un puntatore di default, diversamente da molti linguaggi gestiti ["managed"], e quindi i tipi possono avere dimensioni diverse. 

 		IMPORTANTE LA DIMENSIONE DEL VALORE: Conoscere la dimensione del valore in fase di compilazione è importante per cose  come 
 				passarlo come argomento a una funzione, 
 				spostarlo in giro per lo stack, 
 					e 
				allocare (e deallocare) spazio sullo heap per immagazzinarlo.
		
		NECESSARIO IL PUNTATORE: Per Foo, avrebbo bisogno di avere un valore che potesse rappresentare almeno una String (24 byte) o un u8 (1 byte), e così pure qualunque altro tipo per cui i crate dipendenti possono implementare Foo (assolutamente qualunque numero di bytes). 

		Non c'è modo di garantire che quest'ultimo punto possa funzionare se i valori vengono immagazzinati senza un puntatore, perché quegli altri tipi possono essere arbitrariamente grandi.

		IMPORTANTE LA DIMENSIONE DEL PUNTATORE = Mettere il valore dietro un puntatore significa che la dimensione di tale valore non è rilevante quando si maneggia un oggetto-tratto, e importa solo la dimensione del puntatore stesso.




 	Rappresentazione									4.22.5
 		
 				@oggetti@tratto_(@non@chiaro @non@chiara la esposizione o il @significato)

 		VTABLE NECESSARIA PER CHIAMARE I METODI DEL PUNTATORE  A FUNZIONE : I metodi del tratto possono essere chiamati su un oggetto-tratto tramite uno speciale record di puntatori di funzione tradizionalmente chiamato ‘vtable’ (creato e gestito dal compilatore).

 		Gli oggetti-tratto sono sia semplici che complicati: la loro rappresentazione e disposizione interna è davvero immediata, ma ci sono da scoprire alcuni messaggi d'errore contorti e alcuni comportamenti sorprendenti.

 		Iniziamo dal facile, con la rappresentazione in fase di esecuzione di un oggetto-tratto. Il modulo std::raw contiene le struct con le disposizioni che sono le medesime dei complicati tipi incorporati, compresi gli oggetti-tratto:



		fn main() {
				    
				    mod foo {
				pub struct TraitObject {
				    //due puntatori uno a data ed uno alla vtable
				    pub data: *mut (),
				    pub vtable: *mut (),
				}
				}
				}

				Cioè, un oggetto-tratto come &Foo consiste in un puntatore ‘data’ e un puntatore ‘vtable’.

				Il puntatore 
					data indirizza i dati (di qualche tipo sconosciuto T) che l'oggetto-tratto sta immagazzinando, 
						e il 
					puntatore vtable punta alla vtable (‘tabella dei metodi virtuali’) corrispondente all'implementazione di Foo per T.



				VTABLE - STRUCT PUNTATORI A FUNZIONE: 
				Una vtable è essenzialmente una struct di puntatori a funzione, che puntano al pezzo concreto di codice macchina per ogni metodo nell'implementazione. 

				Una chiamata di metodo come oggetto_tratto.metodo() recupererà il puntatore corretto dalla vtable e poi farà una chiamata dinamica ad esso. Per esempio:

						fn main() {
							    struct FooVtable {
						    	//camp destructor punta ad una funzione che rilascera memoria
							    destructor: fn(*mut ()),
							    //campo size + align immagazzinano il tipo e la dimensione
							    size: usize,
							    align: usize,
							    method: fn(*const ()) -> String,
							}

							// u8:

							fn call_method_on_u8(x: *const ()) -> String {
							    // il compilatore garantisce che questa funzione è chiamata solamente
							    // con `x` che punta a un u8
							    let byte: &u8 = unsafe { &*(x as *const u8) };

							    byte.method()
							}

							static Foo_for_u8_vtable: FooVtable = FooVtable {
							    destructor: /* magia del compilatore */,
							    size: 1,
							    align: 1,

							    // cast a un puntatore a funzione
							    method: call_method_on_u8 as fn(*const ()) -> String,
							};


							// String:

							fn call_method_on_String(x: *const ()) -> String {
							    // il compilatore garantisce che questa funzione è chiamata solamente
							    // con `x` che punta a una String
							    let string: &String = unsafe { &*(x as *const String) };

							    string.method()
							}

							static Foo_for_String_vtable: FooVtable = FooVtable {
							    destructor: /* magia del compilatore */,
							    // questi sono i valori per un target a 64 bit;
							    // bisogna dimezzarli per un target a 32 bit
							    size: 24,
							    align: 8,

							    method: call_method_on_String as fn(*const ()) -> String,
							};
							}
					Il campo destructor in ogni vtable punta a una funzione che rilascerà qualunque risorsa del tipo vtable: per u8 è banale, ma per String libererà della memoria. Questo è necessario per gli oggetti-tratto che possiedono, come Box<Foo>, che ha bisogno di rilasciare sia l'allocazione di Box che il tipo interno, quando escono dall'ambito. I campi size e align immagazzinano la dimensione del tipo cancellato, nonché i suoi requisiti di allineamento; questi campi sono essenzialmente inutilizzati al momento, dato che questa informazione è incorporata nel distruttore, ma verrà usata in futuro, dato che gli oggetti-tratto sono resi progressivamente più flessibili.

					Supponiamo di avere alcuni valori che implementano Foo. La forma esplicita di costruzione e utilizzo degli oggetti-tratto di Foo potrebbe sembrare un po' così (ignorando gli errori di tipo: comunque sono tutti puntatori):


						fn main() {

						    let a: String = "foo".to_string();
								let x: u8 = 1;

								// let b: &Foo = &a;
								let b = TraitObject {

								    // immagazzina i dati
								    data: &a,
								    // immagazzina i metodi
								    vtable: &Foo_for_String_vtable
								};

								// let y: &Foo = x;
								let y = TraitObject {

								    // immagazzina i dati
								    data: &x,
								    // immagazzina i metodi
								    vtable: &Foo_for_u8_vtable
								};

								// b.method();
								(b.vtable.method)(b.data);

								// y.method();
								(y.vtable.method)(y.data);
								}


 	Sicurezza			 come oggetto								4.22.6

 		OGGETTI-TRATTO NO VETTORI : Non tutti i tratti possono essere usati per costruire un oggetto-tratto. Per esempio, i vettori implementano Clone, ma se proviamo a costruirne un oggetto-tratto:

 			fn main() {
				    let v = vec![1, 2, 3];
				let o = &v as &Clone;
				}

				@TRATTI@SICURI_(sono tratti sicuri se non ha Self:Size e i suoi metodi sono sicuri)
				@tratti@trasformati_(in @oggetti-tratto sono sicuri)

				@tratti@non@trasformabili_(non sono trasformabili i oggetto tratti i vettori perchè creano @oggetti@non@sicuri, @no@object@safe)

		L'errore dice che Clone non è ‘object-safe’, cioè ’sicuro come oggetto’. Solamente i tratti che sono sicuri come oggetti possono essere trasformati in oggetti-tratto. Un tratto è sicuro come oggetto se sono vere entrambe le seguenti proprietà:

			il tratto non richiede che valga Self: Sized
			tutti i suoi metodi sono sicuri come oggetti

					@metodi@sicuri_(metodi sicuri come oggetto xhe ha Self:Size oppure NON HA parametri tipo o non usi Self)

			METODI SICURI COME OGGETTO : Ma che cosa rende un metodo sicuro come oggetto? Ogni metodo deve richiedere che 
				valga Self: Sized 

					oppure che valga tutto il seguente:

				deve non avere parametri di tipo
				deve non usare Self

			Urca! Come si può vedere, quasi tutte queste regole parlano di Self. Una buona intuizione è “eccetto in circostanze speciali, se il metodo del tratto usa Self, non è object-safe.”



 CHIUSURE												4.23.
 	Sintassi 											4.23.1
 	Le chiusure e il loro ambiente						4.23.2
 	Le chiusure move 									4.23.3
 	Implementazione delle chiusure						4.23.4
 	Prendere chiusure come argomenti					4.23.5
 	Puntatori a funzione e chiusure						4.23.6
 	Restituire chiusure									4.23.7

 		@da@finire@carlomilanesi
 			https://carlomilanesi.github.io/linguaggio-rust/traits.html

 SINTASSI UNIVERSALE DI CHIAMATA DI FUNZIONE			4.24.
 	["Universal Function Call Syntax" o "UFCS"]			4.24.1
 	Forma a parentesi angolari							4.24.2


 CRATE E MODULI											4.25.	
 CONST E STATIC											4.26.	
 ATTRIBUTI												4.27.	
 ALIAS TRAMITE `TYPE`									4.28.	
 FORZATURA DI TIPO										4.29.	
 TIPI ASSOCIATI											4.30.	
 TIPI NON DIMENSIONATI									4.31.	
 OPERATORI E SOVRACCARICAMENTO							4.32.	
 COERCIZIONE DEREF										4.33.	
 LE MACRO												4.34.	
 PUNTATORI GREZZI										4.35.	
 `UNSAFE`												4.36.	
 RU

 ST EFFICACE											5.0  	
 LO STACK E LO HEAP										5.1.	
 COLLAUDO												5.2.	
 COMPILAZIONE CONDIZIONALE								5.3.	
 DOCUMENTAZIONE											5.4.	
 ITERATORI												5.5.	
 CONCORRENZA											5.6.	
 GESTIONE DEGLI ERRORI									5.7.	
 SCEGLIERE LE GARANZIE									5.8.	
 FFI													5.9.	
 PRESTITO E ASREF										5.10.	
 CANALI DI RILASCIO										5.11.	
 USARE RUST SENZA LA LIBRERIA STANDARD					5.12.	
 RUST NOTTURNO											6.0.0	
 PLUGIN DEL COMPILATORE									6.1.	
 ASSEMBLY IN-LINE										6.2.	
 OMETTERE LA LIBRERIA STDANDARD							6.3.	
 INTRINSECI												6.4.	
 ELEMENTI "LANG"										6.5.	
 LINK AVANZATO											6.6.	
 COLLAUDI PRESTAZIONALI									6.7.	
 SINTASSI DI BOX E RELATIVI PATTERN						6.8.	
 PATTERN DI SLICE										6.9.	
 COSTANTI ASSOCIATE										6.10. 	
 ALLOCATORI PERSONALIZZATI								6.11. 	
 GLOSSARIO												7.00	
 INDICE ANALITICO DELLA SINTASSI						8.00   	


INDICE_COMANDI_CARGO+RUST

	C
		CONVENZIONE IN RUST
					Vedi: https://www.freecodecamp.org/italian/news/impara-a-programmare-con-rust-corso-interattivo-in-linguaggio-rust-su-replit/#le-basi-di-rust
					
				NOMENCLATURA
					In Rust, le convenzioni di nomenclatura sono le seguenti:

					OGGETTO		STILE
					Variabili	snake_case
					Funzioni	snake_case
					File		snake_case
					Costanti	SCREAMING_SNAKE_CASE
					Static	    SCREAMING_SNAKE_CASE
					Tipi	    PascalCase
					Tratti	    PascalCase
					Enum	    PascalCase

				CONVENZIONE PER LE VARIABILI
					il trattino basso _ indica che la variabile non è utilizzata:
						fn main() {
								  let _unused_variable = my_func(10);
								}
				PARAMETRI CON SINTASSI :
					I parametri di funzione vengono scritti usando la sintassi :


						fn my_func(x: u8) -> i32 {
						  x as i32
						}

				STRINGHE E SLICE IN RUST
					Un motivo di confusione comune per i principianti di Rust è la differenza tra la struct String e il tipo str.

						//&str = memorizzata nello stack quindi fissa - riferimento c++ 
						let my_str: &str = "Hello, world!";
						//my_string = memorizzata nell'heap con grandezza sconosciuta al mento della compilazione

						let my_string: String = String::from("Hello, world!");


					Nell'esempio qui sopra, my_str è un riferimento a una stringa letterale, e my_string è un'istanza della struct String.

					Un'importante distinzione tra le due è che my_str è memorizzata nello stack, mentre my_string è allocata nella memoria heap. Ciò significa che il valore di my_str non può cambiare e la sua dimensione è fissa, mentre my_string può avere una dimensione sconosciuta al tempo di compilazione.

					Una stringa letterale è anche conosciuta come string slice (letteralmente "fetta"), poiché &str fa riferimento a parte di una stringa. In linea generale, ecco la somiglianza tra stringhe e array:


	I
		extern
			serve a comunicare a rust di utilizzare una dipendenza indicata nel .toml. CHE
			PROVVEDERA a scaricarla es. 0.3.0 SE NON esiste, e alla prima creazione del progetto
			imposta anche il Cargo.lock a questa dipendenza in modo che se usciranno ulteriori versioni
			es. 0.3.1 instabili non saranno scaricate a meno che non si imposti manualmente nel toml.
			Attenzione se esce altre due versione es. 0.3.9 e la 0.4.0 POSSIAMO USARE IL COMANDO
				cargo update
			che permette di aggiornare la versione solo tra > 0.3.0 and <= 0.3.9 quindi tutte le successive a quella imposta ma non la quattro che dovrà essere specificata nelle dipendenze
			del Cargo.toml.
			Il Cargo.lock fa una difesa del progetto in quanto utilizza sempre quella valida e senza errori.

				esempio
					.toml
						# Iimposto nel cargo.toml  la dipendenza della libreria rand
						[dependencies]
						rand="0.3.0"

					.rs
						//nel file main.rs COMUNICO a rust l'utilizzo della libreria rand con:

						extern crate rand;
