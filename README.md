# Quaternion

This is a simple implementation of quaternions.

__This repository primarily serves as a tutorial on quaternions, not just an implementation.__

__This is my first venture into Computer Graphics, and I believe I'll learn something really cool!__

You can read `Tutorial.md` for more detailed information about Quaternions.

## Introduction

### Structs:
- `Quaternion`: Generic quaternion that supports both `f32` and `f64`

### Functions:
- **Reloaded Operations**: Add, Sub, Mul for Quaternion-Quaternion and Quaternion-Scalar.
- `dot()`: `q1.dot(q2)` -> returns `f32` or `f64`.
- `dot_v()`: `q1.dot(q2)` -> returns `f32` or `f64` (both q1 and q2 should be vectors, where the real part is zero).
- `norm()`: `q.norm()` -> returns the norm of quaternion `q`.
- `normalize()`: `q.normalize()` -> normalizes the quaternion and returns a new quaternion.
- `conjugate()`: `q.conjugate()` -> returns the conjugate of the quaternion.
- `rotate()`: `q.rotate(axis, angle)` -> returns a quaternion that represents rotation by `angle` around the `axis`.
- `mat()`: `q.mat()` -> converts the quaternion to a 3x3 rotation matrix.
- `nlerp()`: `q.nlerp(target, t)` -> returns the normalized linear interpolation between `q` and `target` by a factor of `t`.
- `slerp()`: `q.slerp(target, t)` -> returns the spherical linear interpolation (slerp) between `q` and `target`.
- `squad()`: `q.squad(q1, q2, q3, t)` -> returns the spherical quadratic interpolation between four quaternions.

For the generic quaternion:
- `mat_homogeneous()`: Converts the quaternion to a 4x4 matrix (homogeneous coordinates).
- `display()`: Returns the string representation of the quaternion.
- Supports non-borrowed operations with `+`, `-`, and `*` for quaternions.
- Optimized error handling.

## Example

```rust
fn main() {
    let p = Quaternion::new(0, 0, 3, 2);
    let q = Quaternion::new(1, 2, 3, 4);
    let out = &p * &q;
    
    // Check the result of quaternion multiplication
    assert_eq!(out.w, -17);
    assert_eq!(out.x, 6);
    assert_eq!(out.y, 7);
    assert_eq!(out.z, -4);
    
    println!("Test passed! Quaternion multiplication works as expected.");
}

