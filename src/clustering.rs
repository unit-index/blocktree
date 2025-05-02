use crate::error::BlocktreeError;
use nalgebra::{DMatrix, DVector};
use rand::Rng;

pub trait Clustering {
    fn compute_fiedler_vector(&self) -> Result<DVector<f64>, BlocktreeError>;
    fn partition_nodes(&self, fiedler_vector: &DVector<f64>) -> (Vec<u32>, Vec<u32>);
}

pub struct SpectralClustering {
    nodes: Vec<u32>,
    latency_matrix: DMatrix<f64>,
}

impl SpectralClustering {
    pub fn new(node_count: u32) -> Self {
        let nodes = (0..node_count).collect();
        let latency_matrix = Self::generate_latency_matrix(node_count);
        SpectralClustering {
            nodes,
            latency_matrix,
        }
    }

    fn generate_latency_matrix(node_count: u32) -> DMatrix<f64> {
        let mut rng = rand::thread_rng();
        let mut matrix = DMatrix::zeros(node_count as usize, node_count as usize);
        for i in 0..node_count as usize {
            for j in 0..node_count as usize {
                if i == j {
                    matrix[(i, j)] = 0.0;
                } else {
                    matrix[(i, j)] = rng.gen_range(10.0..100.0);
                    matrix[(j, i)] = matrix[(i, j)];
                }
            }
        }
        matrix
    }
}

impl Clustering for SpectralClustering {
    fn compute_fiedler_vector(&self) -> Result<DVector<f64>, BlocktreeError> {
        let n = self.nodes.len();
        let mut adjacency = DMatrix::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    adjacency[(i, j)] = 1.0 / (self.latency_matrix[(i, j)] + 1e-6);
                }
            }
        }
        let degree = DMatrix::from_diagonal(&DVector::from_iterator(
            n,
            (0..n).map(|i| adjacency.row(i).sum()),
        ));
        let laplacian = degree - adjacency;
        let eigen = laplacian.symmetric_eigen();
        let mut eigen_pairs: Vec<(f64, usize)> = eigen
            .eigenvalues
            .iter()
            .enumerate()
            .map(|(i, &val)| (val, i))
            .collect();
        eigen_pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let fiedler_index = eigen_pairs.get(1).map(|p| p.1).ok_or_else(|| {
            BlocktreeError::ClusteringError("Failed to compute Fiedler vector".to_string())
        })?;
        Ok(eigen.eigenvectors.column(fiedler_index).into())
    }

    fn partition_nodes(&self, fiedler_vector: &DVector<f64>) -> (Vec<u32>, Vec<u32>) {
        let mut cluster1 = Vec::new();
        let mut cluster2 = Vec::new();
        for (i, &val) in fiedler_vector.iter().enumerate() {
            if val >= 0.0 {
                cluster1.push(self.nodes[i]);
            } else {
                cluster2.push(self.nodes[i]);
            }
        }
        (cluster1, cluster2)
    }
}