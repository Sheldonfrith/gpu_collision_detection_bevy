pub fn solve_downwards_step_function(
    x: f32,
    upper_boundaries: Vec<f32>,
    y_values: Vec<f32>,
    final_y: f32,
) -> f32 {
    let mut y = final_y;
    for i in 0..upper_boundaries.len() {
        if x < upper_boundaries[i] {
            y = y_values[i];
            break;
        }
    }
    y
}
#[derive(Clone, Debug)]
pub struct StepFunction {
    upper_boundaries: Vec<f32>,
    y_values: Vec<f32>,
    final_y: f32,
}

/**
*  @example:
       // Creates a step function that starts at y=1.0, drops to 0.5 at x=2.0,
       // drops to 0.2 at x=4.0, and finally becomes 0.0 for x>=6.0
       StepFunction::new(
           vec![2.0, 4.0, 6.0],
           vec![1.0, 0.5, 0.2],
           0.0
       )

*/
impl StepFunction {
    pub fn new(upper_boundaries: Vec<f32>, y_values: Vec<f32>, final_y: f32) -> Self {
        assert_eq!(
            upper_boundaries.len(),
            y_values.len(),
            "Boundaries and y-values must have same length"
        );
        Self {
            upper_boundaries,
            y_values,
            final_y,
        }
    }
    // multiply all boundaries by a factor
    pub fn multiply_boundaries(&mut self, factor: f32) {
        for i in 0..self.upper_boundaries.len() {
            self.upper_boundaries[i] *= factor;
        }
    }

    pub fn evaluate(&self, x: f32) -> f32 {
        solve_downwards_step_function(
            x,
            self.upper_boundaries.clone(),
            self.y_values.clone(),
            self.final_y,
        )
    }
}
