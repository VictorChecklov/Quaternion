use std::ops::{Add, Sub, Mul, Div};
// Quaternion for f32
// Tatsächlich ist es Quaternion-Float
pub struct QuaternionF{
    w: f32,
    x: f32,
    y: f32,
    z: f32
}
// Some basic implements for QuaternionF
impl QuaternionF{
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> QuaternionF {QuaternionF{w, x, y, z}}

    pub fn w(&self) -> f32 {self.w}
    pub fn x(&self) -> f32 {self.x}
    pub fn y(&self) -> f32 {self.y}
    pub fn z(&self) -> f32 {self.z}
}
// Overload " + " for QuaternionF
impl<'a> Add for &'a QuaternionF{
    type Output = QuaternionF;
    fn add(self, rhs: &'a QuaternionF) -> QuaternionF{
        QuaternionF{
            w: self.w + rhs.w,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
// Overload " - " for QuaternionF
impl<'a> Sub for &'a QuaternionF{
    type Output = QuaternionF;
    fn sub(self, rhs: &'a QuaternionF) -> QuaternionF{
        QuaternionF{
            w: self.w - rhs.w,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}
// Overload " * " for QuaternionF and QuaternionF
impl<'a> Mul<&'a QuaternionF> for &'a QuaternionF{
    type Output = QuaternionF;
    fn mul (self, rhs: &QuaternionF) -> QuaternionF{
        QuaternionF {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w
        }
    }
}
//Overload " * " for QuaternionF and Scalar
impl<'a> Mul<f32> for &'a QuaternionF{
    type Output = QuaternionF;
    fn mul (self, scalar: f32) -> QuaternionF{
        QuaternionF{
            w: self.w * scalar,
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
// Overload " / " for QuaternionF and Scalar
impl<'a> Div<f32> for &'a QuaternionF{
    type Output = QuaternionF;
    fn div(self, scalar: f32) -> QuaternionF{
        QuaternionF{
            w: self.w / scalar,
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
impl QuaternionF{
    // dot for pure quaternion or vector
    pub fn dot_v(&self, other:&QuaternionF) -> f32{
        if self.w != 0.0 || other.w != 0.0{
            panic!("Dies ist kein Vektor: Der Realteil ist ungleich null!")
        }else{
            self.x * other.x + self.y * other.y + self. z * other.z
        }
    }
    // dot for normal quaternion
    pub fn dot(&self, other:&QuaternionF) -> f32{
        self.w * other.w + self.x * other.x + self.y * other.y + self. z * other.z
    }
    // get the l2 norm/module
    pub fn norm(&self) -> f32{
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    // normalize a quaternion
    pub fn normalize(&self) -> QuaternionF{
        let norm = self.norm();
        QuaternionF{
            w: self.w / norm,
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm
        }
    }
    // get the conjugate of a quaternion
    pub  fn conjugate(&self) -> QuaternionF{
        QuaternionF{
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
    // rotate the quaternion
    pub fn rotate(&self, axis: &QuaternionF, angle: f32) -> QuaternionF{
        let half_angle = angle / 2.0;
        let sine = half_angle.sin();
        let rotation_quaternion = QuaternionF{
            w: half_angle.cos(),
            x: sine * axis.x(),
            y: sine * axis.y(),
            z: sine * axis.z()
        }.normalize();
        &(&rotation_quaternion * self) * &rotation_quaternion.conjugate()
    }
    // turn quaternion into matrix
    pub fn mat(&self) -> [[f32; 3]; 3]{
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
    pub fn nlerp(&self, target: &QuaternionF, t: f32) -> QuaternionF{
        &(&(self * (1.0 - t)) + &(target * t)) / (&(self * (1.0 - t)) + &(target * t)).norm()
    }
    // Spherical Linear Interpolation -- Slerp
    pub fn slerp(&self, target: &QuaternionF, t: f32) -> QuaternionF{
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
    pub fn squad(&self, q1: &QuaternionF, q2: &QuaternionF, q3: QuaternionF, t: f32) -> QuaternionF{
        let m1 = self.slerp(&q3, t);
        m1.slerp(&q1.slerp(&q2, t), 2.0 * t * (1.0 - t))
    }
}
