use std::env;
/* 
use qrcode::QrCode;
use qrcode::render::unicode;
use image::Luma;
use std::fs::File;
*/

fn show_help()
{
    println!(
    "\
qrcli 0.1.0
Générateur de QR Code en ligne de commande — en Rust 🦀

USAGE:
    qr_code_generator --text <TEXTE> [OPTIONS]

FLAGS:
    -h, --help          Affiche ce message d’aide

OPTIONS:
    -t, --text <TEXTE>          Texte ou URL à encoder dans le QR code (obligatoire)
    -o, --output <FICHIER>      Sauvegarde le QR code au format PNG dans le fichier donné

EXEMPLES:
    qr_code_generator --text \"https://example.com\"
    qr_code_generator -t \"Hello World\" --output hello.png
    qr_code_generator -t \"Caché\" --quiet --output hidden.png

AUTEUR:
    Thomas.S

DÉPENDANCES:
    - qrcode : Génération de QR code
    - image  : Export PNG
"   );

}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.as_slice()
    {
        [_program] => {
            eprintln!("Erreur : aucun argument fourni.");
            eprintln!("Usage : qrcli -t <texte> [-o <fichier.png>]");
        }

        [_program, flag] if flag == "--help" || flag == "-h" => show_help(),

        [_program, flag, value] if flag == "-t" || flag == "--text" => {
            println!("text a transformer en Qr code : {}", value);
        }

        [_program, flag1, value1, flag2, value2] if (flag1 == "-t" || flag1 == "--text") && (flag2 == "-o" || flag2 == "--output") => {
            println!("text a transformer en Qr code : {}\nsauvegarde {}", value1, value2);
        }

        _ => {
            eprintln!("Arguments invalides : {:?}", &args[1..]);
            eprintln!("Usage : qrcli -t <texte> [--output <fichier.png>]");
        }
    }

}
