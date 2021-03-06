use step_timer::StepTimer;
use winapi::shared::dxgi::{IDXGIAdapter, IDXGIDevice1};
use winapi::shared::dxgi1_2::{
    IDXGIFactory2, IDXGISwapChain1, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
};
use winapi::shared::dxgiformat::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_D24_UNORM_S8_UINT};
use winapi::shared::dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT;
use winapi::shared::minwindef::{TRUE, UINT};
use winapi::shared::windef::HWND;
use winapi::shared::winerror::{DXGI_ERROR_DEVICE_REMOVED, DXGI_ERROR_DEVICE_RESET};
use winapi::um::d3d11::{
    D3D11CreateDevice, ID3D11DepthStencilView, ID3D11Device, ID3D11DeviceContext,
    ID3D11RenderTargetView, D3D11_CLEAR_DEPTH, D3D11_CLEAR_STENCIL, D3D11_CREATE_DEVICE_DEBUG,
    D3D11_MAX_DEPTH, D3D11_MIN_DEPTH, D3D11_SDK_VERSION, D3D11_VIEWPORT,
};
use winapi::um::d3d11_1::{ID3D11Device1, ID3D11DeviceContext1};
use winapi::um::d3dcommon::{
    D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1,
    D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_9_1, D3D_FEATURE_LEVEL_9_2,
    D3D_FEATURE_LEVEL_9_3,
};
use winapi::um::unknwnbase::IUnknown;
use winapi::Interface;
use wio::com::ComPtr;

//TODO: mark everything as unsafe

pub struct Game {
    window: HWND,
    output_width: i32,
    output_height: i32,
    feature_level: D3D_FEATURE_LEVEL,
    d3d_device: Option<ComPtr<ID3D11Device1>>,
    d3d_context: Option<ComPtr<ID3D11DeviceContext1>>,
    swap_chain: Option<ComPtr<IDXGISwapChain1>>,
    render_target_view: Option<ComPtr<ID3D11RenderTargetView>>,
    depth_stencil_view: Option<ComPtr<ID3D11DepthStencilView>>,
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
                d3d_device: None,
                d3d_context: None,
                swap_chain: None,
                render_target_view: None,
                depth_stencil_view: None,
                timer: StepTimer::new(),
            }
        }
    }
    pub unsafe fn initialize(&mut self, window: HWND, width: i32, height: i32) {
        self.window = window;
        self.output_width = std::cmp::max(width, 1);
        self.output_height = std::cmp::max(height, 1);

        self.create_device();
        self.create_resources();
    }

    pub fn tick(&mut self) {
        //this is kinda sucky to port from c++ and I don't know how to fix it.
        //we can't call self.update because self is already borrowed when we take the timer.
        //stackoverflow suggests borrowing only the fields you need instead of taking self as param:
        //https://stackoverflow.com/questions/29896672/can-you-control-borrowing-a-struct-vs-borrowing-a-field
        //for now let's just leave the update function without access to Game (avoid the borrow)
        {
            let timer = &mut self.timer;
            timer.tick(|t| Game::update(t));
        }

        self.render();
    }

    fn update(timer: &StepTimer) {
        let elapsed_time = timer.get_elapsed_seconds() as f32;

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
        if self.render_target_view.is_some()
            && self.depth_stencil_view.is_some()
            && self.d3d_context.is_some()
        {
            let rtv = &mut self.render_target_view.as_ref().unwrap();
            let dsv = &mut self.depth_stencil_view.as_ref().unwrap();
            let context = self.d3d_context.as_ref().unwrap();
            unsafe {
                context.ClearRenderTargetView(rtv.as_raw(), &[0.0, 0.0, 0.5, 1.0f32]);
                context.ClearDepthStencilView(
                    dsv.as_raw(),
                    D3D11_CLEAR_DEPTH | D3D11_CLEAR_STENCIL,
                    1.0f32,
                    0,
                );
                context.OMSetRenderTargets(1, &rtv.as_raw(), dsv.as_raw());

                let viewport = D3D11_VIEWPORT {
                    TopLeftX: 0.0f32,
                    TopLeftY: 0.0f32,
                    Width: self.output_width as f32,
                    Height: self.output_height as f32,
                    MinDepth: D3D11_MIN_DEPTH,
                    MaxDepth: D3D11_MAX_DEPTH,
                };

                context.RSSetViewports(1, &viewport);
            }
        }
    }

    fn present(&mut self) {
        // The first argument instructs DXGI to block until VSync, putting the application
        // to sleep until the next VSync. This ensures we don't waste any cycles rendering
        // frames that will never be displayed to the screen.
        unsafe {
            let hr = self.swap_chain.as_ref().unwrap().Present(1, 0); //TODO: get rid of unwraps

            // If the device was reset we must completely reinitialize the renderer.
            if hr == DXGI_ERROR_DEVICE_REMOVED || hr == DXGI_ERROR_DEVICE_RESET {
                self.on_device_lost();
            } else {
                //not sure what to do about this one. In C++ you can theoretically catch it but
                //would anyone do that in practice?
                panic!("Present failed without device removed or reset error");
            }
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

    pub unsafe fn on_window_size_changed(&mut self, width: i32, height: i32) {
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

    unsafe fn create_device(&mut self) {
        let mut creation_flags: UINT = 0;

        #[cfg(debug_assertions)]
        {
            creation_flags |= D3D11_CREATE_DEVICE_DEBUG;
        }

        let feature_levels = [
            // TODO: Modify for supported Direct3D feature levels
            D3D_FEATURE_LEVEL_11_1,
            D3D_FEATURE_LEVEL_11_0,
            D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_10_0,
            D3D_FEATURE_LEVEL_9_3,
            D3D_FEATURE_LEVEL_9_2,
            D3D_FEATURE_LEVEL_9_1,
        ];

        let mut device_ptr: *mut ID3D11Device = std::ptr::null_mut();
        let mut context_ptr: *mut ID3D11DeviceContext = std::ptr::null_mut();
        let hr = D3D11CreateDevice(
            std::ptr::null_mut(), // specify nullptr to use the default adapter
            D3D_DRIVER_TYPE_HARDWARE,
            std::ptr::null_mut(),
            creation_flags,
            &feature_levels[0],
            feature_levels.len() as u32,
            D3D11_SDK_VERSION,
            &mut device_ptr,
            &mut self.feature_level,
            &mut context_ptr,
        );

        if ::failed(hr) {
            panic!("D3D11CreateDevice failed with HRESULT {:x}", hr);
        }

        //TODO: debug layer support
        let device = ComPtr::from_raw(device_ptr)
            .cast::<ID3D11Device1>()
            .unwrap();
        self.d3d_device = Some(device);
        let context = ComPtr::from_raw(context_ptr)
            .cast::<ID3D11DeviceContext1>()
            .unwrap();
        self.d3d_context = Some(context);

        // TODO: Initialize device dependent objects here (independent of window size).
    }

    // Allocate all memory resources that change on a window SizeChanged event.
    unsafe fn create_resources(&mut self) {
        let null_views: [*mut ID3D11RenderTargetView; 1] = [std::ptr::null_mut()];

        {
            let context = self.d3d_context.as_ref().unwrap();
            context.OMSetRenderTargets(0, &null_views[0], std::ptr::null_mut());
            self.render_target_view = None;
            self.depth_stencil_view = None;
            context.Flush();
        }

        let back_buffer_width = self.output_width;
        let back_buffer_height = self.output_height;
        let back_buffer_format = DXGI_FORMAT_B8G8R8A8_UNORM;
        let depth_buffer_format = DXGI_FORMAT_D24_UNORM_S8_UINT;
        let buffer_count = 2;

        // If the swap chain already exists, resize it, otherwise create one.
        if self.swap_chain.is_some() {
            let hr = self.swap_chain.as_ref().unwrap().ResizeBuffers(
                buffer_count,
                back_buffer_width as u32,
                back_buffer_height as u32,
                back_buffer_format,
                0,
            );

            if hr == DXGI_ERROR_DEVICE_REMOVED || hr == DXGI_ERROR_DEVICE_RESET {
                // If the device was removed for any reason, a new device and swap chain will need to be created.
                self.on_device_lost();

                // Everything is set up now. Do not continue execution of this method. OnDeviceLost will reenter this method
                // and correctly set up the new device.
                return;
            } else {
                panic!("Unexpected error: {}", hr);
            }
        } else {
            // First, retrieve the underlying DXGI Device from the D3D Device.
            let dxgi_device = self
                .d3d_device
                .as_ref()
                .unwrap()
                .cast::<IDXGIDevice1>()
                .expect("D3D device doesn't support IDXGIDevice1 interface");

            // Identify the physical adapter (GPU or card) this device is running on.
            let mut dxgi_adapter_ptr = std::ptr::null_mut();
            let mut hr = dxgi_device.GetAdapter(&mut dxgi_adapter_ptr);
            if ::failed(hr) {
                panic!("Couldn't get physical adapter, HRESULT: {}", hr);
            }
            let dxgi_adapter = ComPtr::from_raw(dxgi_adapter_ptr);

            let mut dxgi_factory_ptr: *mut IDXGIFactory2 = std::ptr::null_mut();
            //this looks...wild. I really should check if there's a better way to do this.
            hr = dxgi_adapter.GetParent(
                &IDXGIFactory2::uuidof(),
                &mut dxgi_factory_ptr as *mut *mut _ as *mut *mut winapi::ctypes::c_void,
            );
            if ::failed(hr) {
                panic!("Couldn't get DXGI factory, HRESULT: {}", hr);
            }
            let dxgi_factory = ComPtr::from_raw(dxgi_factory_ptr);

            let mut swap_chain_desc: DXGI_SWAP_CHAIN_DESC1 = std::mem::zeroed();
            swap_chain_desc.Width = back_buffer_width as u32;
            swap_chain_desc.Height = back_buffer_height as u32;
            swap_chain_desc.Format = back_buffer_format;
            swap_chain_desc.SampleDesc.Count = 1;
            swap_chain_desc.SampleDesc.Quality = 0;
            swap_chain_desc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
            swap_chain_desc.BufferCount = buffer_count;

            let mut fullscreen_swap_chain_desc: DXGI_SWAP_CHAIN_FULLSCREEN_DESC =
                std::mem::zeroed();
            fullscreen_swap_chain_desc.Windowed = TRUE;

            let mut swap_chain_ptr: *mut IDXGISwapChain1 = std::ptr::null_mut();
            hr = dxgi_factory.CreateSwapChainForHwnd(
                self.d3d_device.as_ref().unwrap().as_raw() as *mut IUnknown, //this looks kinda ugly too.
                self.window,
                &swap_chain_desc,
                &fullscreen_swap_chain_desc,
                std::ptr::null_mut(),
                &mut swap_chain_ptr,
            );
            if ::failed(hr) {
                panic!("Failed to create swap chain, HRESULT {}", hr);
            }
        }
    }

    unsafe fn on_device_lost(&mut self) {
        // TODO: Add Direct3D resource cleanup here.

        self.depth_stencil_view = None;
        self.render_target_view = None;
        self.swap_chain = None;
        self.d3d_context = None;
        self.d3d_device = None;
        self.create_device();
        self.create_resources();
    }
}
