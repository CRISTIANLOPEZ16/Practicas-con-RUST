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

    if tmini < -5 {
        println!("Que pinche frio amigo mio, temperatura minima {} ",tmini);
    }else{
        let promedio:i8=(tmaxi as i8+tmini)/2;
        println!("ta bien el promedio del clima {}",promedio);
    }
    
}
