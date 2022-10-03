use std::{f64::consts::{E, PI}, ops::Index};

use super::data::StepperData;

/// Returns the current torque of a motor (data) at the given polarization frequency (f)
/// Unit: [Nm]
pub fn torque(data : &StepperData, f : f64) -> f64
{
    if f == 0.0 {
        return data.t_s;
    }

    let tau = data.tau();
    let pow = E.powf( -1.0 / tau / f );

    return (1.0 - pow) / (1.0 + pow) * data.t_s;
}

/// Returns the start freqency of a motor (data)
/// Unit: [Hz]
pub fn start_frequency(data : &StepperData) -> f64
{
    return (data.t_s / data.j * (data.n_s as f64) / 4.0 / PI).powf(0.5);
}

/// The angluar velocity of a motor that is constantly accelerating after the time t [in s], [in s^-1]
pub fn angluar_velocity(data : &StepperData, t : f64) -> f64
{
    return data.alpha_max() * (t + data.tau()*E.powf(-t/data.tau()));
}

/// 
pub fn acc_curve(data : &StepperData, t_min : f64, max_len : u64) -> Vec<f64> 
{
    let mut list : Vec<f64> = vec![
        1.0 / start_frequency(data)
    ];

    let mut t_total = list[0];
    for i in 1 .. max_len {
        list.push(2.0 * PI / (data.n_s as f64) / angluar_velocity(data, t_total));
        t_total += list.index(i as usize);

        if *list.index(i as usize) < t_min {
            return list;
        }
    };

    return list;
}
