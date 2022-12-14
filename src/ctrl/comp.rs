use super::*;

/// Trait for defining controls and components
pub trait Component 
{
    /// Move the component to the given position as fast as possible and returns the actual distance traveled
    ///  - The distance `dist` can be either an angle (Unit radians) or a distancce (Unit mm)
    ///  - The velocity `vel` is the maximum change rate of the distance, either angular velocity (Unit radians per secoond) or linear velocity (mm per second)
    fn drive(&mut self, dist : f32, vel : f32) -> f32;

    /// Move the component to the given position as fast as possible
    ///  - The distance `dist` can be either an angle (Unit radians) or a distancce (Unit mm)
    ///  - The velocity `vel` is the maximum change rate of the distance, either angular velocity (Unit radians per secoond) or linear velocity (mm per second) \
    /// To wait unti the movement operation is completed, use the `await inactive` function
    fn drive_async(&mut self, dist : f32, vel : f32);

    fn drive_abs(&mut self, dist : f32, vel : f32) -> f32;

    fn drive_abs_async(&mut self, dist : f32, vel : f32);

    /// Measure the component by driving the component with the velocity `vel` until either the measurement condition is true or the maximum distance `dist` 
    /// is reached. When the endpoint is reached, the controls will set the distance to `set_dist`. The lower the `accuracy`, the higher 
    /// are the computational difficulties, as the function checks more often if the measure pin has a HIGH signal
    fn measure(&mut self, dist : f32, vel : f32, set_dist : f32, accuracy : u64) -> bool;

    fn measure_async(&mut self, dist : f32, vel : f32, accuracy : u64);

    // fn lin_move(&mut self, dist : f32, vel : f32, vel_max : f32) -> f32;

    // Position
        fn get_dist(&self) -> f32;

        fn write_dist(&mut self, dist : f32);

        fn get_limit_dest(&self, pos : f32) -> LimitDest;
    // 

    // Load calculation
        fn accel_dyn(&self, vel : f32, pos : f32) -> f32;

        fn apply_load_force(&mut self, force : f32);

        fn apply_load_inertia(&mut self, inertia : f32);
    // 
}

pub trait RotElement
{

}

pub trait LinElement
{
    
}


pub trait ComponentGroup<const N : usize> 
{
    // Data
        fn comps(&self) -> &[Box<dyn Component>; N];

        fn comps_mut(&mut self) -> &mut [Box<dyn Component>; N];
    //

    fn drive(&mut self, dist : [f32; N], vel : [f32; N]) -> [f32; N] {
        let mut res = [0.0; N];
        for i in 0 .. N {
            res[i] = self.comps_mut()[i].drive(dist[i], vel[i]);
        }
        res
    }

    fn drive_abs(&mut self, dist : [f32; N], vel : [f32; N]) -> [f32; N] {
        let mut res = [0.0; N];
        for i in 0 .. N {
            res[i] = self.comps_mut()[i].drive(dist[i], vel[i]);
        }
        res
    }

    fn drive_async(&mut self, dist : [f32; N], vel : [f32; N]) {
        for i in 0 .. N {
            self.comps_mut()[i].drive_async(dist[i], vel[i]);
        }
    }

    fn drive_async_abs(&mut self, dist : [f32; N], vel : [f32; N]) {
        for i in 0 .. N {
            self.comps_mut()[i].drive_abs_async(dist[i], vel[i]);
        }
    }

    fn measure(&mut self, dist : [f32; N], vel : [f32; N], set_dist : [f32; N], accuracy : [u64; N]) -> [bool; N] {
        let mut res = [false; N];
        for i in 0 .. N {
            res[i] = self.comps_mut()[i].measure(dist[i], vel[i], set_dist[i], accuracy[i])
        }
        res
    }

    fn measure_async(&mut self, dist : [f32; N], vel : [f32; N], accuracy : [u64; N]) {
        for i in 0 .. N {
            self.comps_mut()[i].measure_async(dist[i], vel[i], accuracy[i])
        }
    }

    // Position
        fn get_dist(&self) -> [f32; N] {
            let mut dists = [0.0; N];
            for i in 0 .. N {
                dists[i] = self.comps()[i].get_dist();
            }
            dists
        }

        fn get_limit_dest(&self, dist : [f32; N]) -> [LimitDest; N] {
            let mut limits = [LimitDest::NotReached; N]; 
            for i in 0 .. N {
                limits[i] = self.comps()[i].get_limit_dest(dist[i]);
            }
            limits
        }

        fn valid_dist(&self, dist : [f32; N]) -> bool {
            let mut res = true;
            for i in 0 .. N {
                res = res & ((!self.comps()[i].get_limit_dest(dist[i]).reached()) & dist[i].is_finite()); 
            }
            res
        }

        fn valid_dist_verb(&self, dist : [f32; N]) -> [bool; N] {
            let mut res = [true; N];
            for i in 0 .. N {
                res[i] = (!self.comps()[i].get_limit_dest(dist[i]).reached()) & dist[i].is_finite(); 
            }
            res
        }
    //
}

impl<const N : usize> ComponentGroup<N> for [Box<dyn Component>; N] 
{
    fn comps(&self) -> &[Box<dyn Component>; N] {
        self
    }
    
    fn comps_mut(&mut self) -> &mut [Box<dyn Component>; N] {
        self
    }
}