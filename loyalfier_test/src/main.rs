use libloyalfier::{
    arranger::{make_table, Sample},
    img::{Paper, PaperObfuscation, PaperSize},
};
use photon_rs::{native::save_image, PhotonImage};
use rand::Rng;
use std::{collections::HashMap, rc::Rc};
fn main() {
    // Configuration field
    const SAMPLES: usize = 15;
    const COLUMNS: usize = 5;
    const ROWS: usize = 20;
    const PAGES: usize = 6;
    let paper_size = PaperSize::B5.pixels();
    let in_path = String::from(".\\images\\");
    let out_path = String::from(".\\output\\");

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
    let mut rng = rand::thread_rng();
    for index in 0..PAGES {
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
        i.alter();
        let ultimate = i.make_image();
        let x = save_image(ultimate, &(out_path.clone() + &index.to_string() + ".png"));
        match x {
            Ok(_) => {}
            Err(x) => {
                println!("{:?}", x);
            }
        }
    }
}
