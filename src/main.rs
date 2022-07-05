use std::io;
fn main() {
    println!("Digita la temperatura maxima");
    // En esta parte defino con claridad el tipo de dato
    let mut tmax:String= String::new();
    io::stdin().read_line(&mut tmax).unwrap();
    println!("Digita la temperatura minima");
    // En esta parte no defino con claridad el tipo de dato
    let  mut tmin=String::new();
    io::stdin().read_line(&mut tmin).unwrap();
    let tmaxi:u8=tmax.trim().parse().unwrap();
    let tmini:i8=tmin.trim().parse().unwrap();
    println!("La temperatura maxima de mi ciudad fue {} y la temperatura minima fue {}",tmaxi,tmini);
}
