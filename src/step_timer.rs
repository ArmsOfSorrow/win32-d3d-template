use winapi::shared::ntdef::LARGE_INTEGER;

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
    target_elapsed_ticks: u64
}

impl StepTimer {
    pub fn get_elapsed_ticks(&self) -> u64 {
        self.elapsed_ticks
    }

    pub fn get_total_ticks(&self) -> u64 {
        self.total_ticks
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
}