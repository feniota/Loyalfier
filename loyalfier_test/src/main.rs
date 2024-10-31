use libloyalfier::{
    arranger::{make_table, Sample},
    img::{Paper, PaperObfuscation, PaperSize},
};
use photon_rs::{native::save_image, PhotonImage};
use rand::Rng;
use std::{collections::HashMap, rc::Rc};
use std::{io, io::Write, time::Instant};

fn main() {
    let now = Instant::now();

    // Configuration field
    const SAMPLES: usize = 20;
    const COLUMNS: usize = 5;
    const ROWS: usize = 20;
    const PAGES: usize = 5;
    let paper_size = PaperSize::B5.pixels();
    let in_path = String::from(".\\images\\");
    let out_path = String::from(".\\output\\");

    let now_read_images = Instant::now();
    let mut images: Vec<Rc<PhotonImage>> = vec![];
    for i in 1..=SAMPLES {
        let img = photon_rs::native::open_image(&(in_path.clone() + &i.to_string() + ".png"));
        match img {
            Ok(x) => {
                images.push(Rc::new(x));
            }
            Err(x) => {
                println!("{:?}", x);
            }
        }
    }
    println!(
        "Reading images took {:?} ({:?} from the program started)",
        now_read_images.elapsed(),
        now.elapsed()
    );

    let now_init = Instant::now();
    let mut samples: Vec<Sample> = vec![];
    let mut map: HashMap<Sample, Rc<PhotonImage>> = HashMap::new();
    for i in 0..SAMPLES {
        let new_smp = Sample {
            id: i,
            dummy: false,
        };
        samples.push(new_smp);
        map.insert(new_smp, images[i].clone());
    }
    let table = make_table(samples, PAGES, ROWS, COLUMNS).unwrap();
    let mut papers: Vec<Paper> = vec![];
    for i in 0..PAGES {
        papers.push(Paper::make(map.clone(), table.clone(), i, paper_size).unwrap());
    }
    println!(
        "Initialization took {:?} ({:?} from the program started)",
        now_init.elapsed(),
        now.elapsed()
    );

    let now_make_images = Instant::now();
    let mut rng = rand::thread_rng();
    for index in 0..PAGES {
        let now_page = Instant::now();

        println!("Making image {}/{}...", index + 1, PAGES);

        let i = &papers[index];
        let obf = match rng.gen_range(0..=1) {
            0 => PaperObfuscation::Upward,
            1 => PaperObfuscation::Downward,
            _ => panic!("Should never happen"),
        };
        //let obf = PaperObfuscation::Upward;
        let row = rng.gen_range(1..=ROWS);
        //let row = 3;
        i.obfuscate(obf, row, 30.0);
        println!("Obfuscation done ({:?} elapsed)", now_page.elapsed());
        i.alter();
        println!("Altering done ({:?} elapsed)", now_page.elapsed());
        let ultimate = i.make_image();

        println!(
            "Calculation done ({:?} elapsed), saving to file...",
            now_page.elapsed()
        );
        let _ = io::stdout().flush();

        let x = save_image(ultimate, &(out_path.clone() + &index.to_string() + ".png"));

        println!("Done in {:?}.", now_page.elapsed());

        match x {
            Ok(_) => {}
            Err(x) => {
                println!("{:?}", x);
            }
        }
    }
    println!(
        "Generating images took {:?} ({:?} from the program started)",
        now_make_images.elapsed(),
        now.elapsed()
    );

    let end = now.elapsed();
    println!("Generation stopped, totally {:?} elapsed.", end);
}
