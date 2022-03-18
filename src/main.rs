type Scalar = num::rational::Ratio<i128>;
type Chromaticity = rgb_derivation::Chromaticity<Scalar>;

fn chromaticity(x: (i128, i128), y: (i128, i128)) -> Chromaticity {
    Chromaticity::new(Scalar::new(x.0, x.1), Scalar::new(y.0, y.1)).unwrap()
}

fn print_vector(header: &str, vector: &[Scalar; 3]) {
    print!("{}: [", header);
    for (idx, value) in vector.iter().enumerate() {
        print!("{} {} / {}",
               if idx == 0 { "" } else { "," },
               value.numer(), value.denom());
    }
    println!(" ]");
}

fn print_matrix(header: &str, matrix: &[[Scalar; 3]; 3]) {
    fn make_array<T>(f: impl Fn(usize) -> T) -> [T; 3] { [f(0), f(1), f(2)] }

    let formatted = make_array(|row| make_array(|col| {
        let value = &matrix[row][col];
        (format!("{}", value.numer()), format!("{}", value.denom()))
    }));
    let lengths = make_array(|col| (
        formatted.iter().map(|row| row[col].0.len()).max().unwrap(),
        formatted.iter().map(|row| row[col].1.len()).max().unwrap(),
    ));

    println!("{}:", header);
    for row in formatted.iter() {
        print!("  [");
        for (idx, value) in row.iter().enumerate() {
            print!("{comma}{numer:>numer_len$} / {denom:>denom_len$}",
                   comma = if idx == 0 { "" } else { ", " },
                   numer = value.0, numer_len = lengths[idx].0,
                   denom = value.1, denom_len = lengths[idx].1);
        }
        println!("],");
    }
}

fn main() {
    // All definitions taken from CSS (different standards differ slightly)
    // https://www.w3.org/TR/css-color-4/#predefined

    let d65_white_xy = chromaticity((3127, 10000), (3290, 10000));
    let d65_white_xyz = d65_white_xy.to_xyz();
    print_vector("D65 white point", &d65_white_xyz);

    // srgb
    {
        let primaries_xy = [
            chromaticity((64, 100), (33, 100)),
            chromaticity((30, 100), (60, 100)),
            chromaticity((15, 100), (6, 100)),
        ];

        let matrix = rgb_derivation::matrix::calculate(
            &d65_white_xyz, &primaries_xy).unwrap();
        let inverse = rgb_derivation::matrix::inversed_copy(&matrix).unwrap();
        let primaries_xyz = rgb_derivation::matrix::transposed_copy(&matrix);

        print_matrix("sRGB primaries", &primaries_xyz);
        print_matrix("sRGB→XYZ", &matrix);
        print_matrix("XYZ→sRGB", &inverse);
    }
    // display-p3
    {
        let primaries_xy = [
            chromaticity((68, 100), (32, 100)),
            chromaticity((265, 1000), (69, 100)),
            chromaticity((15, 100), (6, 100)),
        ];

        let matrix = rgb_derivation::matrix::calculate(
            &d65_white_xyz, &primaries_xy).unwrap();
        let inverse = rgb_derivation::matrix::inversed_copy(&matrix).unwrap();
        let primaries_xyz = rgb_derivation::matrix::transposed_copy(&matrix);

        print_matrix("display-p3 primaries", &primaries_xyz);
        print_matrix("display-p3→XYZ", &matrix);
        print_matrix("XYZ→display-p3", &inverse);
    }
}
