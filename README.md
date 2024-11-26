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
  - `Quaternion` : generic quaternion
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
  - `squad()`: q.squad(q1, q2, q3, t) -> Quaternion

    __For the generic quaternion__
    - `mat_homogeneous()`: mat_homogeneous() -> [[T; 4]; 4] for `homogeneous coordinates`
    - `display()`: display() -> String for the output of the quaternion
    - Optimized error handling
      
## Example
```rust
fn main() {
    let p = Quaternion::new(0, 0, 3, 2);
    let q = Quaternion::new(1, 2, 3, 4);
    let out = &p * &q;
    assert_eq!(out.w, -17);
    assert_eq!(out.x, 6);
    assert_eq!(out.y, 7);
    assert_eq!(out.z, -4);
    println!("Test passed!");
}

```
