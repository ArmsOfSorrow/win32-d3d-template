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

    pub fn tick<F>(&mut self, update_func: F)
    where
        F: Fn(),
    {
        unsafe {
            let mut current_time: LARGE_INTEGER = std::mem::zeroed();

            if QueryPerformanceCounter(&mut current_time) == 0 {
                panic!("QueryPerformanceCounter failed");
            }

            let mut time_delta = (current_time.QuadPart() - self.qpc_last_time.QuadPart()) as u64;
            self.qpc_last_time = current_time;
            self.qpc_second_counter += time_delta;

            // Clamp excessively large time deltas (e.g. after paused in the debugger).
            if time_delta > self.qpc_max_delta {
                time_delta = self.qpc_max_delta;
            }

            // Convert QPC units into a canonical tick format. This cannot overflow due to the previous clamp.
            time_delta *= TICKS_PER_SECOND;
            time_delta /= *self.qpc_frequency.QuadPart() as u64;

            let last_frame_count = self.frame_count;

            if self.is_fixed_timestep {
                // Fixed timestep update logic

                // If the app is running very close to the target elapsed time (within 1/4 of a millisecond) just clamp
                // the clock to exactly match the target value. This prevents tiny and irrelevant errors
                // from accumulating over time. Without this clamping, a game that requested a 60 fps
                // fixed update, running with vsync enabled on a 59.94 NTSC display, would eventually
                // accumulate enough tiny errors that it would drop a frame. It is better to just round
                // small deviations down to zero to leave things running smoothly.
                if (time_delta - self.target_elapsed_ticks) < TICKS_PER_SECOND / 4000 {
                    time_delta = self.target_elapsed_ticks;
                }

                self.leftover_ticks += time_delta;

                while self.leftover_ticks >= self.target_elapsed_ticks {
                    self.elapsed_ticks = self.target_elapsed_ticks;
                    self.total_ticks += self.target_elapsed_ticks;
                    self.leftover_ticks -= self.target_elapsed_ticks;
                    self.frame_count += 1;

                    update_func();
                }
            } else {
                // Variable timestep update logic.
                self.elapsed_ticks = time_delta;
                self.total_ticks += time_delta;
                self.leftover_ticks = 0;
                self.frame_count += 1;

                update_func();
            }

            // Track the current framerate
            if self.frame_count != last_frame_count {
                self.frames_this_second += 1;
            }

            if self.qpc_second_counter >= *self.qpc_frequency.QuadPart() as u64 {
                self.frames_per_second = self.frames_this_second;
                self.frames_this_second = 0;
                self.qpc_second_counter %= *self.qpc_frequency.QuadPart() as u64;
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
