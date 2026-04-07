#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Cost {
    pub energy: f32,
    pub ions: f32,
    pub neutrinos: f32,
}

impl Cost {
    pub fn with_energy(mut self, energy: f32) -> Self {
        self.energy = energy;
        self
    }

    pub fn with_ions(mut self, ions: f32) -> Self {
        self.ions = ions;
        self
    }

    pub fn with_neutrinos(mut self, neutrinos: f32) -> Self {
        self.neutrinos = neutrinos;
        self
    }

    pub fn into_values_checked(self) -> Option<Self> {
        if self.energy.is_nan()
            || self.energy.is_infinite()
            || self.ions.is_nan()
            || self.ions.is_infinite()
            || self.neutrinos.is_nan()
            || self.neutrinos.is_infinite()
        {
            None
        } else {
            Some(self)
        }
    }
}
