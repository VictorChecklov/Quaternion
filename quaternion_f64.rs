use std::ops::{Add, Sub, Mul, Div};
// Quaternion for f64
// Tatsächlich ist es Quaternion-Long
pub struct QuaternionL{
    w: f64,
    x: f64,
    y: f64,
    z: f64
}
// Some basic implements for QuaternionL
impl QuaternionL{
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> QuaternionL {QuaternionL{w, x, y, z}}

    pub fn w(&self) -> f64 {self.w}
    pub fn x(&self) -> f64 {self.x}
    pub fn y(&self) -> f64 {self.y}
    pub fn z(&self) -> f64 {self.z}
}
// Overload " + " for QuaternionL
impl<'a> Add for &'a QuaternionL{
    type Output = QuaternionL;
    fn add(self, rhs: &'a QuaternionL) -> QuaternionL{
        QuaternionL{
            w: self.w + rhs.w,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
// Overload " - " for QuaternionL
impl<'a> Sub for &'a QuaternionL{
    type Output = QuaternionL;
    fn sub(self, rhs: &'a QuaternionL) -> QuaternionL{
        QuaternionL{
            w: self.w - rhs.w,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}
// Overload " * " for QuaternionL and QuaternionL
impl<'a> Mul<&'a QuaternionL> for &'a QuaternionL{
    type Output = QuaternionL;
    fn mul (self, rhs: &QuaternionL) -> QuaternionL{
        QuaternionL {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w
        }
    }
}
//Overload " * " for QuaternionL and Scalar
impl<'a> Mul<f64> for &'a QuaternionL{
    type Output = QuaternionL;
    fn mul (self, scalar: f64) -> QuaternionL{
        QuaternionL{
            w: self.w * scalar,
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
// Overload " / " for QuaternionL and Scalar
impl<'a> Div<f64> for &'a QuaternionL{
    type Output = QuaternionL;
    fn div(self, scalar: f64) -> QuaternionL{
        QuaternionL{
            w: self.w / scalar,
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
impl QuaternionL{
    // dot for pure quaternion ore vector
    pub fn dot_v(&self, other:&QuaternionL) -> f64{
        if self.w != 0.0 || other.w != 0.0{
            panic!("Dies ist kein Vektor: Der Realteil ist ungleich null!")
        }else{
        self.x * other.x + self.y * other.y + self. z * other.z
        }
    }
    // dot for normal quaternion
    pub fn dot(&self, other:&QuaternionL) -> f64{
        self.w * other.w + self.x * other.x + self.y * other.y + self. z * other.z
    }
    // get the l2 norm/module
    pub fn norm(&self) -> f64{
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    // normalize a quaternion
    pub fn normalize(&self) -> QuaternionL{
        let norm = self.norm();
        QuaternionL{
            w: self.w / norm,
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm
        }
    }
    // get the conjugate of a quaternion
    pub  fn conjugate(&self) -> QuaternionL{
        QuaternionL{
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
    // rotate the quaternion
    pub fn rotate(&self, axis: &QuaternionL, angle: f64) -> QuaternionL{
        let half_angle = angle / 2.0;
        let sine = half_angle.sin();
        let rotation_quaternion = QuaternionL{
            w: half_angle.cos(),
            x: sine * axis.x(),
            y: sine * axis.y(),
            z: sine * axis.z()
        }.normalize();
        &(&rotation_quaternion * self) * &rotation_quaternion.conjugate()
    }
    // turn quaternion into matrix
    pub fn mat(&self) -> [[f64; 3]; 3]{
        let w = self.w;
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w_pow = w * w;
        let x_pow = x * x;
        let y_pow = y * y;
        let z_pow = z * z;
        let wx = w * x;
        let wz = w * z;
        let wy = w * y;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        [[w_pow + x_pow - y_pow - z_pow, 2.0 * xy - 2.0 * wz, 2.0 * xz + 2.0 * wy],
        [2.0 * xy + 2.0 * wz, w_pow - x_pow + y_pow - z_pow, 2.0 * yz - 2.0 * wx],
        [2.0 * xz - 2.0 * wy, 2.0 * yz + 2.0 * wx, w_pow - x_pow - y_pow + z_pow]]
    }
    // Normalized Linear Interpolation -- Nlerp
    pub fn nlerp(&self, target: &QuaternionL, t: f64) -> QuaternionL{
        &(&(self * (1.0 - t)) + &(target * t)) / (&(self * (1.0 - t)) + &(target * t)).norm()
    }
    // Spherical Linear Interpolation -- Slerp
    pub fn slerp(&self, target: &QuaternionL, t: f64) -> QuaternionL{
        let cosine = self.dot_v(target);
        let cosine = if cosine < 0.0 {- cosine} else {cosine};

        if cosine >= 0.9998 {
            &(self * (1.0 - t)) + &(target * t)
        } else {

            let angle = cosine.acos();
            &(&(self * ((1.0- t) * angle).sin()) + &(target * (t * angle).sin())) / angle.sin()
        }
    }
    //Spherical and quadrangle -- Squad
    pub fn squad(&self, q1: &QuaternionL, q2: &QuaternionL, q3: QuaternionL, t: f64) -> QuaternionL{
        let m1 = self.slerp(&q3, t);
        m1.slerp(&q1.slerp(&q2, t), 2.0 * t * (1.0 - t))
    }
}
