#![allow(dead_code)]
use std::io::Read;
use std::time::Instant;

fn unpack_map(map: Vec<&str>) -> Vec<(usize, usize, usize)> {
    map.iter()
        .map(|x| {
            let mut s = x.split(' ').map(|x| x.parse().unwrap());
            (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
        })
        .collect()
}

fn convert_map(val: usize, maps: Vec<(usize, usize, usize)>) -> usize {
    let map_maybe = maps.iter().find(|x| val >= x.1 && val <= x.1 + x.2);
    if let Some(map) = map_maybe {
        map.0 + (val - map.1)
    } else {
        val
    }
}

fn convert_maps(val: usize, maps: Vec<Vec<(usize, usize, usize)>>) -> usize {
    let mut val_conv = val;
    for map in maps {
        val_conv = convert_map(val_conv, map);
    }
    val_conv
}

pub fn run() {
    let mut now = Instant::now();
    let mut file = std::fs::File::open("day5/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let info: Vec<_> = contents.split('\n').collect();

    let seeds_unparsed: Vec<usize> = info[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut seeds_pairs: Vec<(usize, usize)> = Vec::new();

    for idx in (0..seeds_unparsed.len()).step_by(2) {
        seeds_pairs.push((seeds_unparsed[idx], seeds_unparsed[idx+1]));
    }

    let mut seeds = Vec::new();

    for pair in seeds_pairs {
        seeds.extend(pair.0..pair.0+pair.1-1);
    }

    println!("allocated: {}", now.elapsed().as_secs());
    now = Instant::now();
    
    let seed_to_soil = unpack_map(info[3..21].to_vec());
    let soil_to_fertilizer = unpack_map(info[23..31].to_vec());
    let fertilizer_to_water = unpack_map(info[33..68].to_vec());
    let water_to_light = unpack_map(info[70..115].to_vec());
    let light_to_temperature = unpack_map(info[117..131].to_vec());
    let temperature_to_humidity = unpack_map(info[133..161].to_vec());
    let humidity_to_location = unpack_map(info[163..174].to_vec());

    /*
    let seed_to_soil = unpack_map(info[3..5].to_vec());
    let soil_to_fertilizer = unpack_map(info[7..10].to_vec());
    let fertilizer_to_water = unpack_map(info[12..16].to_vec());
    let water_to_light = unpack_map(info[18..20].to_vec());
    let light_to_temperature = unpack_map(info[22..25].to_vec());
    let temperature_to_humidity = unpack_map(info[27..29].to_vec());
    let humidity_to_location = unpack_map(info[31..33].to_vec());
    */

    let maps = vec![seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location];

    let seeds_mapped: Vec<_> = seeds
        .iter()
        .map(|x| convert_maps(*x, maps.clone()))
        .collect();
    println!("done: {}", now.elapsed().as_secs());
    now = Instant::now();

    println!("{:?}", seeds_mapped.iter().min().unwrap());
    println!("done: {}", now.elapsed().as_secs());
}
