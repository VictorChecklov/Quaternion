# Quaternion

Simple implementation of quaternions

## Introduction of Quaternion

### Background

- ` Euler angles ` could have glimbal lock issues. When you rotate an object in a specific order and angle, you may lose one degree of freedom;
- As for ` Matrix `, although it can avoid universal joint locks, if your rotation matrix is not an orthogonal matrix, it will cause awful errors, and the rotation matrix needs to store 9 quantities

### Advantage

` Quaternions ` can represent rotation using only four numbers with

$$ q = w + xi + yj + zk\ $$

and when the real part is zero, this pure quaternion can represent a vector

ant we could use the following formula to represent rotation

$$ \mathbf{X}' = \mathbf{Q} \mathbf{X} \mathbf{Q}^{-1} $$

$$ \mathbf{Q} = \cos\left(\frac{\theta}{2}\right) + \sin\left(\frac{\theta}{2}\right)(u_x i + u_y j + u_z k) $$

### Shortage 

Quaternions cannot represent translation or scaling, which means that homogeneous coordinates are still needed for these fields

## Introduction to This Little Module

- Created a struct named Quaternion
- Overloaded addition, subtraction, and quaternion multiplication including scalar multiplication
- Implemented four functions: conjugate, modulus, inverse, and rotation

## Useage

after you imported this module

```rs
fn main(){
    let q1 = Quaternion::new(0.0, 1.0, 0.0, 0.0);
    let shaft = Quaternion::new(0.0, 0.0, 0.0, 1.0);
    let q2  = q1.rotate(&shaft, std::f64::consts::FRAC_PI_2, 2.0);
    println!("Quaternion: {} + {}i + {}j + {}k", q2.w(), q2.x(), q2.y(), q2.z());

    let q3 = Quaternion::new(1, 2, 3, 4);
    let q4 = Quaternion::new(2, 3, 4, 5);
    let q5 = &q3 * &q4;
    println!("Quaternion: {} + {}i + {}j + {}k", q5.w(), q5.x(), q5.y(), q5.z())
}

```


