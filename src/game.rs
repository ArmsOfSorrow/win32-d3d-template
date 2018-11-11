use step_timer::StepTimer;
use winapi::shared::dxgi1_2::IDXGISwapChain1;
use winapi::shared::windef::HWND;
use winapi::um::d3d11::{ID3D11DepthStencilView, ID3D11RenderTargetView};
use winapi::um::d3d11_1::{ID3D11Device1, ID3D11DeviceContext1};
use winapi::um::d3dcommon::{D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_9_1};
use wio::com::ComPtr;

pub struct Game {
    window: HWND,
    output_width: i32,
    output_height: i32,
    feature_level: D3D_FEATURE_LEVEL,
    d3d_device: ComPtr<ID3D11Device1>,
    d3d_context: ComPtr<ID3D11DeviceContext1>,
    swap_chain: ComPtr<IDXGISwapChain1>,
    render_target_view: ComPtr<ID3D11RenderTargetView>,
    depth_stencil_view: ComPtr<ID3D11DepthStencilView>,
    timer: StepTimer,
}

impl Game {
    pub fn new() -> Game {
        unsafe {
            Game {
                window: std::ptr::null_mut(),
                output_width: 800,
                output_height: 600,
                feature_level: D3D_FEATURE_LEVEL_9_1,
                d3d_device: ComPtr::from_raw(std::ptr::null_mut()),
                d3d_context: ComPtr::from_raw(std::ptr::null_mut()),
                swap_chain: ComPtr::from_raw(std::ptr::null_mut()),
                render_target_view: ComPtr::from_raw(std::ptr::null_mut()),
                depth_stencil_view: ComPtr::from_raw(std::ptr::null_mut()),
                timer: StepTimer::new(),
            }
        }
    }
    pub fn initialize(&mut self, window: HWND, width: i32, height: i32) {
        self.window = window;
        self.output_width = std::cmp::max(width, 1);
        self.output_height = std::cmp::max(height, 1);

        self.create_device();
        self.create_resources();
    }

    pub fn tick() {}

    pub fn on_activated() {}

    pub fn on_deactivated() {}

    pub fn on_suspending() {}

    pub fn on_resuming() {}

    pub fn on_window_size_changed() {}

    pub fn get_default_size(width: i32, height: i32) {}

    fn update(timer: &StepTimer) {}
    fn render() {}
    fn clear() {}
    fn present() {}
    fn create_device(&mut self) {}
    fn create_resources(&mut self) {}
    fn on_device_lost() {}
}
