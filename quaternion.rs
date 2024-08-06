pub struct Quaternion<T> {
    w: T,
    x: T,
    y: T,
    z: T,
}


impl<T> Quaternion<T> {
    pub fn new(w: T, x: T, y: T, z: T) -> Self {
        Quaternion { w, x, y, z }
    }
    
    pub fn w(&self) -> T where T: Copy {self.w}
    pub fn x(&self) -> T where T: Copy {self.x}
    pub fn y(&self) -> T where T: Copy {self.y}
    pub fn z(&self) -> T where T: Copy {self.z}

    pub fn conjugate(&self) -> Quaternion<T>
    where
        T: std::ops::Neg<Output = T> + Copy,
    {
        Quaternion {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn norm(&self) -> T
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
    {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn inverse(&self) -> Quaternion<T>
    where
        T: std::ops::Div<Output = T> + std::ops::Neg<Output = T> +
           std::ops::Mul<Output = T> + std::ops::Add<Output = T> +  Copy,
    {
        let module = self.norm();
        Quaternion{
            w: self.w / module,
            x: - self.x / module,
            y: - self.y / module,
            z: - self.z / module
        }
    }
}


impl<'a, T> std::ops::Add for &'a Quaternion<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Quaternion<T>;
    fn add(self, other: &'a Quaternion<T>) -> Quaternion<T> {
        Quaternion {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


impl<'a, T> std::ops::Sub for &'a Quaternion<T> 
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Quaternion<T>;
    fn sub(self, other: &'a Quaternion<T>) -> Quaternion<T> {
        Quaternion {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl<'a, T> std::ops::Mul <&'a Quaternion<T>> for &'a Quaternion<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    type Output = Quaternion<T>;
    fn mul(self, other: &'a Quaternion<T>) -> Quaternion<T>{
        Quaternion{
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w
        }
    }
}


impl<'a, T> std::ops::Mul<T> for &'a Quaternion<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Quaternion<T>;

    fn mul(self, scalar: T) -> Self::Output {
        Quaternion {
            w: self.w * scalar,
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T> Quaternion<T>
where
    T: num::traits::Float + num::traits::FloatConst + std::ops::Div<Output = T> + std::ops::Mul<Output = T>,
{
    pub fn rotate(&self, shaft: & Quaternion<T>, angle: T, division: T) -> Quaternion<T>{
        let half_angle = angle / division;
        let sine = half_angle.sin();
        let exp = Quaternion{w: half_angle.cos(),
                             x: sine * shaft.x(),
                             y: sine * shaft.y(),
                             z: sine * shaft.z()};
        &(&exp * self) * &exp.inverse()
    }
}
