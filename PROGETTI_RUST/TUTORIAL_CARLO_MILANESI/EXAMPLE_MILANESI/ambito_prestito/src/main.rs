            //@esempio@di_(area scope di y) progetto di Rust per la verifica.

            //il prestito mutabile valido e parte da x e finisce alla graffa finale,
            // l'ambita parte da --> { .... e finisce con ---> }
            fn main() {
                    //prestito mutabile parte qui'
                    let mut x = 5;      //&mut T = riferimento MUTABILE di x
                    
                   { let y = &mut x;     //&mut T = faccio il prestito mutabile di x
                                        // a ---> y

                    *y += 1;            //prendo il riferimento con *Y = puntatore e lo
                                        // incremento
                                        //fino qui il prestito è mutabile e x non è piu
                                        // possessore
                    println!("PRESTITO MUTABILE a y= {}\n
                         ------------------------------------- 
                         ho messo le graffe per lo scope di y e questo è
                         l'area di Y", y);
                    }                    

                    println!("finisce lo scope di y e il posesso torna a X=  
                              e il valore di x passa da 5 a ---> {}", x);  //da errore il prestito è sempre nell'ambito di y
                                        //fino nell'ambito delle graffe e quindi mutabile 
                                        //e non è tornato a x come immutabile 
                                        //quindi errore perche PRESTITO IMMUTABILE
                    
                } //prestito mutabile valido fino a fine programma 