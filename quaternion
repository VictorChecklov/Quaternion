#![forbid(unsafe_code)]

use std::fmt::{Display, Debug};
use std::ops::{Add, Mul, Sub, Div, Neg};
use num_traits::{Zero, One, Float};

#[derive(Debug, Clone)]
pub struct Quaternion<T> {
    w: T,
    x: T,
    y: T,
    z: T
}

impl<T> Quaternion<T>
where T: Copy{
    pub fn new(w: T, x: T, y: T, z: T) -> Self {
        Quaternion { w, x, y, z }
    }

    pub fn w(&self) -> T { self.w }
    pub fn x(&self) -> T { self.x }
    pub fn y(&self) -> T { self.y }
    pub fn z(&self) -> T { self.z }
}

impl<T> Add for &Quaternion<T>
where
    T: Add<Output = T> + Copy{
    type Output = Quaternion<T>;
    fn add(self, rhs: &Quaternion<T>) -> Quaternion<T> {
        Quaternion{
            w: self.w + rhs.w,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T> Add for Quaternion<T>
where
    T: Add<Output = T> + Copy{
    type Output = Quaternion<T>;
    fn add(self, rhs: Quaternion<T>) -> Quaternion<T> {
        Quaternion{
            w: self.w + rhs.w,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T> Sub for & Quaternion<T>
where
    T: Sub<Output = T> + Copy{
    type Output = Quaternion<T>;
    fn sub(self, rhs: &Quaternion<T>) -> Quaternion<T> {
        Quaternion{
            w: self.w - rhs.w,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T> Sub for Quaternion<T>
where
    T: Sub<Output = T> + Copy{
    type Output = Quaternion<T>;
    fn sub(self, rhs: Quaternion<T>) -> Quaternion<T> {
        Quaternion{
            w: self.w - rhs.w,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T> Mul for &Quaternion<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy{
    type Output = Quaternion<T>;
    fn mul(self, rhs: &Quaternion<T>) -> Quaternion<T> {
        Quaternion {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w
        }
    }
}

impl<T> Mul for Quaternion<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy{
    type Output = Quaternion<T>;
    fn mul(self, rhs: Quaternion<T>) -> Quaternion<T> {
        Quaternion {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w
        }
    }
}

impl<T> Mul<T> for &Quaternion<T>
where
    T: Mul<Output = T> + Copy{
    type Output = Quaternion<T>;
    fn mul(self, scalar:T) -> Quaternion<T> {
        Quaternion{
            w: self.w * scalar,
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl<T> Div<T> for &Quaternion<T>
where
    T: Div<Output = T> + Copy{
    type Output = Quaternion<T>;
    fn div(self, scalar:T) -> Quaternion<T> {
        Quaternion{
            w: self.w / scalar,
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar
        }
    }
}

impl<T> Quaternion<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> +
       Zero + One +  Copy + Display{
    pub fn dot(self, other: &Quaternion<T>) -> T {
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn dot_v(&self, other: &Quaternion<T>) -> Result<T, String> {
        if self.w.is_zero() || other.w.is_zero(){
            Err("Dot for Vector: Non-vector between two quaternions.".to_string())
        } else {
            Ok(self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z)
        }
    }

    pub fn conjugate(&self) -> Quaternion<T> {
        Quaternion{
            w: self.w,
            x: - self.x,
            y: - self.y,
            z: - self.z
        }
    }

    pub fn mat(&self) -> [[T; 3]; 3] {
        let w = self.w;
        let x = self.x;
        let y = self.y;
        let z = self.z;
        [[w * w + x * x - y * y - z * z,
          x * y + x * y - w * z - w * z,
          x * z + w * z + w * y + w * y],
         [x * y + x * y + w * z + w * z,
          w * w - x * x + y * y - z * z,
          y * z + y * z - w * x - w * x],
         [x * z + x * z - w * y - w * y,
          y * z + y * z + w * x + w * x,
          w * w - x * x - y * y + z * z]]
    }

    pub fn mat_homogeneous(&self) -> [[T; 4]; 4] {
        let w = self.w;
        let x = self.x;
        let y = self.y;
        let z = self.z;
        [[w * w + x * x - y * y - z * z,
          x * y + x * y - w * z - w * z,
          x * z + w * z + w * y + w * y, T::zero()],
         [x * y + x * y + w * z + w * z,
          w * w - x * x + y * y - z * z,
          y * z + y * z - w * x - w * x, T::zero()],
         [x * z + x * z - w * y - w * y,
          y * z + y * z + w * x + w * x,
          w * w - x * x - y * y + z * z, T::zero()],
         [T::zero(), T::zero(), T::zero(), T::one()]]
    }

    pub fn display(&self) -> String {
        format!("{} + {}i + {}j + {}k", self.w, self.x, self.y, self.z)
    }
}

impl<T> Quaternion<T>
where
    T: Float + Display + Debug{

    pub fn norm(&self) -> T {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Result<Quaternion<T>, String> {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;
        let norm = self.norm();
        if norm == T::zero() {
            Err("Normalize: Cannot divide by zero".to_string())
        } else {
            Ok(Quaternion::new(w / norm, x / norm, y / norm, z / norm))
        }
    }

    pub fn rotate(&self, axis: &Quaternion<T>, angle: T) -> Result<Quaternion<T>, String> {
        if axis.w == T::zero() {
            Err("Rotate: Axis is not a vector".to_string())
        } else {
            let axis = axis.normalize()
                .map_err(|error|format!("Rotate -> {}",error))?;

            let half_angle = angle / T::from(2).unwrap();
            let sine = half_angle.sin();
            let rotation_quaternion = Quaternion{
                w: half_angle.cos(),
                x: sine * axis.x,
                y: sine * axis.y,
                z: sine * axis.z};
            Ok(&(&rotation_quaternion * self) * &rotation_quaternion.conjugate())
        }
    }

    pub fn nlerp(&self, target: &Self, t: T) -> Result<Quaternion<T>, String> {
        if t < T::zero() || t > T::one() {
            return Err("NLERP: t must be in [0, 1]".to_string());
        }

        let one_minus_t = T::one() - t;
        let result = &(self * one_minus_t) + &(target * t);
        let norm = result.norm();

        if norm == T::zero() {
            Err("NLERP: Norm of result is zero. self".to_string())
        } else {
            Ok(&result / norm)
        }
    }

    pub fn slerp(&self, target: &Quaternion<T>, t: T) -> Result<Quaternion<T>, String> {
        let cosine = self.dot_v(target)
            .map_err(|error|format!("SLERP -> {}", error))?;

        let cosine = if cosine < T::zero() { -cosine } else { cosine };
        if cosine >= T::from(0.9998).unwrap() {
            Ok(self.nlerp(target, t)?)
        } else {

            let angle = cosine.acos();
            let sin_angle = angle.sin();
            let one_minus_t_angle = (T::one() - t) * angle.sin();
            let t_angle = t * angle.sin();
            let result = &(self * one_minus_t_angle) + &(target * t_angle);

            Ok(&result / sin_angle)
        }
    }

    pub fn squad(&self, q1: &Quaternion<T>, q2: &Quaternion<T>, q3: &Quaternion<T>, t: T)
                 -> Result<Quaternion<T>, String> {
        let m1 = self.slerp(&q3, t)
            .map_err(|error| format!("SQUAD [1] -> {}", error))?;
        let m2 = q1.slerp(&q2, t)
            .map_err(|error| format!("SQUAD [2] -> {}", error))?;
        let final_t = (t + t) * (T::one() - t);

        m1.slerp(&m2, final_t)
            .map_err(|error| format!("SQUAD [3] -> {}", error))
    }
}
