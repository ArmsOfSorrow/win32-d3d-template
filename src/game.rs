use winapi::shared::windef::HWND;
use winapi::um::d3dcommon::D3D_FEATURE_LEVEL;
use winapi::um::d3d11::{ID3D11RenderTargetView, ID3D11DepthStencilView};
use winapi::um::d3d11_1::{ID3D11Device1, ID3D11DeviceContext1};
use winapi::shared::dxgi1_2::IDXGISwapChain1;
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
    //TODO: StepTimer
}

impl Game {
    pub fn on_activated() {

    }

    pub fn on_deactivated() {

    }

    pub fn on_suspending() {

    }

    pub fn on_resuming() {

    }

    pub fn on_window_size_changed() {

    }
}