/*
 * Generate Clustered Data
 * =======================
 *
 * Create a dataset that can be used to test a K-Means clustering
 * algorithm.
 */

use ndarray::Array2;
use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;
use std::error::Error;
use std::io;

/* Settings */
const CENTROIDS: [f64; 6] = [22.5, 40.5, 38.0, 50.0, 25.5, 48.0];
const NOISE: f64 = 1.8;
const SAMPLE_PER_CENTROID: usize = 2000;

fn generate_data(
    centroids: &Array2<f64>,
    points_per_centroid: usize,
    noise: f64,
) -> Result<Array2<f64>, Box<dyn Error>> {
    /* Verify the function input. */
    assert!(!centroids.is_empty(), "Centroids cannot be empty!");
    assert!(noise >= 0f64, "Noise must be non-negative!");

    let rows = centroids.shape()[0];
    let cols = centroids.shape()[1];

    let mut rng = thread_rng();
    let normal_rv = Normal::new(0f64, noise)?;

    let mut raw_cluster_data = Vec::with_capacity(rows * points_per_centroid * cols);

    for _ in 0..points_per_centroid {
        for cent in centroids.rows() {
            let mut point = Vec::with_capacity(centroids.shape()[1]);

            for feature in cent.into_iter() {
                point.push(feature + normal_rv.sample(&mut rng));
            }

            /* Save the point to the cluster data. */
            raw_cluster_data.extend(point);
        }
    }

    return Ok(Array2::from_shape_vec(
        (rows * points_per_centroid, cols),
        raw_cluster_data,
    )?);
}

fn main() -> Result<(), Box<dyn Error>> {
    let centroids = Array2::from_shape_vec((3, 2), CENTROIDS.to_vec())?;

    let samples = generate_data(&centroids, SAMPLE_PER_CENTROID, NOISE)?;

    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["height", "length"])?;

    for sample in samples.rows() {
        let mut sample_iter = sample.into_iter();
        writer.serialize((sample_iter.next().unwrap(), sample_iter.next().unwrap()))?;
    }

    return Ok(());
}
