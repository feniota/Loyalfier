use libloyalfier::{
    arranger::{make_table, Sample},
    img::{Paper, PaperObfuscation, PaperSize},
};
use rand::Rng;
use ril::prelude::*;
use std::{collections::HashMap, rc::Rc};
fn main() {
    let path = String::from(
        "C:\\Users\\Tienyu Yang\\Documents\\GitHub\\Loyalfier\\loyalfier_test\\images\\",
    );
    let mut images: Vec<Rc<Image<Rgba>>> = vec![];
    for i in 1..=12 {
        let img = Image::<Rgba>::open(path.clone() + &i.to_string() + ".png");
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
    let mut map: HashMap<Sample, Rc<Image<Rgba>>> = HashMap::new();
    for i in 0..12 {
        let new_smp = Sample {
            id: i,
            dummy: false,
        };
        samples.push(new_smp);
        map.insert(new_smp, images[i].clone());
    }
    let table = make_table(samples, 3, 10, 8).unwrap();
    let mut papers: Vec<Paper> = vec![];
    for i in 0..3 {
        papers.push(Paper::make(
            map.clone(),
            table.clone(),
            i,
            PaperSize::pixels(&PaperSize::A4),
        ));
    }
    let mut rng = rand::thread_rng();
    for index in 0..3 {
        let i = &papers[index];
        let obf = match rng.gen_range(0..=1) {
            0 => PaperObfuscation::Upward,
            1 => PaperObfuscation::Downward,
            _ => panic!("Should never happen"),
        };
        let row = rng.gen_range(1..=3);
        i.alter();
        i.obfuscate(obf, row);
        let _ = i.make_image().save(
            ImageFormat::Png,
            String::from("./") + &index.to_string() + ".png",
        );
    }
    println!("Done");
}
