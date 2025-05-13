#[cfg(feature = "f16")]
mod elems {
    pub type ElemType = burn::tensor::f16;
    pub const DTYPE_NAME: &str = "f16";
}

#[cfg(feature = "bf16")]
mod elems {
    pub type ElemType = burn::tensor::bf16;
    pub const DTYPE_NAME: &str = "bf16";
}

#[cfg(not(any(feature = "f16", feature = "bf16")))]
mod elems {
    pub type ElemType = f32;
    pub const DTYPE_NAME: &str = "f32";
}

pub use elems::*;

// Candle --------------------------------------------------------------------

#[cfg(any(feature = "candle-accelerate", feature = "candle-cpu",))]
pub mod burn_backend_types {
    use super::*;
    use burn::backend::candle::{Candle, CandleDevice};
    pub type InferenceBackend = Candle<ElemType>;
    pub type InferenceDevice = CandleDevice;
    pub const INFERENCE_DEVICE: std::sync::LazyLock<CandleDevice> =
        std::sync::LazyLock::new(|| CandleDevice::Cpu);
    pub const NAME: &str = "candle-cpu";
}

#[cfg(feature = "candle-cuda")]
pub mod burn_backend_types {
    use super::*;
    use burn::backend::candle::{Candle, CandleDevice};
    pub type InferenceBackend = Candle<ElemType>;
    pub type InferenceDevice = CandleDevice;
    pub const INFERENCE_DEVICE: std::sync::LazyLock<CandleDevice> =
        std::sync::LazyLock::new(|| CandleDevice::cuda(0));
    pub const NAME: &str = "candle-cuda";
}

#[cfg(feature = "candle-metal")]
pub mod burn_backend_types {
    use super::*;
    use burn::backend::candle::{Candle, CandleDevice};
    pub type InferenceBackend = Candle<ElemType>;
    pub type InferenceDevice = CandleDevice;
    pub const INFERENCE_DEVICE: std::sync::LazyLock<CandleDevice> =
        std::sync::LazyLock::new(|| CandleDevice::metal(0));
    pub const NAME: &str = "candle-metal";
}

// Cuda ----------------------------------------------------------------------

#[cfg(feature = "cuda")]
pub mod burn_backend_types {
    use super::*;
    use burn::backend::cuda::{Cuda, CudaDevice};
    pub type InferenceBackend = Cuda<ElemType>;
    pub type InferenceDevice = CudaDevice;
    pub const INFERENCE_DEVICE: std::sync::LazyLock<CudaDevice> =
        std::sync::LazyLock::new(|| CudaDevice::default());
    pub const NAME: &str = "cuda";
}

// ROCm ----------------------------------------------------------------------

#[cfg(feature = "rocm")]
pub mod burn_backend_types {
    use super::*;
    use burn::backend::rocm::{HipDevice, Rocm};
    pub type InferenceBackend = Rocm<ElemType>;
    pub type InferenceDevice = HipDevice;
    pub const INFERENCE_DEVICE: std::sync::LazyLock<HipDevice> =
        std::sync::LazyLock::new(|| HipDevice::default());
    pub const NAME: &str = "rocm";
}

// ndarray -------------------------------------------------------------------
// This backend is used for testing and by default when no backend is selected.

#[cfg(any(feature = "ndarray", not(feature = "selected-backend")))]
pub mod burn_backend_types {
    use super::*;
    use burn::backend::ndarray::{NdArray, NdArrayDevice};

    pub type InferenceBackend = NdArray<ElemType>;
    pub type InferenceDevice = NdArrayDevice;
    pub const INFERENCE_DEVICE: InferenceDevice = NdArrayDevice::Cpu;
    pub const NAME: &str = "ndarray";
}

// LibTorch ------------------------------------------------------------------

#[cfg(feature = "libtorch")]
pub mod burn_backend_types {
    use super::*;
    use burn::backend::libtorch::{LibTorch, LibTorchDevice};

    pub type InferenceBackend = LibTorch<ElemType>;
    pub type InferenceDevice = LibTorchDevice;

    #[cfg(not(target_os = "macos"))]
    pub const INFERENCE_DEVICE: std::sync::LazyLock<LibTorchDevice> =
        std::sync::LazyLock::new(|| LibTorchDevice::Cuda(0));
    #[cfg(target_os = "macos")]
    pub const INFERENCE_DEVICE: InferenceDevice = LibTorchDevice::Mps;
    pub const NAME: &str = "libtorch";
}

#[cfg(feature = "libtorch-cpu")]
pub mod burn_backend_types {
    use super::*;
    use burn::backend::libtorch::{LibTorch, LibTorchDevice};

    pub type InferenceBackend = LibTorch<ElemType>;
    pub type InferenceDevice = LibTorchDevice;
    pub const INFERENCE_DEVICE: InferenceDevice = LibTorchDevice::Cpu;
    pub const NAME: &str = "libtorch-cpu";
}

// WebGPU --------------------------------------------------------------------

#[cfg(any(feature = "wgpu", feature = "vulkan", feature = "metal"))]
pub mod burn_backend_types {
    use super::*;
    use burn::backend::wgpu::{Wgpu, WgpuDevice};
    pub type InferenceBackend = Wgpu<ElemType>;
    pub type InferenceDevice = WgpuDevice;
    pub const INFERENCE_DEVICE: InferenceDevice = WgpuDevice::DefaultDevice;
    #[cfg(all(feature = "wgpu", not(feature = "vulkan"), not(feature = "metal")))]
    pub const NAME: &str = "wgpu";
    #[cfg(feature = "vulkan")]
    pub const NAME: &str = "vulkan";
    #[cfg(feature = "metal")]
    pub const NAME: &str = "metal";
}

#[cfg(feature = "wgpu-cpu")]
pub mod burn_backend_types {
    use super::*;
    use burn::backend::wgpu::{Wgpu, WgpuDevice};
    pub type InferenceBackend = Wgpu<ElemType>;
    pub type InferenceDevice = WgpuDevice;
    pub const INFERENCE_DEVICE: InferenceDevice = WgpuDevice::Cpu;
    pub const NAME: &str = "wgpu-cpu";
}
