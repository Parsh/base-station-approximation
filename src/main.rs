use multilateration::multilaterate;
use plotter::plot_smartphone_data;
use rand::Rng;
use std::collections::HashMap;

const RADIUS: f64 = 600.0;

mod multilateration;
mod plotter;

#[derive(Debug, Clone)]
struct SmartphoneData {
    ci: i32,
    rssi: f64,
    x: f64,
    y: f64,
    max_rssi: f64,
    time: i32,
    distance: f64,
    noisy_rssi: f64,
    noisy_distance: f64,
    scaled_distance: f64,
}

fn rssi_to_distance(rssi: f64, max_rssi: f64) -> f64 {
    ((rssi - max_rssi) * (RADIUS / (max_rssi - 1.0))).abs()
}

fn add_rssi_noise(rssi: f64, noise_stddev: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rssi + rng.gen_range(-noise_stddev..noise_stddev)
}

fn main() {
    let mut smartphone_data: Vec<SmartphoneData> = vec![
        SmartphoneData {
            ci: 12801,
            rssi: 10.0,
            x: 1450.0,
            y: 1040.0,
            max_rssi: 61.0,
            time: 100045,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12801,
            rssi: 53.0,
            x: 936.0,
            y: 752.0,
            max_rssi: 61.0,
            time: 100230,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12801,
            rssi: 36.0,
            x: 850.0,
            y: 600.0,
            max_rssi: 61.0,
            time: 110002,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12801,
            rssi: 176.0,
            x: 827.2,
            y: 850.4,
            max_rssi: 251.0,
            time: 129885,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12801,
            rssi: 201.0,
            x: 904.0,
            y: 728.0,
            max_rssi: 251.0,
            time: 134546,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12802,
            rssi: 108.0,
            x: 1468.0,
            y: 2716.8,
            max_rssi: 251.0,
            time: 156778,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12804,
            rssi: 9.0,
            x: 219.2,
            y: 2000.0,
            max_rssi: 251.0,
            time: 164747,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 0,
            rssi: 0.0,
            x: 339.2,
            y: 2614.4,
            max_rssi: 251.0,
            time: 169567,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12801,
            rssi: 10.0,
            x: 784.0,
            y: 1262.0,
            max_rssi: 61.0,
            time: 4725,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12802,
            rssi: 24.0,
            x: 1720.0,
            y: 2050.8,
            max_rssi: 61.0,
            time: 7321,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12803,
            rssi: 8.0,
            x: 1720.0,
            y: 2050.8,
            max_rssi: 61.0,
            time: 7321,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12802,
            rssi: 87.0,
            x: 1984.0,
            y: 2313.6,
            max_rssi: 251.0,
            time: 94521,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12803,
            rssi: 17.0,
            x: 2216.0,
            y: 2118.4,
            max_rssi: 251.0,
            time: 136744,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 12804,
            rssi: 70.0,
            x: 800.0,
            y: 2434.4,
            max_rssi: 251.0,
            time: 156554,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
        SmartphoneData {
            ci: 0,
            rssi: 0.0,
            x: 161.6,
            y: 2748.8,
            max_rssi: 251.0,
            time: 174677,
            distance: 0.0,
            noisy_rssi: 0.0,
            noisy_distance: 0.0,
            scaled_distance: 0.0,
        },
    ];

    for data in smartphone_data.iter_mut() {
        data.distance = rssi_to_distance(data.rssi, data.max_rssi);
    }

    let mut ci_grouped_data: HashMap<i32, Vec<SmartphoneData>> = HashMap::new();
    let mut unassigned_points: Vec<SmartphoneData> = Vec::new();
    for data in smartphone_data.iter() {
        if data.distance <= RADIUS {
            ci_grouped_data
                .entry(data.ci)
                .or_insert(vec![])
                .push(data.clone());
        } else {
            unassigned_points.push(data.clone());
        }
    }
    println!("Grouped Data: {:#?}", ci_grouped_data);

    let mut approximated_positions: HashMap<i32, Option<(f64, f64)>> = HashMap::new();
    for (ci, ci_data) in ci_grouped_data.iter() {
        approximated_positions.insert(*ci, multilaterate(ci_data));
    }
    println!("Approximated Positions: {:#?}", approximated_positions);

    plot_smartphone_data(&smartphone_data, &approximated_positions, "./assets/base_station_approximation.png");


    for data in smartphone_data.iter_mut() {
        data.noisy_rssi = add_rssi_noise(data.rssi, 3.5);
        data.noisy_distance = rssi_to_distance(data.noisy_rssi, data.max_rssi);
    }

    for (ci, ci_data) in ci_grouped_data.iter_mut() {
        for d in ci_data.iter_mut() {
            d.distance = rssi_to_distance(d.noisy_rssi, d.max_rssi);
        }
        approximated_positions.insert(*ci, multilaterate(ci_data));
    }

    println!(
        "Approximated Positions with Noise: {:#?}",
        approximated_positions
    );

    plot_smartphone_data(&smartphone_data, &approximated_positions, "./assets/base_station_approximation_with_noise.png");
}
