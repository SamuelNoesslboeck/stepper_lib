use crate::{ctrl::{Component, LimitType, LimitDest}};

use crate::comp::Cylinder;

pub struct CylinderTriangle 
{
    // Cylinder
    pub cylinder : Cylinder,

    // Triangle
    pub l_a : f32,
    pub l_b : f32
}

impl CylinderTriangle 
{
    pub fn new(cylinder : Cylinder, l_a : f32, l_b : f32) -> Self
    {
        let mut tri = CylinderTriangle {
            l_a, 
            l_b,
            cylinder 
        };

        tri.cylinder.write_dist(l_a.max(l_b));

        return tri;
    }

    // Conversions
        /// Returns the cylinder length for the given angle gamma
        pub fn len_for_gam(&self, gam : f32) -> f32 {
            (self.l_a.powi(2) + self.l_b.powi(2) - 2.0 * self.l_a * self.l_b * gam.cos()).powf(0.5)
        }

        /// Returns the angle gamma for the given cylinder length _(len < (l_a + l_b))_
        pub fn gam_for_len(&self, len : f32) -> f32 {
            ((self.l_a.powi(2) + self.l_b.powi(2) - len.powi(2)) / 2.0 / self.l_a / self.l_b).acos()
        }

        // Other angles
        pub fn alpha_for_gam(&self, gam : f32) -> f32 {
            (self.l_a / self.len_for_gam(gam) * gam.sin()).asin()
        }

        pub fn beta_for_gam(&self, gam : f32) -> f32 {
            (self.l_b / self.len_for_gam(gam) * gam.sin()).asin()
        }

        pub fn omega_for_gam(&self, vel : f32, gam : f32) -> f32 {
            vel / self.l_a * self.beta_for_gam(gam).sin()
        }

        pub fn vel_for_gam(&self, omega : f32, gam : f32) -> f32 {
            omega * self.l_a / self.beta_for_gam(gam).sin()
        }
    //

    // Limit
        pub fn set_limit(&mut self, limit_min : LimitType, limit_max : LimitType) {
            self.cylinder.set_limit(limit_max, limit_min)
        }
    //
}

impl Component for CylinderTriangle {
    /// See [Component::drive()](`Component::drive()`)
    /// - `dist`is the angular distance to be moved (Unit radians)
    /// - `vel` is the cylinders extend velocity (Unit mm per second)
    fn drive(&mut self, dist : f32, vel : f32) -> f32 {
        self.cylinder.drive(self.len_for_gam(dist) - self.cylinder.get_dist(), vel)
    }

    /// See [Component::drive_async()](`Component::drive_async()`)
    /// - `dist`is the angular distance to be moved (Unit radians)
    /// - `vel` is the cylinders extend velocity (Unit mm per second)
    fn drive_async(&mut self, dist : f32, vel : f32) {
        self.cylinder.drive_async(self.len_for_gam(dist) - self.cylinder.get_dist(), vel)
    }

    /// See [Component::measure()](`Component::measure()`)
    /// - `dist` is the maximum distance for the cylinder in mm
    /// - `vel` is the maximum linear velocity for the cylinder in mm per second
    /// - `set_dist` is the set distance for the cylinder in mm
    fn measure(&mut self, dist : f32, vel : f32, set_dist : f32, accuracy : u64) -> bool {
        self.cylinder.measure(dist, vel, set_dist, accuracy)
    }

    fn measure_async(&mut self, dist : f32, vel : f32, accuracy : u64) {
        self.cylinder.measure_async(dist, vel, accuracy)
    }

    // Distance
        fn get_dist(&self) -> f32 {
            self.gam_for_len(self.cylinder.get_dist())
        }

        /// See [Component::drive_abs](`Component::drive_abs()`)
        /// - `dist`is the angular distance to be moved (Unit radians)
        /// - `vel` is the cylinders extend velocity (Unit mm per second)
        fn drive_abs(&mut self, dist : f32, vel : f32) -> f32 {
            self.cylinder.drive_abs(self.len_for_gam(dist), vel)
        }

        fn drive_abs_async(&mut self, dist : f32, vel : f32) {
            self.cylinder.drive_abs_async(self.len_for_gam(dist), vel)
        }

        fn write_dist(&mut self, dist : f32) {
            self.cylinder.write_dist(self.len_for_gam(dist))
        }

        fn get_limit_dest(&self, gam : f32) -> LimitDest {
            match self.cylinder.get_limit_dest(self.len_for_gam(gam)) {
                LimitDest::Maximum(dist) => LimitDest::Maximum(self.gam_for_len(dist)),
                LimitDest::Minimum(dist) => LimitDest::Minimum(self.gam_for_len(dist)),
                other => other  
            }
        }
    // 
    
    // Forces
        fn accel_dyn(&self, vel : f32, pos : f32) -> f32 {
            self.omega_for_gam(self.cylinder.accel_dyn(self.vel_for_gam(vel, pos), pos), pos)
        }

        fn apply_load_force(&mut self, force : f32) {
            self.cylinder.apply_load_force(force)
        }

        fn apply_load_inertia(&mut self, inertia : f32) {
            self.cylinder.apply_load_inertia(inertia)
        }
    // 
}