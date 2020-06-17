# Numerical Analysis

## Definitions
1. A solution is correct within $p$ decimal places if the error is less than $0.5 \times 10^{-p}$


## General Concepts and Taylor
### Horners method
A way to evaluate polynomials using as few operations as possible. 
Take the polynomial $P(x) = 2x^4 + 3x^3 -3x^2 + 5x -1$ as an example.
Using Horners method $P$ is rewritten as 
$$ 
P(x) = -1+x(5-3x+3x^2+2x^3)\\ 
= -1+x(5 + x(-3+3x+2x^2)) \\
= -1 + x(5 + x(-3 + x(3 + 2x)))
$$
This method allows us to evaluate $P$ using only 4 additions and 4 multiplications. This form of writing polynomials is called __nested form__.

### Intermediate Value Theorem
Let $f$ be a continuous function on the interval [a,b]. Then $f$ realizes every value between $f(a)$ and $f(b)$.
### Mean Value Theorem
Let $f$ be a continuous differentiable function on the interval [a,b]. Then there exists a number $c$ between $a$ and $b$ such that $f'(c) = (f(b)-f(a))/(b-a)$.
### Taylors theorem with remainder
Let $x$ and $x_0$ be real numbers, and let $f$ be $k+1$ times continuously differentiable on the interval between $x$ and $x_0$. Then there exists a number $c$ between $x$ and $x_0$ such that
$$
f(x) = f(x_0) + f'(x_0)(x-x_0)+\frac{f''(x_0)}{2!}(x-x_0)^2+\frac{f'''(x_0)}{3!}(x-x_0)^3+...\\
+ \frac{f^{(k)}(x_0)}{k!}(x-x_0)^k + \frac{f^{(k+1)}(c)}{(k+1)!}(x-x_0)^{k+1}
$$
The final term is called the __Taylor remainder__.


## Root finding

### Bisection method
if $f(a) < 0$ and $f(b) > 0$ there must be a $c$ such that $a < c < b$ and $f(c) = 0$, in other words there exists a root in this interval. Another way to check whether an interval has a root is to make sure that $f(a)f(b) < 0$.
```
// Given initial interval [a,b] such that f(a)f(b) < 0
// where TOL is the shortest interval allowed.
while (b - a)/2 > TOL {
    c = (a+b)/2
    if f(c) == 0 END
    if f(a)f(c) < 0 {
        b = c
    } else { 
        a = c
    }
}
return (a+b)/2
```
Solution error = $|x_c - r| < \frac{b-a}{2^{n+1}}$
### Fixed Point Iteration

## Rate of convergence

## Newtons' method

## Gaussian + LU

## Iterative Schemes

## Swamping and Condition number

## Linear Algebra review

## Eighenvalues and Power Method

## Gram Schmidt and QR

## Least Squares

## Interpolation 

### Lagrange/Chebyshev

### Cubic splines

### VanderMonde

## Cubic Splines and Bezier curves

## DFT

## Fourier Transforms

## Numerical Integration

## Gaussian Quadratures

## Ordinary Differential Equations

## Higher Order ODE's

---
---
## EXAM QUESTIONS
+ If the condition number for the matrix A of the system Ax = b is large then we can multiply the whole equation with a small number and create a new system Âx = b̂. This new system has the same solution x but better condition number for the matrix A.
+ The power method converges to the largest eigenvalue.
+ If the Newton iteration does converge to the root then it will converge to it with quadratic order of convergence.
+ An interpolating cubic spline agrees in 1st, 2nd and 3rd derivatives at all of the interior nodes.
+ The trapezoidal rule is an open Newton-Cotes formula.
+ −1 is a primitive 2nd root of unity and a non-primitive 4th root of unity.
+ The explicit Euler method has order of convergence 1.

+ The secant method is a two-point, encompassing method (i.e it is required that the two starting values x 0 and x 1 must contain between them the root of the function f (x)).
+ The Vantermonde interpolating polynomial can be constructed from either equidistant or non-equidistant nodes.
+ Is the Lagrange interpolating polynomial for 5 points the same as the interpolating polynomial produced by the DFT for the same 5 points?
+ Every Bézier curve is infinitely differentiable. 
+ Is the midpoint rule exact for polynomials of degree 1?
+ Is it possible that the degree of precision (ADA) of a Newton-Cotes formula would ever equal that of a Gaussian quadrature over the same number of points?
+ Suppose the following quadrature formula, $$\int_{-1}^1 f(x) dx = 2f(0)$$ This is a one point, closed Newton-Cotes formula

### Concepts
+ Fixed Point Iteration
+ Gramm-Schmidt ortogonalization
+ derive a Gaussian quadrature and Legendre polynomials
+ Bézier curve
+ divide and conquer
+ Legendre nodes
+ Order of convergence
+ Gauss-Seidel scheme
+ least squares
    + QR method
+ Newton-Cotes
+ ODE
+ Newton's iteration method
+ Jacobian matrix