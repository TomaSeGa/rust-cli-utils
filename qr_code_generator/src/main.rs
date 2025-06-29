use std::env;
use qrcode::QrCode;
use image::Luma;
use std::fs;
use std::path::Path;


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

fn main()
{
    let args: Vec<String> = env::args().collect();

    let path_to_save_qrcode = "qrcode_output";
    if !Path::new(path_to_save_qrcode).exists() 
    {
        let _ = fs::create_dir(path_to_save_qrcode);
    }

    match args.as_slice()
    {
        [_program] => {
            eprintln!("Erreur : aucun argument fourni.");
            eprintln!("Usage : qrcli -t <texte> [-o <fichier.png>]");
        }

        [_program, flag] if flag == "--help" || flag == "-h" => show_help(),

        [_program, flag, value] if flag == "-t" || flag == "--text" => {
            println!("text a transformer en Qr code : {}", value);
            // encode Qrcode
            let code_bits = QrCode::new(value.as_bytes()).unwrap();
            // print Qrcode
            let qrcode_string = code_bits.render()
                .light_color(' ')
                .dark_color('#')
                .build();
            println!("{}", qrcode_string);

        }

        [_program, flag1, value1, flag2, value2] if (flag1 == "-t" || flag1 == "--text") && (flag2 == "-o" || flag2 == "--output") => {
            // check if save name is a png
            let save_name;
            if !flag2.ends_with(".png")
            {
                save_name = format!("{}.png", value2);
            }
            else 
            {
                save_name = value2.clone();
            }
            println!("text a transformer en Qr code : {}\nsauvegarde {}", value1, value2);

            let code_bits = QrCode::new(value1.as_bytes()).unwrap();
            let image = code_bits.render::<Luma<u8>>().build();
            // Save the image.
            let path = format!("./qrcode_output/{}", save_name);
            image.save(path).unwrap();
            // print Qrcode
            let qrcode_string = code_bits.render()
                .light_color(' ')
                .dark_color('#')
                .build();
            println!("{}", qrcode_string);
        }

        _ => {
            eprintln!("Arguments invalides : {:?}", &args[1..]);
            eprintln!("Usage : qrcli -t <texte> [--output <fichier.png>]");
        }
    }
}
