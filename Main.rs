use std::io;

fn main()
{
    println!("Bienvenue dans mon programme de calcul de nombre premiers");

    let mut input_nombre_string : String =  String::new();

    println!("Veuillez entrer un nombre");

    io::stdin().read_line(&mut input_nombre_string).expect("Erreur lors de la lecture du fichier");

    let mut input_nombre_int : u32 = input_nombre_string.trim().parse().expect("Erreur");

    println!("{}", inputNombreInt);

    let mut tableau_nombre_premiers : Vec<u64> = Vec::new();
}