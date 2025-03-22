
fn main() {
    /* let mut v = vec![1, 3, 5, 7, 9];
    println!("size: {}, capacity: {}", v.len(), v.capacity());
    /*
    * Output:
    * size: 5, capacity: 5
    */
    v.push(11);
    println!("size: {}, capacity: {}", v.len(), v.capacity());
    /*
    * Output:
    * size: 6, capacity: 10 (ha raddoppiato la capacità, se lo esaurissi la nuova capacità sarebbe 20)
    */

    while v.len() > 0 {
        v.remove(0); // toglie dalla posizione 0 e sposta gli altri (operazione costosa)
        println!("{:?}", v);
        // v.pop(); // tira via dal fondo, accorcia
        print!("len: {}, capacity: {}\n", v.len(), v.capacity());
    }
    println!("{:?}", v);*/

    let a = if 10 > 5 { // valutazione della condizione per decidere cosa assegnare ad a, con il relativo tipo
        10
    } else {
        5 // "ciao" // se io scrivo una stringa, non va bene, in quanto sono due tipi diversi per due casistiche diverse
    };
} // quando arrivo qui, il blocco di memoria preso da v viene liberato (evapora)
