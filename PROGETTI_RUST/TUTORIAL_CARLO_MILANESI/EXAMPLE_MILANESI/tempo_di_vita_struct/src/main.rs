    //@esempio@tempo_(main @progetto tempo di vita foo della @struttura o @struct)
    //TEMPI DI VITA ESPLICITI = riferimento a Foo con tempo di vita = a i32 che lo contiene e non superiore
    struct Foo<'a> {  //Foo<'a> = strutture con tempi di vita espliciti
            x: &'a i32,
        }

        fn main() {
            let y = &5; // questo è lo stesso che `let _y = 5; let y = &_y;`
            println!("/n imposto la variabile y =&5 :  {}", y);
            let f = Foo { x: y };   //chiami foo e passi x:Y ossia il valore di Y a x
            println!("/n chiamo foo e passo y = :  {}", y);
            //stampo f.x con il tempo di vita di foo e &y
            println!("/n tempo di vita della variabile &y f.x = {}", f.x);
        }


