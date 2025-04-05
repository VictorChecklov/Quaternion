# Tutorial

__In this file I'm going to show you why we should use quaternions to represent rotation and some basics about it__

## Why should we use quaternions?

In the field of __Computer Graphics__, rotation is always an important component due to its widespread use

There are several methods to represent 3-D rotation. Such as `Euler Angle`, `Matrix`, `Quaternions` ...

___

### Homogeneous Coordinates

So, you might be wondering: why are we starting with this obscure concept instead of Euler Angles?
- __First of all__, Euler angle could be represented by matrices, which we will mention in the following text.
- __Secondly__, I believe that this article should not be misleading and I also don't want anyone to keep using incomplete concepts because of this article
, so I should be more comprehensive in my explanation.

We need to know that transformations are not limited to just rotations. Other transformations like translation and scaling are also important. 
And in standard 3D space, translation cannot be represented in matrix form, because it is __NOT__ a linear transform.

$$\begin{bmatrix} x' \\ 
                  y'\\
                  z' \end{bmatrix}=\begin{bmatrix}a&b&c \\ 
                                                  d&e&f \\
                                                  h&i&j\end{bmatrix}\begin{bmatrix}x \\ 
                                                                                   y \\
                                                                                   z \end{bmatrix}+\begin{bmatrix}t_x \\
                                                                                                                   t_y \\ 
                                                                                                                   t_z \end{bmatrix}$$

So we need a unified way to represent all transformations. We use $(\mathbf{x}, \mathbf{y}, \mathbf{z}, 1)^\mathbf{T}$ for a 3-D point and $(\mathbf{x}, \mathbf{y}, \mathbf{z}, 0)$ for a 3-D vector.
The previous formula could be replaced into:

$$\begin{bmatrix}x' \\
                 y' \\ 
                 z' \\
                 1 \end{bmatrix}=\begin{bmatrix}a&b&c&t_x \\
                                                d&e&f&t_y \\ 
                                                h&i&j&t_z \\
                                                0&0&0&1\end{bmatrix}\begin{bmatrix}x \\
                                                                                   y \\
                                                                                   z \\
                                                                                   1\end{bmatrix}$$

Since I've clarified this concept, so the other formulas in the following text will __NOT__ use the Homogeneous Coordinates.
After all, the main mission of this article is to explain the implementation of rotation, not translation.
___

###  Euler Angle 

**Euler angles** are quite simple and intuitive, because what we need are just 3 values to represent the angles of rotation around xyz axes.
Euler proved that this method can represent every possible orientation in space.
To use Euler angles, a clear rotation order must be specified. 
Because even with the same angle, different order can lead to different orientations.

However, after rotating 90 degrees around the second axis, the other two rotation axes align, causing a loss of one degree of freedom.
This phenomenon is known as __Gimbal Lock__.
Typically, we can use matrices  to indicate that this method cannot avoid Gimbal Lock. the proof is as follows:


__First of all, the representation of an object's rotation around the X, Y and Z axes:__

$$ R_x(\alpha) = \begin{bmatrix} 1 & 0 & 0 \\
                                 0 & \cos(\alpha) & -\sin(\alpha) \\
                                 0 & \sin(\alpha) & \cos(\alpha) \end{bmatrix} $$

$$ R_y(\beta) = \begin{bmatrix} \cos(\beta) & 0 & \sin(\beta) \\
                                0 & 1 & 0 \\
                                -\sin(\beta) & 0 & \cos(\beta) \end{bmatrix}$$

$$ R_z(\gamma) = \begin{bmatrix} \cos(\gamma) & -\sin(\gamma) & 0 \\
                                 \sin(\gamma) & \cos(\gamma) & 0 \\
                                 0 & 0 & 1 \end{bmatrix}$$

__Also, we need some knowledge of trigonometric functions:__

$$ \begin{aligned}&\sin(\alpha - \beta) = \sin(\alpha)\cos(\beta) - \cos(\alpha)\sin(\beta) \\
                  &\cos(\alpha - \beta) = \cos(\alpha)\cos(\beta) + \sin(\alpha)\sin(\beta)\end{aligned} $$

__And then we get__

$$ \begin{equation}\begin{aligned} 
& R_z(\gamma)R_y(\frac{\pi}{2})R_x(\alpha) \\
& = \begin{bmatrix} \cos(\gamma) & -\sin(\gamma) & 0 \\
                    \sin(\gamma) & \cos(\gamma) & 0 \\ 
                    0 & 0 & 1 \end{bmatrix}\begin{bmatrix} 0 & 0 & 1 \\
                                                           0 & 1 & 0 \\
                                                           -1 & 0 & 0 \end{bmatrix}\begin{bmatrix} 1 & 0 & 0 \\
                                                                                                   0 & \cos(\alpha) & -\sin(\alpha) \\ 
                                                                                                   0 & \sin(\alpha) & \cos(\alpha) \end{bmatrix}\\
& = \begin{bmatrix} 0 & \cos(\gamma)\sin(\alpha)-\cos(\alpha)\sin(\gamma) & \sin(\alpha)\sin(\gamma)+\cos(\alpha)\cos(\gamma)\\
    0 & \sin(\alpha)\sin(\gamma)+\cos(\alpha)\cos(\gamma) & \cos(\alpha)\sin(\gamma)-\cos(\gamma)\sin(\alpha)\\
    -1 & 0 & 0 \end{bmatrix} \\
& = \begin{bmatrix} 0 & \sin(\alpha - \gamma) & \cos(\alpha - \gamma) \\ 
    0 & \cos (\alpha - \gamma) & -\sin(\alpha - \gamma) \\
    -1 & 0 & 0 \end{bmatrix} \\
& = R_y(\frac{\pi}{2})R_x(\alpha - \gamma)
\end{aligned}\end{equation}$$

We observe that the rotation matrix $R_z$ is no longer present in the final expression, which demonstrates the loss of one rotational degree of freedom.
__So if you really want to use this method, you should consider which axis is most impossible to be rotated by 90 degrees,and then place it in the second one.__
Why would this happen? Euler Angle could represent any attitude, but this doesn't mean that each attitude refers to unique euler angle.Which means (α, π/2, γ) and (-γ, π/2, -α) could represent the same attitude. Noticed? During a rotation, after you set the second angle as 90 degrees, every operation to the first and the third degrees would lead to the same result. this is the simplist explaination I could give :(

You might wonder, isn't this using a dynamic coordinate system based on the objects' local coordinate. 
There should be a static coordinate system based on the world coordinate.\
That's right, but if we adopt a static coordinate system, rotations will become complex and unpredictable, thus losing the advantage of Euler angle. 
## Matrix ##

Here we have __Rodrigues' Rotation Formula__,With $\vec{r}$ for the rotation axis and $\theta$ for the rotation angle

$$K = \begin{bmatrix}0&-r_z&r_y \\
                     r_z&0&-r_x \\
                     -r_y&r_x&0\end{bmatrix} $$

$$R(\theta,\vec{r}) = I + \sin(\theta) K + (1 - \cos\theta)K^2$$

Then we can get:

$$ R(\theta,\vec{r}) = 
\begin{bmatrix}\cos\theta+r_x^2(1-\cos\theta)&r_xr_y(1-\cos\theta)-r_z\sin\theta&r_xr_z(1-\cos\theta)+r_y\sin\theta \\
               r_yr_x(1-\cos\theta)+r_z\sin\theta&\cos\theta+r_y^2(1-\cos\theta)&r_yr_z(1-\cos\theta)-r_x\sin\theta \\
               r_zr_x(1-\cos\theta)-r_y\sin\theta&r_zr_y(1-\cos\theta)+r_x\sin\theta&\cos\theta+r_z^2(1-\cos\theta)\end{bmatrix} $$

As you can see, it's quite complex, but it can avoid Gimbal Lock. And there are other shortcomings about it.


- The rotation matrix requires the storage of 9 numbers, with quaternions need only 4 numbers.
- During the operation, due to the loss of precision and the accumulation of errors, the rotation may lose its orthogonality,
leading to shear deformation. It's necessary to orthogonalize the correction from time to time.
- The rotation matrix is difficult to interpolate smoothly.
___
### Quaternions

Quaternions, unlike Euler angles and rotation matrices, provide a more compact and numerically stable solution for representing rotations, 
especially in scenarios like animation and 3D simulations where smooth interpolation and avoiding gimbal lock are crucial.

A quaternion is generally represented as: $q = w + xi + yj + zk\$\
when the real part is zero, this pure quaternion can represent a vector

Here are some basic knowledge about quaternions:
- __Conjugate: $q^* = w - xi - yj - zk$__
- __Dot: $p \cdot q= w_1w_2 + x_1x_2 + y_1y_2 + z_1z_2$__
- __Cross: $p\times q=(w_1w_2-x_1x_2-y_1y_2-z_1z_2)+(w_1x_2+x_1w_2+y_1z_2-y_2z_1)i+(w_1y_2-x_1z_2+y_1w_2+z_1x_2)j+(w_1z_2=z_1x_2-y_1x_2+z_1w_2)k$__
- __Inverse: $q^{-1} = \frac{q^*}{q \cdot q}$__

__We could use the following formula based on Rodrigues' Rotation Formula to represent rotation:__

$$ Q = \cos\left(\frac{\theta}{2}\right) + \sin\left(\frac{\theta}{2}\right)(u_x i + u_y j + u_z k) $$

$$ X' = QXQ^{-1} $$

__With $(u_x, u_y, u_z)$ for the rotation axis and $\theta$ for the rotation angle__\
__A quaternion can also be converted into a matrix:__

$$ w+xi+yj+zk=\begin{bmatrix}w^2+x^2-y^2-z^2&2xy-2wz&2xz+2wy\\
                             2xy+2wz&w^2-x^2+y^2-z^2&2yz-2wx\\
                             2xz-2wy&2yz+2wx&w^2-x^2-y^2+z^2\end{bmatrix} $$

__For linear interpolation of quaternions, we have some great and simple methods__

1. Linear Interpolation (Lerp):

$$ Lerp(p, q, t) = (1-t)p+tq $$

   - Advantages: Simple and fast.
   - Disadvantages: 
     - Didn't normalize the output, may influence the stability. 
     - The rotation speed changes significantly, making it unsuitable for smooth rotation scenes.
   - When to use: Suitable for very small angles or situations where extremely high efficiency is required.
3. Normalized Linear Interpolation (Nlerp):

$$ Nlerp(p,q,t) = \frac{(1-t)p+tq}{||(1-t)p+tq||}$$

  - Advantage: Fast to compute.
  - Disadvantage: It doesn't interpolate along a constant speed on the unit quaternion sphere
  - When to use: Best for real-time applications where computational efficiency is more important than perfect rotational smoothness.
4. Spherical linear interpolation (Slerp):

$$ Slerp(p, q, t) = \frac{\sin[(1 - t)\theta]p + \sin(t\theta)q}{\sin\theta}$$

  - Advantages: Slerp interpolates along the shortest path on the unit quaternion sphere, 
        maintaining constant rotational speed, making it ideal for smooth transitions.
- Disadvantages: 
  - When the dot product of p and q is a negative number, Slerp will go the __LONG__ path,
        and we must negate one of the quaternions, which won't change the effect of the rotation.
  - When $\theta\rightarrow0$, $\sin\theta\rightarrow0$, and $\sin\theta$ is in the denominator,
        so we need to use linear interpolation when $\theta$ is very small.
- When to use:  Preferred for applications where precision and smoothness are critical, 
        such as in animations, game development, or 3D simulations.

6. Spherical and Quadrangle Spline Interpolation (Squad):

$$ Squad(p_0,p_1,p_2,p_3,t) = Slerp(Slerp(p_0,p_1,t),Slerp(p_2,p_3,t),2t(1-t)) $$

   - Advantages:
     - It uses four control points (quaternions) to define the shape of the interpolation curve, 
       enabling smoother transitions than simple linear or spherical interpolation.
     - It is designed for smooth transitions between multiple quaternions. 
       It ensures smooth second-order continuity, making it suitable for continuous motion in complex animations.
   - Disadvantages: 
     Complex, clearly.
   - When to use: Ideal for spline-based rotations where smooth transitions between multiple rotations are needed, 
     typically in animation or robotics.

### Shortcomings

Quaternions cannot represent translation or scaling, which means that homogeneous coordinates are still needed for these fields
