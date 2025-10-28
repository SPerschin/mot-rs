use crate::mot::SimpleBlob;

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    dot_product / (norm_a * norm_b)
}

pub fn reid_distance(a: &SimpleBlob, b: &SimpleBlob) -> f32 {
    if let (Some(features_a), Some(features_b)) = (a.get_features(), b.get_features()) {
        // Cosine similarity is in [-1, 1], so distance is in [0, 2]
        1.0 - cosine_similarity(features_a, features_b)
    } else {
        // If features are missing, return a neutral distance.
        // A distance of 1.0 results in a normalized score of 0.5,
        // which neither encourages nor discourages a match.
        1.0
    }
}
