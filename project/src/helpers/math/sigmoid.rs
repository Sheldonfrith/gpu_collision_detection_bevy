fn get_coeficient_given_percent_away(percent_away: f32) -> f32 {
    const a: f32 = 73.67549;
    const b: f32 = -0.9095928;
    percent_away.powf(1. / b) / a.powf(1. / b)
}

#[derive(Clone, Debug)]

pub struct SuperEasySigmoidReverse {
    sigmoid: BoundedSigmoidCurve,
    very_strong_until: f32,
    almost_gone_after: f32,
}
#[derive(Clone, Debug)]

pub struct SuperEasySigmoidRegular {
    sigmoid: BoundedSigmoidCurve,
    very_strong_after: f32,
    almost_gone_below: f32,
}

impl SuperEasySigmoidReverse {
    pub fn new(very_strong_until: f32, almost_gone_after: f32) -> Self {
        Self {
            sigmoid: BoundedSigmoidCurve::new(1.0, very_strong_until, almost_gone_after, 5., true),
            very_strong_until,
            almost_gone_after,
        }
    }
    // getters
    pub fn very_strong_until(&self) -> f32 {
        self.very_strong_until
    }
    pub fn almost_gone_after(&self) -> f32 {
        self.almost_gone_after
    }

    fn create_sigmoid(&self) -> BoundedSigmoidCurve {
        BoundedSigmoidCurve::new(
            1.0,
            self.very_strong_until,
            self.almost_gone_after,
            5.,
            true,
        )
    }

    pub fn evaluate(&self, x: f32) -> f32 {
        self.sigmoid.evaluate(x)
    }

    pub fn multiply_strong_until(&mut self, multiplier: f32) {
        self.very_strong_until *= multiplier;
        self.sigmoid = self.create_sigmoid();
    }
    pub fn multiply_almost_gone_after(&mut self, multiplier: f32) {
        self.almost_gone_after *= multiplier;
        self.sigmoid = self.create_sigmoid();
    }
    pub fn add_to_strong_until(&mut self, addition: f32) {
        self.very_strong_until += addition;
        self.sigmoid = self.create_sigmoid();
    }
    pub fn add_to_almost_gone_after(&mut self, addition: f32) {
        self.almost_gone_after += addition;
        self.sigmoid = self.create_sigmoid();
    }
}

impl SuperEasySigmoidRegular {
    pub fn new(almost_gone_below: f32, very_strong_after: f32) -> Self {
        Self {
            sigmoid: BoundedSigmoidCurve::new(1.0, almost_gone_below, very_strong_after, 5., false),
            very_strong_after,
            almost_gone_below,
        }
    }
    // getters
    pub fn almost_gone_below(&self) -> f32 {
        self.almost_gone_below
    }
    pub fn very_strong_after(&self) -> f32 {
        self.very_strong_after
    }

    pub fn evaluate(&self, x: f32) -> f32 {
        self.sigmoid.evaluate(x)
    }
    pub fn create_sigmoid(&self) -> BoundedSigmoidCurve {
        BoundedSigmoidCurve::new(
            1.0,
            self.almost_gone_below,
            self.very_strong_after,
            5.,
            false,
        )
    }
    pub fn multiply_almost_gone_below(&mut self, multiplier: f32) {
        self.almost_gone_below *= multiplier;
        self.sigmoid = self.create_sigmoid();
    }
    pub fn multiply_very_strong_after(&mut self, multiplier: f32) {
        self.very_strong_after *= multiplier;
        self.sigmoid = self.create_sigmoid();
    }
    pub fn add_to_almost_gone_below(&mut self, addition: f32) {
        self.almost_gone_below += addition;
        self.sigmoid = self.create_sigmoid();
    }
    pub fn add_to_very_strong_after(&mut self, addition: f32) {
        self.very_strong_after += addition;
        self.sigmoid = self.create_sigmoid();
    }
}

#[derive(Clone, Debug)]

pub struct EasySigmoidCurve {
    pub amplitude: f32,
    pub center: f32,
    pub distance_from_center: f32,
    /// % away from min or max
    pub desired_percent_away: f32,
    /// If true, the curve will be flipped so that it goes from large y to small y as x increases
    pub flip_direction: bool,
}

#[derive(Clone, Debug)]

pub struct BoundedSigmoidCurve {
    pub amplitude: f32,
    pub curve_start_x: f32,
    pub curve_end_x: f32,
    pub curve_start_desired_percent: f32,
    pub flip_direction: bool,
}

impl EasySigmoidCurve {
    pub fn evaluate(&self, x: f32) -> f32 {
        let coeficient = get_coeficient_given_percent_away(self.desired_percent_away);
        let mut x = self.center - x;
        if self.flip_direction {
            x = -x;
        }
        x = x / self.distance_from_center;
        self.amplitude / (1.0 + coeficient.powf(x))
    }
}

impl BoundedSigmoidCurve {
    pub fn new(
        amplitude: f32,
        curve_start_x: f32,
        curve_end_x: f32,
        curve_start_desired_percent: f32,
        flip_direction: bool,
    ) -> Self {
        Self {
            amplitude,
            curve_start_x,
            curve_end_x,
            curve_start_desired_percent,
            flip_direction,
        }
    }

    pub fn to_easy_sigmoid(&self) -> EasySigmoidCurve {
        let center = (self.curve_end_x + self.curve_end_x) / 2.0;
        let length = (self.curve_end_x - self.curve_start_x).abs();

        EasySigmoidCurve {
            amplitude: self.amplitude,
            center,
            distance_from_center: length / 2.0,
            desired_percent_away: self.curve_start_desired_percent,
            flip_direction: self.flip_direction,
        }
    }

    pub fn evaluate(&self, x: f32) -> f32 {
        self.to_easy_sigmoid().evaluate(x)
    }
}
