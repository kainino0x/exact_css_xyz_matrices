type Scalar = num::rational::Ratio<i128>;
type Chromaticity = rgb_derivation::Chromaticity<Scalar>;

fn chromaticity(x: (i128, i128), y: (i128, i128)) -> Chromaticity {
    Chromaticity::new(Scalar::new(x.0, x.1), Scalar::new(y.0, y.1)).unwrap()
}

fn print_vector(header: &str, vector: &[Scalar; 3]) {
    print!("{}: [", header);
    for (idx, value) in vector.iter().enumerate() {
        print!(
            "{} {} / {}",
            if idx == 0 { "" } else { "," },
            value.numer(),
            value.denom()
        );
    }
    println!(" ]");
}

fn print_matrix(header: &str, matrix: &[[Scalar; 3]; 3]) {
    fn make_array<T>(f: impl Fn(usize) -> T) -> [T; 3] {
        [f(0), f(1), f(2)]
    }

    let formatted = make_array(|row| {
        make_array(|col| {
            let value = &matrix[row][col];
            (format!("{}", value.numer()), format!("{}", value.denom()))
        })
    });
    let lengths = make_array(|col| {
        (
            formatted.iter().map(|row| row[col].0.len()).max().unwrap(),
            formatted.iter().map(|row| row[col].1.len()).max().unwrap(),
        )
    });

    println!("{}:", header);
    for row in formatted.iter() {
        print!("    [ ");
        for (idx, value) in row.iter().enumerate() {
            print!(
                "{comma}{numer:>numer_len$} / {denom:>denom_len$}",
                comma = if idx == 0 { "" } else { ", " },
                numer = value.0,
                numer_len = lengths[idx].0,
                denom = value.1,
                denom_len = lengths[idx].1
            );
        }
        println!(" ],");
    }
}

fn print_color_space_matrices(
    name: &str,
    white_xyz: &[Scalar; 3],
    primaries_xy: &[Chromaticity; 3],
) {
    let matrix = rgb_derivation::matrix::calculate(white_xyz, primaries_xy).unwrap();
    let inverse = rgb_derivation::matrix::inversed_copy(&matrix).unwrap();
    let primaries_xyz = rgb_derivation::matrix::transposed_copy(&matrix);

    println!("");
    println!("{}", name);
    print_matrix("  Primaries", &primaries_xyz);
    print_matrix("  To XYZ", &matrix);
    print_matrix("  From XYZ", &inverse);
}

fn main() {
    // All definitions taken from CSS (different standards differ slightly)
    // https://www.w3.org/TR/css-color-4/#predefined

    let d50_white_xy = chromaticity((3457, 10000), (3585, 10000));
    let d50_white_xyz = d50_white_xy.to_xyz();
    print_vector("D50 white point", &d50_white_xyz);

    let d65_white_xy = chromaticity((3127, 10000), (3290, 10000));
    let d65_white_xyz = d65_white_xy.to_xyz();
    print_vector("D65 white point", &d65_white_xyz);

    print_color_space_matrices(
        "srgb",
        &d65_white_xyz,
        &[
            chromaticity((640, 1000), (330, 1000)),
            chromaticity((300, 1000), (600, 1000)),
            chromaticity((150, 1000), (060, 1000)),
        ],
    );

    print_color_space_matrices(
        "display-p3",
        &d65_white_xyz,
        &[
            chromaticity((680, 1000), (320, 1000)),
            chromaticity((265, 1000), (690, 1000)),
            chromaticity((150, 1000), (060, 1000)),
        ],
    );

    print_color_space_matrices(
        "a98-rgb",
        &d65_white_xyz,
        &[
            chromaticity((6400, 10000), (3300, 10000)),
            chromaticity((2100, 10000), (7100, 10000)),
            chromaticity((1500, 10000), (0600, 10000)),
        ],
    );

    print_color_space_matrices(
        "prophoto-rgb",
        &d50_white_xyz,
        &[
            chromaticity((734_699, 1_000_000), (265_301, 1_000_000)),
            chromaticity((159_597, 1_000_000), (840_403, 1_000_000)),
            chromaticity((036_598, 1_000_000), (000_105, 1_000_000)),
        ],
    );

    print_color_space_matrices(
        "rec2020",
        &d65_white_xyz,
        &[
            chromaticity((708, 1000), (292, 1000)),
            chromaticity((170, 1000), (797, 1000)),
            chromaticity((131, 1000), (046, 1000)),
        ],
    );
}
