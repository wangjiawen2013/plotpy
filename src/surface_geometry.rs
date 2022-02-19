use crate::{StrError, Surface};
use russell_lab::{generate3d, suq_cos, suq_sin, Matrix};
use std::f64::consts::PI;

impl Surface {
    /// Draws a plane that has a normal vector with a non-zero z (nzz) component
    ///
    /// The plane may be perpendicular to z if n = (0,0,1)
    ///
    /// # Input
    ///
    /// * `p` -- (len=3) point on plane
    /// * `n` -- (len=3) normal vector
    /// * `xmin` and `xmax` -- limits along x
    /// * `ymin` and `ymax` -- limits along y
    /// * `nx` -- number of divisions along x
    /// * `ny` -- number of divisions along y
    ///
    /// # Output
    ///
    /// * `x`, `y`, `z` -- the coordinates of all points as in a meshgrid
    pub fn draw_plane_nzz(
        &mut self,
        p: &[f64],
        n: &[f64],
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
        nx: usize,
        ny: usize,
    ) -> Result<(Matrix, Matrix, Matrix), StrError> {
        if p.len() != 3 || n.len() != 3 {
            return Err("p.len() and n.len() must be equal to 3");
        }
        if f64::abs(n[2]) < 1e-10 {
            return Err("the z-component of the normal vector cannot be zero");
        }
        if nx < 1 || ny < 1 {
            return Err("nx and ny must be greater than or equal to 2");
        }
        let d = -n[0] * p[0] - n[1] * p[1] - n[2] * p[2];
        let (x, y, z) = generate3d(xmin, xmax, ymin, ymax, nx + 1, ny + 1, |x, y| {
            (-d - n[0] * x - n[1] * y) / n[2]
        });
        self.draw(&x, &y, &z);
        Ok((x, y, z))
    }

    /// Draws a hemisphere
    ///
    /// # Input
    ///
    /// * `c` -- (len=3) center coordinates
    /// * `r` -- radius
    /// * `alpha_min` -- min α angle in [-180, 180) degrees
    /// * `alpha_max` -- max α angle in (-180, 180] degrees
    /// * `n_alpha` -- number of divisions along α
    /// * `n_theta` -- number of divisions along θ
    /// * `cup` -- upside-down; like a cup
    ///
    /// # Output
    ///
    /// * `x`, `y`, `z` -- the coordinates of all points as in a meshgrid
    pub fn draw_hemisphere(
        &mut self,
        c: &[f64],
        r: f64,
        alpha_min: f64,
        alpha_max: f64,
        n_alpha: usize,
        n_theta: usize,
        cup: bool,
    ) -> Result<(Matrix, Matrix, Matrix), StrError> {
        if c.len() != 3 {
            return Err("c.len() must be equal to 3");
        }
        if n_alpha < 1 || n_theta < 1 {
            return Err("n_alpha and n_theta must be greater than or equal to 1");
        }
        let a_min = alpha_min * PI / 180.0;
        let a_max = alpha_max * PI / 180.0;
        let d_alpha = (a_max - a_min) / (n_alpha as f64);
        let d_theta = (PI / 2.0) / (n_theta as f64);
        let mut x = Matrix::new(n_alpha + 1, n_theta + 1);
        let mut y = Matrix::new(n_alpha + 1, n_theta + 1);
        let mut z = Matrix::new(n_alpha + 1, n_theta + 1);
        for i in 0..n_alpha + 1 {
            let alpha = a_min + (i as f64) * d_alpha;
            for j in 0..n_theta + 1 {
                let theta = (j as f64) * d_theta;
                if cup {
                    x[i][j] = c[0] + r * f64::cos(alpha) * f64::sin(theta);
                    y[i][j] = c[1] + r * f64::sin(alpha) * f64::sin(theta);
                    z[i][j] = c[2] - r * f64::cos(theta);
                } else {
                    x[i][j] = c[0] + r * f64::cos(alpha) * f64::sin(theta);
                    y[i][j] = c[1] + r * f64::sin(alpha) * f64::sin(theta);
                    z[i][j] = c[2] + r * f64::cos(theta);
                }
            }
        }
        self.draw(&x, &y, &z);
        Ok((x, y, z))
    }

    /// Draws a superquadric (includes sphere, super-ellipsoid, and super-hyperboloid)
    ///
    /// # Input
    ///
    /// * `c` -- (len=3) center coordinates
    /// * `r` -- (len=3) radii
    /// * `k` -- (len=3) exponents
    /// * `alpha_min` -- min α angle in [-180, 180) degrees
    /// * `alpha_max` -- max α angle in (-180, 180] degrees
    /// * `theta_min` -- min θ angle in [-90, 90) degrees
    /// * `theta_max` -- max θ angle in (-90, 90] degrees
    /// * `n_alpha` -- number of divisions along α
    /// * `n_theta` -- number of divisions along θ
    ///
    /// # Output
    ///
    /// * `x`, `y`, `z` -- the coordinates of all points as in a meshgrid
    ///
    /// Reference: <https://en.wikipedia.org/wiki/Superquadrics>
    pub fn draw_superquadric(
        &mut self,
        c: &[f64],
        r: &[f64],
        k: &[f64],
        alpha_min: f64,
        alpha_max: f64,
        theta_min: f64,
        theta_max: f64,
        n_alpha: usize,
        n_theta: usize,
    ) -> Result<(Matrix, Matrix, Matrix), StrError> {
        if c.len() != 3 || r.len() != 3 || k.len() != 3 {
            return Err("c.len(), r.len(), and k.len() must be equal to 3");
        }
        if n_alpha < 1 || n_theta < 1 {
            return Err("n_alpha and n_theta must be greater than or equal to 1");
        }
        if k[0] < 0.0 || k[1] < 0.0 || k[2] < 0.0 {
            return Err("exponents k must be greater than zero");
        }
        let (aa, bb, cc) = (2.0 / k[0], 2.0 / k[1], 2.0 / k[2]);
        let a_min = alpha_min * PI / 180.0;
        let a_max = alpha_max * PI / 180.0;
        let t_min = theta_min * PI / 180.0;
        let t_max = theta_max * PI / 180.0;
        let d_alpha = (a_max - a_min) / (n_alpha as f64);
        let d_theta = (t_max - t_min) / (n_theta as f64);
        let mut x = Matrix::new(n_alpha + 1, n_theta + 1);
        let mut y = Matrix::new(n_alpha + 1, n_theta + 1);
        let mut z = Matrix::new(n_alpha + 1, n_theta + 1);
        for i in 0..n_alpha + 1 {
            let alpha = a_min + (i as f64) * d_alpha;
            for j in 0..n_theta + 1 {
                let theta = t_min + (j as f64) * d_theta;
                x[i][j] = c[0] + r[0] * suq_cos(theta, aa) * suq_cos(alpha, aa);
                y[i][j] = c[1] + r[1] * suq_cos(theta, bb) * suq_sin(alpha, bb);
                z[i][j] = c[2] + r[2] * suq_sin(theta, cc);
            }
        }
        self.draw(&x, &y, &z);
        Ok((x, y, z))
    }

    /// Draws a sphere
    ///
    /// # Input
    ///
    /// * `c` -- (len=3) center coordinates
    /// * `r` -- radius
    /// * `n_alpha` -- number of divisions along α
    /// * `n_theta` -- number of divisions along θ
    ///
    /// # Output:
    ///
    /// * `x`, `y`, `z` -- the coordinates of all points as in a meshgrid
    pub fn draw_sphere(
        &mut self,
        c: &[f64],
        r: f64,
        n_alpha: usize,
        n_theta: usize,
    ) -> Result<(Matrix, Matrix, Matrix), StrError> {
        if c.len() != 3 {
            return Err("c.len() must be equal to 3");
        }
        if n_alpha < 1 || n_theta < 1 {
            return Err("n_alpha and n_theta must be greater than or equal to 1");
        }
        let (alpha_min, alpha_max) = (-180.0, 180.0);
        let (theta_min, theta_max) = (-90.0, 90.0);
        self.draw_superquadric(
            c,
            &[r, r, r],
            &[2.0, 2.0, 2.0],
            alpha_min,
            alpha_max,
            theta_min,
            theta_max,
            n_alpha,
            n_theta,
        )
    }
}
