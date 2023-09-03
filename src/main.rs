use nalgebra:: Matrix4;
use std::io;
fn read_value(prompt: String) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            _ => println!("Invalid input. Please enter a valid float value."),
        };
    }
}

fn main() {
    let degrees: bool = true; // Set this to true if you want to input angles in degrees

    let number_of_frames: usize = loop {
        println!("Enter Number of frames:");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(frames) if frames >= 1 => break frames,
            _ => println!("Number of Frames must be greater than or equal to 1"),
        }
    };

    let mut matrices: Vec<Matrix4<f64>> = Vec::new();

    for i in 0..number_of_frames {
        let theta: f64 = read_value(format!("Enter theta_{}:", i + 1));
        let alpha: f64 = read_value(format!("Enter alpha_{}:", i + 1));
        let a: f64 = read_value(format!("Enter a_{}:", i + 1));
        let d: f64 = read_value(format!("Enter d_{}:", i + 1));

        let theta_val: f64 = if degrees { theta.to_radians() } else { theta };
        let alpha_val: f64 = if degrees { alpha.to_radians() } else { alpha };

        let mut matrix = Matrix4::<f64>::identity();
        matrix[(0, 0)] = theta_val.cos();
        matrix[(0, 1)] = -(theta_val.sin());
        matrix[(0, 3)] = a;
        matrix[(1, 0)] = theta_val.sin() * alpha_val.cos();
        matrix[(1, 1)] = theta_val.cos() * alpha_val.cos();
        matrix[(1, 2)] = -(alpha_val.sin());
        matrix[(1, 3)] = -(d * alpha_val.sin());
        matrix[(2, 0)] = theta_val.sin() * alpha_val.sin();
        matrix[(2, 1)] = theta_val.cos() * alpha_val.sin();
        matrix[(2, 2)] = alpha_val.cos();
        matrix[(2, 3)] = d * alpha_val.cos();

        matrices.push(matrix);
    }
    let mut result_matrix = Matrix4::<f64>::identity();
    for matrix in matrices {
        result_matrix = result_matrix * matrix;

        println!("{:.5}", matrix)
    }
    println!("Result:");
    println!("{:.5}", result_matrix);
    let theta = result_matrix[(1, 0)].atan2(result_matrix[(0, 0)]);
    println!("Theta (θ): {}", theta.to_degrees());

}
