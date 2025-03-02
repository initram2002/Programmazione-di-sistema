use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // la parte in grigio chiaro è un'annotazione che fa emergere RustRover, che indica il tipo 
    // della variabile (ecco perché è bello usare gli IDE)
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    let f1 = BufReader::new(f); // per guardarci solo dentro, lo devo passare come puntatore in sola lettura (&), se lo deve modificare te lo deve restituire
    // f.set_len(0); // (value used after being moved) provo a fare qualcosa di sbagliato, uso f quando l'ho già passato a BufReader, non posso più toccarlo (principio di "single ownership" => non esiste valore se non c'è variabile che lo possiede)
    for line in f1.lines() { // line è di tipo "result<String>" capisce se è una stringa o un errore
        if line.is_err() {
            // posso dirgli di non prendere possesso ma solo di guardarci dentro con & (puntatore in sola lettura), poi lo restituisce

            // Concetto molto lontano dai soliti, ecco perché Rust è un outlier in termini di efficienza e sicurezza

            match &line { // esamina la struttura di un dato (binario, buono o cattivo), ci fa guardare dentro e vedere cosa c'è
                Ok(l) => println!("{}", l), // è andato tutto bene, ecco il contenuto, racchiude la linea
                Err(e) => println!("Error: {}", e), // è successo qualcosa, errore, racchiude l'errore
            }
            println!("Failed to read line");
            // continue;
        }
        // (senza &) value used after being moved: quando line è stata presa in match, match l'ha spacchettata per vederci dentro, ora line non esiste più, ci sono le parti di cui è composta: ok (che contiene la linea se è andato tutto bene) ed e (che contiene l'errore)
        // line originale è sparita, non posso più usarla
        println!("{}", line.expect("Failed to read line"));
    }

    sleep(Duration::from_secs(60));
}
