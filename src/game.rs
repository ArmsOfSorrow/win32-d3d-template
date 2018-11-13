use step_timer::StepTimer;
use winapi::shared::dxgi1_2::IDXGISwapChain1;
use winapi::shared::windef::HWND;
use winapi::um::d3d11::{
    ID3D11DepthStencilView, ID3D11RenderTargetView, D3D11_CLEAR_DEPTH, D3D11_CLEAR_STENCIL,
    D3D11_MAX_DEPTH, D3D11_MIN_DEPTH, D3D11_VIEWPORT,
};
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

    pub fn tick(&mut self) {
        //this is kinda sucky to port from c++

        self.timer.tick(|t| {});

        self.render();
    }

    fn update(&mut self, timer: &mut StepTimer) {
        let elapsed_time = self.timer.get_elapsed_seconds() as f32;

        // TODO: Add your game logic here
    }

    fn render(&mut self) {
        // Don't try to render anything before the first Update.
        if self.timer.get_frame_count() == 0 {
            return;
        }

        self.clear();

        // TODO: Add your rendering code here.

        self.present();
    }

    fn clear(&mut self) {
        let render_target_view = &mut self.render_target_view;
        let depth_stencil_view = &mut self.depth_stencil_view;
        unsafe {
            self.d3d_context
                .ClearRenderTargetView(render_target_view.as_raw(), &[0.0, 0.0, 0.5, 1.0f32]);
            self.d3d_context.ClearDepthStencilView(
                depth_stencil_view.as_raw(),
                D3D11_CLEAR_DEPTH | D3D11_CLEAR_STENCIL,
                1.0f32,
                0,
            );
            self.d3d_context.OMSetRenderTargets(
                1,
                &render_target_view.as_raw(),
                depth_stencil_view.as_raw(),
            );

            let viewport = D3D11_VIEWPORT {
                TopLeftX: 0.0f32,
                TopLeftY: 0.0f32,
                Width: self.output_width as f32,
                Height: self.output_height as f32,
                MinDepth: D3D11_MIN_DEPTH,
                MaxDepth: D3D11_MAX_DEPTH,
            };

            self.d3d_context.RSSetViewports(1, &viewport);
        }
    }

    pub fn on_activated(&mut self) {
        // TODO: Game is becoming active window.
    }

    pub fn on_deactivated(&mut self) {
        // TODO: Game is becoming background window.
    }

    pub fn on_suspending(&mut self) {
        // TODO: Game is being power-suspended (or minimized).
    }

    pub fn on_resuming(&mut self) {
        self.timer.reset_elapsed_time();

        // TODO: Game is being power-resumed (or returning from minimize).
    }

    pub fn on_window_size_changed(&mut self, width: i32, height: i32) {
        self.output_width = std::cmp::max(width, 1);
        self.output_height = std::cmp::min(height, 1);

        self.create_resources();

        // TODO: Game window is being resized.
    }

    pub fn get_default_size(&self, width: &mut i32, height: &mut i32) {
        // TODO: Change to desired default window size (note minimum size is 320x200).
        *width = 800;
        *height = 600;
    }

    fn present(&mut self) {}
    fn create_device(&mut self) {}
    fn create_resources(&mut self) {}
    fn on_device_lost() {}
}
