use libloyalfier::{
    arranger::{make_table, Sample},
    img::{Paper, PaperObfuscation, PaperSize},
};
use photon_rs::{native::save_image, PhotonImage};
use rand::Rng;
use std::{collections::HashMap, rc::Rc};
fn main() {
    let path = String::from(".\\images\\");
    let mut images: Vec<Rc<PhotonImage>> = vec![];
    for i in 1..=12 {
        let img = photon_rs::native::open_image(&(path.clone() + &i.to_string() + ".png"));
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
    for i in 0..12 {
        let new_smp = Sample {
            id: i,
            dummy: false,
        };
        samples.push(new_smp);
        map.insert(new_smp, images[i].clone());
    }
    let table = make_table(samples, 1, 10, 8).unwrap();
    let mut papers: Vec<Paper> = vec![];
    for i in 0..1 {
        papers.push(
            Paper::make(
                map.clone(),
                table.clone(),
                i,
                PaperSize::pixels(&PaperSize::A4),
            )
            .unwrap(),
        );
    }
    let mut rng = rand::thread_rng();
    // for index in 0..3 {
    let index = 0;
    let i = &papers[index];
    let obf = match rng.gen_range(0..=1) {
        0 => PaperObfuscation::Upward,
        1 => PaperObfuscation::Downward,
        _ => panic!("Should never happen"),
    };
    let row = rng.gen_range(1..=3);
    i.alter();
    i.obfuscate(obf, row);
    let ultimate = i.make_image();
    let x = save_image(
        ultimate,
        &(String::from("./") + &index.to_string() + ".png"),
    );
    match x {
        Ok(_) => {}
        Err(x) => {
            println!("{:?}", x);
        }
    }
    //}
}
