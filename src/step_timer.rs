use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};
use winapi::um::winnt::LARGE_INTEGER;

const TICKS_PER_SECOND: u64 = 10000000;

struct StepTimer {
    qpc_frequency: LARGE_INTEGER,
    qpc_last_time: LARGE_INTEGER,
    qpc_max_delta: u64,
    elapsed_ticks: u64,
    total_ticks: u64,
    leftover_ticks: u64,
    frame_count: u32,
    frames_per_second: u32,
    frames_this_second: u32,
    qpc_second_counter: u64,
    is_fixed_timestep: bool,
    target_elapsed_ticks: u64,
}

impl StepTimer {
    pub fn new() -> StepTimer {
        unsafe {
            let mut freq: LARGE_INTEGER = std::mem::zeroed();
            if QueryPerformanceFrequency(&mut freq) == 0 {
                panic!("QueryPerformanceFrequency failed");
            }

            let mut last_time: LARGE_INTEGER = std::mem::zeroed();
            if QueryPerformanceCounter(&mut last_time) == 0 {
                panic!("QueryPerformanceCounter failed");
            }

            let max_delta = (freq.QuadPart() / 10) as u64;

            StepTimer {
                qpc_frequency: freq,
                qpc_last_time: last_time,
                qpc_max_delta: max_delta,
                elapsed_ticks: 0,
                total_ticks: 0,
                leftover_ticks: 0,
                frame_count: 0,
                frames_per_second: 0,
                frames_this_second: 0,
                qpc_second_counter: 0,
                is_fixed_timestep: false,
                target_elapsed_ticks: TICKS_PER_SECOND / 60,
            }
        }
    }

    pub fn reset_elapsed_time(&mut self) {
        unsafe {
            if QueryPerformanceCounter(&mut self.qpc_last_time) == 0 {
                panic!("QueryPerformanceCounter failed");
            }
        }

        self.leftover_ticks = 0;
        self.frames_per_second = 0;
        self.frames_this_second = 0;
        self.qpc_second_counter = 0;
    }

    pub fn get_elapsed_ticks(&self) -> u64 {
        self.elapsed_ticks
    }

    pub fn get_elapsed_seconds(&self) -> f64 {
        //directly ported but probably doesn't need to be associated fn
        Self::ticks_to_seconds(self.elapsed_ticks)
    }

    pub fn get_total_ticks(&self) -> u64 {
        self.total_ticks
    }

    pub fn get_total_seconds(&self) -> f64 {
        Self::ticks_to_seconds(self.total_ticks)
    }

    pub fn get_frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn get_frames_per_second(&self) -> u32 {
        self.frames_per_second
    }

    pub fn set_fixed_time_step(&mut self, is_fixed_timestep: bool) {
        self.is_fixed_timestep = is_fixed_timestep;
    }

    pub fn set_target_elapsed_ticks(&mut self, target_elapsed: u64) {
        self.target_elapsed_ticks = target_elapsed;
    }

    pub fn set_target_elapsed_seconds(&mut self, target_elapsed_s: f64) {
        self.target_elapsed_ticks = Self::seconds_to_ticks(target_elapsed_s)
    }

    pub fn ticks_to_seconds(ticks: u64) -> f64 {
        ticks as f64 / TICKS_PER_SECOND as f64
    }

    pub fn seconds_to_ticks(seconds: f64) -> u64 {
        seconds as u64 * TICKS_PER_SECOND
    }
}
