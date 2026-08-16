#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Struktura poravnana na 32 bajta za ultra-brzi AVX2 rad
#[repr(C, align(32))]
pub struct AlignedSimdBuffer {
    pub data: Vec<f32>,
}

pub struct QuantumSimdEngine {
    pub has_avx2: bool,
    pub has_sse2: bool,
}

impl QuantumSimdEngine {
    pub fn new() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Self {
                has_avx2: is_x86_feature_detected!("avx2"),
                has_sse2: is_x86_feature_detected!("sse2"),
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            Self {
                has_avx2: false,
                has_sse2: false,
            }
        }
    }

    // =========================================================================
    // 1. MATEMATIČKE VEKTORSKE OPERACIJE
    // =========================================================================

    /// SIMD Vektorsko sabiranje niza float32 brojeva: C[i] = A[i] + B[i]
    pub fn add_f32_buffers(&self, a: &[f32], b: &[f32], result: &mut [f32]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), result.len());

        let len = a.len();

        #[cfg(target_arch = "x86_64")]
        if self.has_avx2 && len >= 8 {
            unsafe {
                self.add_f32_avx2(a, b, result);
                return;
            }
        }

        for i in 0..len {
            result[i] = a[i] + b[i];
        }
    }

    #[target_feature(enable = "avx2")]
    #[cfg(target_arch = "x86_64")]
    unsafe fn add_f32_avx2(&self, a: &[f32], b: &[f32], result: &mut [f32]) { unsafe {
        let len = a.len();
        let chunks = len / 8;
        let remainder = len % 8;

        for i in 0..chunks {
            let offset = i * 8;
            let va = _mm256_loadu_ps(a.as_ptr().add(offset));
            let vb = _mm256_loadu_ps(b.as_ptr().add(offset));
            let vres = _mm256_add_ps(va, vb);
            _mm256_storeu_ps(result.as_mut_ptr().add(offset), vres);
        }

        for i in (len - remainder)..len {
            result[i] = a[i] + b[i];
        }
    }}

    /// Skaliranje zvuka ili svetline (Gain scaling)
    pub fn scale_buffer_simd(&self, buffer: &mut [f32], factor: f32) {
        let len = buffer.len();

        #[cfg(target_arch = "x86_64")]
        if self.has_avx2 && len >= 8 {
            unsafe {
                let factor_vec = _mm256_set1_ps(factor);
                let chunks = len / 8;

                for i in 0..chunks {
                    let offset = i * 8;
                    let ptr = buffer.as_mut_ptr().add(offset);
                    let vbuf = _mm256_loadu_ps(ptr);
                    let vscaled = _mm256_mul_ps(vbuf, factor_vec);
                    _mm256_storeu_ps(ptr, vscaled);
                }

                for i in (len - (len % 8))..len {
                    buffer[i] *= factor;
                }
                return;
            }
        }

        for val in buffer.iter_mut() {
            *val *= factor;
        }
    }

    // =========================================================================
    // 2. KERNEL MEMORY PRIMITIVES (SIMD memcpy & memset)
    // =========================================================================

    /// Ultra-brzo kopiranje memorije preko 256-bitnih registara (AVX2 Memcpy)
    pub fn simd_memcpy(&self, src: &[u8], dst: &mut [u8]) {
        assert_eq!(src.len(), dst.len());
        let len = src.len();

        #[cfg(target_arch = "x86_64")]
        if self.has_avx2 && len >= 32 {
            unsafe {
                let chunks = len / 32;
                for i in 0..chunks {
                    let offset = i * 32;
                    // Učitaj 32 bajta odjednom u YMM registar
                    let chunk = _mm256_loadu_si256(src.as_ptr().add(offset) as *const __m256i);
                    // Upši 32 bajta u destinaciju
                    _mm256_storeu_si256(dst.as_mut_ptr().add(offset) as *mut __m256i, chunk);
                }

                let remainder = len % 32;
                for i in (len - remainder)..len {
                    dst[i] = src[i];
                }
                return;
            }
        }

        dst.copy_from_slice(src);
    }

    /// Fast RAM Zero / Memset za sistemske stranice
    pub fn simd_memset(&self, dst: &mut [u8], value: u8) {
        let len = dst.len();

        #[cfg(target_arch = "x86_64")]
        if self.has_avx2 && len >= 32 {
            unsafe {
                let zero_vec = _mm256_set1_epi8(value as i8);
                let chunks = len / 32;

                for i in 0..chunks {
                    let offset = i * 32;
                    _mm256_storeu_si256(dst.as_mut_ptr().add(offset) as *mut __m256i, zero_vec);
                }

                let remainder = len % 32;
                for i in (len - remainder)..len {
                    dst[i] = value;
                }
                return;
            }
        }

        dst.fill(value);
    }

    // =========================================================================
    // 3. GRAPHICS & GOP FRAMEBUFFER ACCELERATION (Pixel Blitting)
    // =========================================================================

    /// Brzo popunjavanje ekrana (Framebuffer Fill) u 32-bitnoj RGBA/BGRA paleti
    pub fn fill_framebuffer_32bpp(&self, framebuffer: &mut [u32], color_argb: u32) {
        let len = framebuffer.len();

        #[cfg(target_arch = "x86_64")]
        if self.has_avx2 && len >= 8 {
            unsafe {
                // Popuni YMM sa 8 kopija istog 32-bitnog piksela
                let color_vec = _mm256_set1_epi32(color_argb as i32);
                let chunks = len / 8;

                for i in 0..chunks {
                    let offset = i * 8;
                    let ptr = framebuffer.as_mut_ptr().add(offset) as *mut __m256i;
                    _mm256_storeu_si256(ptr, color_vec);
                }

                let remainder = len % 8;
                for i in (len - remainder)..len {
                    framebuffer[i] = color_argb;
                }
                return;
            }
        }

        framebuffer.fill(color_argb);
    }
}