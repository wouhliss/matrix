use matrix::*;

fn main() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(v);
    println!("{}", u);

    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(v);
    println!("{}", u);

    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    println!("{}", u);

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(v);
    println!("{}", u);

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(v);
    println!("{}", u);

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("{}", u);

    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    println!(
        "{}",
        Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5])
    );

    println!("{}", Vector::linear_combination(&[v1, v2], &[10., -2.]));

    println!("{}", lerp(0., 1., 0.));

    println!("{}", lerp(0., 1., 1.));

    println!("{}", lerp(0., 1., 0.5));

    println!("{}", lerp(21., 42., 0.3));

    println!(
        "{}",
        Vector::lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3)
    );

    println!(
        "{}",
        Matrix::lerp(
            Matrix::from([[2., 1.], [3., 4.]]),
            Matrix::from([[20., 10.], [30., 40.]]),
            0.5
        )
    );
}
