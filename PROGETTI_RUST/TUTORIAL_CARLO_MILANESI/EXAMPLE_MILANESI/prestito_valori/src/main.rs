fn main() {
        //@esempio_(@prestito valori)
            // Non importa se non si capisce cosa fa `fold`, quello che importa
            // qui è che un riferimento immutabile viene preso in prestito.
            fn somma_vec(v: &Vec<i32>) -> i32 {
                return v.iter().fold(0, |a, &b| a + b);
            }
            // Prendi in prestito due vettori e sommane gli elementi.
            // Questo tipo di prestito non permette che gli oggetti siano mutati.
            fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
                // fa' qualcosa con v1 e con v2
                let s1 = somma_vec(v1);
                let s2 = somma_vec(v2);
                // restituisci la risposta
                println!("\n\n\n 
                          i due vettori sono presi in prestito
                          che non possono essere mutati ma vengono
                          restituiti come somma s1+s2, s1= {} s2= {} \n", s1,s2);
                s1 + s2
            }

            let v1 = vec![1, 2, 3];
            let v2 = vec![4, 5, 6];

            let risposta = foo(&v1, &v2);
            println!("risultato della presa in prestito di due vettori: {}", risposta);
        }