# Quaternion

Simple implementation of quaternions 

In fact, The main part of this reposity is the Toturial.md, not the way I implement the quaternion.

Anyway

__This is my first venture into Computer Graphics, and I believe I'll learn something really cool__

__You can read Tutorial.md for more information about Quaternions__

## Introductions
- Struct:
  - `QuaternionL`: quaternions for f64
  - `QuaternionF`: quaternions for f32
- Functions:
  - Reloaded: Add, Sub, Mul for Quaterion-Quaternion and Quaternion-Scalar
  - `dot()`: q1.dot(q2) -> f32/f64
  - `dot_v()`: q1.dot(q2) -> f32/f64 __q1, q2 must be vector(real part is zero)__
  - `norm()`: q.norm() -> f32/f64
  - `mormalize()`: q.normalize() -> Quaternion
  - `conjugate()`: q.conjugate() -> Quaternion
  - `rotate()`: q.rotate(axis, angle) -> Quaternion
  - `mat()`: q.mat() -> [[f32/f64; 3]; 3]
  - `nlerp()`: q.nlerp(target, t) -> Quaternion
  - `slerp()`: q.slerp(target, t) -> Quaternion
  - `squad()`: q.squad(q0, q1, q2, q3, t) -> Quaternion
## Example
```rust
fn main(){
    let q1 = QuaternionL::new(0.0, 1.0, 0.0, 0.0);
    let shaft = QuaternionL::new(0.0, 0.0, 0.0, 1.0);
    let q2  = q1.rotate(&shaft, std::f64::consts::FRAC_PI_2);
    println!("Quaternion-Rotation: {} + {}i + {}j + {}k", q2.w(), q2.x(), q2.y(), q2.z());

}
```
