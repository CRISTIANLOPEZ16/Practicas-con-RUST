fn main() {
    println!("Hola amigo");
        // a ver cuantos nombres conoces 
    let mut nombres:Vec<String>= Vec::new();
    for i in 0..3{
        println!("Digita el nombre numero {}  que conoces ", i);
        let mut nombre = String::new();
        std::io::stdin().read_line(&mut nombre).unwrap();
        nombres.push(nombre);
    }   
    // rust esta muy integrado los vectores con el bucle for, ejemplo de foreach
    for nombre in nombres {
        println!("El nombre es :{}", nombre);
    }
    
}
