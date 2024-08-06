# Quaternion

Simple implementation of quaternions 

__This is my first venture into Computer Graphics, and I believe I'll learn something really cool__

## Introduction to Quaternion

### Background

- ` Euler angles ` could cause the gimbal lock. When you rotate an object in a specific order and angle, you may lose one degree of freedom;
- As for ` Matrics `, although it can avoid gimbal lock, if your rotation matrix is not an orthogonal matrix, it will cause awful errors, and the rotation matrix needs to store 9 quantities

### Advantages

` Quaternions ` can represent rotation using only 4 numbers:

$$ q = w + xi + yj + zk\ $$

and when the real part is zero, this pure quaternion can represent a vector

We could use the following formula to represent rotation

$$ \mathbf{X}' = \mathbf{Q} \mathbf{X} \mathbf{Q}^{-1} $$

Where:

$$ \mathbf{Q} = \cos\left(\frac{\theta}{2}\right) + \sin\left(\frac{\theta}{2}\right)(u_x i + u_y j + u_z k) $$

With:

$$ \mathbf{RotationAxis} = (u_x, u_y, u_z) $$

$$ \mathbf{RotationAngle} = {\theta} $$

### Shortcomings

Quaternions cannot represent translation or scaling, which means that homogeneous coordinates are still needed for these fields

## Introduction to This Little Module

- Created a struct named Quaternion
- Overloaded addition, subtraction, and quaternion multiplication including scalar multiplication
- Implemented four functions: conjugate, modulus, inverse, and rotation

## Usage

after you imported this module ( ` Ctrl + C `  and ` Ctrl + V ` solves everything)

```rs
fn main(){
    let q1 = Quaternion::new(0.0, 1.0, 0.0, 0.0);
    let shaft = Quaternion::new(0.0, 0.0, 0.0, 1.0);
    let q2  = q1.rotate(&shaft, std::f64::consts::FRAC_PI_2, 2.0);
    println!("Quaternion-Rotation: {} + {}i + {}j + {}k", q2.w(), q2.x(), q2.y(), q2.z());

    let q3 = Quaternion::new(1, 2, 3, 4);
    let q4 = Quaternion::new(2, 3, 4, 5);
    let q5 = &q3 * &q4;
    println!("Quaternion-Multiplication: {} + {}i + {}j + {}k", q5.w(), q5.x(), q5.y(), q5.z());

    let q6 = q1.conjugate();
    println!("Quaternion-conjugate: {} + {}i + {}j + {}k", q6.w(), q6.x(), q6.y(), q6.z());

    let norm = q3.norm();
    println!("Module: {}", norm);

    let q7 = q1.inverse();
    println!("Quaternion-Inverse: {} + {}i + {}j + {}k", q7.w(), q7.x(), q7.y(), q7.z());
}

```


