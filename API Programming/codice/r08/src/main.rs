struct Point { // definizione struttura point
    x: i32,
    y: i32,
}

impl Point { // definizione implementazione point
    fn new(x: i32, y: i32) -> Point {
        println!("Creating a new Point ({}, {})", x, y); // creazione di un nuovo punto
        Point { x, y }
    }
}

impl Drop for Point { // impegno a dare un corpo alla funzione
    fn drop(&mut self) {
        println!("Dropping Point ({}, {})", self.x, self.y); // butto via un punto
    }
}

fn test(p: Point){
    println!("Inside test function: ({}, {})", p.x, p.y); // vedo che effettivamente ha ricevuto un dato
}

fn main() {
    let mut p1 = Point::new(1, 2); // stampo "creo il punto (1, 2)" e "distruggo il punto (1, 2)"
    { // Attivazione nuovo scope
        let p2 = p1; // lo stack cresce, ho bisogno di una nuova variabile locale dove metto il contenuto di p1
        p1 = Point::new(3, 4);
        // MOVIMENTO:
        test(p2); // p2 ha temporanamente preso il possesso di (1, 2) e ne invoco sopra la funzione test => cedo il contenuto a test
        // quando finisce test, ciò che ha ricevuto lo distrugge (lo droppa PRIMA della fine dello scope interno, a differenza di prima dove non avevo la funzione test)
        // la funzione test si occupa dello smaltimento
        println!("Fine dello scope interno"); // stampa
    } // p2 esce di scena, perché esiste solo nello scope interno, devo rilasciare p2 (stampa "Dropping ...")
    println!("Fine dello scope esterno")
} // fa uscire di scena p1, che devo distruggere

// Warning: variabile p non usata (vabbè)

/*
 * Output:
 * Creating a new Point (1, 2)
 * Dropping Point (1, 2)
 */

/*
 * Output:
 * Creating a new Point (1, 2)
 * Fine dello scope interno
 * Dropping Point (1, 2)
 * Fine dello scope esterno
 */

/*
 * Creating a new Point (1, 2)
 * Creating a new Point (3, 4)
 * Fine dello scope interno
 * Dropping Point (1, 2)
 * Fine dello scope esterno
 * Dropping Point (3, 4)
*/

/*
 * Creating a new Point (1, 2)
 * Creating a new Point (3, 4)
 * Inside test function: (1, 2)
 * Dropping Point (1, 2)
 * Fine dello scope interno
 * Fine dello scope esterno
 * Dropping Point (3, 4)
*/
