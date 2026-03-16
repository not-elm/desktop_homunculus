# STT Inference Latency Optimization: Dynamic `audio_ctx`

## Problem

Whisper large-v3-turbo on CPU takes 7.2 seconds to process a 0.8-second audio chunk. The root cause is that whisper.cpp's encoder processes a fixed 30-second context window (`audio_ctx=1500`) regardless of actual input length, wasting ~97% of computation on zero-padded silence for short chunks.

### Evidence

```
VAD: emitting chunk seq=6, 12480 samples (0.8s)
Inference: received chunk seq=6, 12480 samples (0.8s), queue_latency=0ms
Inference: completed in 7169ms
```

VAD and queue latency are negligible. The entire 7.2s is spent in `whisper_full()`.

## Solution

Dynamically set `audio_ctx` based on actual audio length to reduce encoder computation.

### Changed File

`engine/crates/homunculus_microphone/src/inference.rs` only.

### Formula

```
encoder_tokens = div_ceil(padded_sample_count, 320)
audio_ctx = clamp(ceil_to_64(encoder_tokens + 128), 768, ctx.model_n_audio_ctx())
```

Where:
- `padded_sample_count` is the sample count **after** `pad_short_chunk()` (minimum 16000 = 1s at 16kHz)
- `320` = mel spectrogram hop_length (160) x encoder stride (2). One encoder token per 320 audio samples.
- `+128` = safety margin for decoder search space. Not an official recommendation but used in practice (whisper.cpp issue #1855). Conservative buffer to avoid decoder instability.
- `ceil_to_64` = round up to nearest multiple of 64. Performance optimization, not a hard alignment requirement (whisper.cpp discussion #297).
- **Lower bound 768**: Upstream-recommended minimum. The whisper.cpp maintainer noted values below 768 produced poor results in streaming experiments (discussion #297).
- **Upper bound `ctx.model_n_audio_ctx()`**: Queried at runtime from the loaded model's architectural parameters via `whisper-rs` API (`whisper_model_n_audio_ctx`), avoiding hardcoded assumptions. This returns the model's inherent context size (e.g. 1500) regardless of any runtime `set_audio_ctx` overrides, unlike `n_audio_ctx()` which reflects runtime state.

### Effective Values

With the current VAD configuration (chunks: 0.3s-8s, `max_chunk_ms=8000`), all chunks clamp to the 768 floor:

| Chunk Duration | Padded Samples | encoder_tokens + 128 | ceil_64 | Clamped | Reduction |
|---------------|---------------|---------------------|---------|---------|-----------|
| 0.8s          | 16000         | 178                 | 192     | **768** | 49%       |
| 3s            | 48000         | 278                 | 320     | **768** | 49%       |
| 8s            | 128000        | 528                 | 576     | **768** | 49%       |

The dynamic formula exists to correctly handle future configurations where `max_chunk_ms` exceeds ~12.8s (at which point `encoder_tokens + 128 > 768`).

### Code Changes

#### 1. New helper: `compute_audio_ctx`

```rust
/// Compute the optimal `audio_ctx` value for the given sample count.
///
/// Restricts the encoder's attention window to the actual audio length plus
/// a safety margin, avoiding unnecessary computation on zero-padded silence.
///
/// Uses the model's `model_n_audio_ctx` as the upper bound (queried at runtime)
/// and 768 as the lower bound (upstream-recommended minimum for streaming).
fn compute_audio_ctx(sample_count: usize, max_audio_ctx: i32) -> i32 {
    let encoder_tokens = sample_count.div_ceil(320) as i32;
    let with_margin = encoder_tokens + 128;
    let aligned = (with_margin + 63) & !63; // ceil to multiple of 64
    aligned.clamp(768, max_audio_ctx)
}
```

#### 2. Modified `create_whisper_params`

Signature changes from `(language)` to `(language, sample_count, max_audio_ctx)`:

```rust
fn create_whisper_params<'a>(
    language: &'a str,
    sample_count: usize,
    max_audio_ctx: i32,
) -> FullParams<'a, 'a> {
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_suppress_nst(true);
    params.set_single_segment(true);
    params.set_n_threads(optimal_n_threads());
    params.set_no_context(true);
    params.set_temperature_inc(0.2);
    params.set_audio_ctx(compute_audio_ctx(sample_count, max_audio_ctx));

    if language == "auto" {
        params.set_language(None);
    } else {
        params.set_language(Some(language));
    }

    params
}
```

#### 3. Modified `run_inference`

Accepts `max_audio_ctx` and passes padded sample count to `create_whisper_params`:

```rust
fn run_inference(
    state: &mut WhisperState,
    samples: &[f32],
    language: &str,
    max_audio_ctx: i32,
) -> Result<Option<(String, String)>, InferenceError> {
    let samples = pad_short_chunk(samples);
    let params = create_whisper_params(language, samples.len(), max_audio_ctx);
    state.full(params, &samples).map_err(|e| InferenceError::Full(e.to_string()))?;
    // ... (rest unchanged)
}
```

Note: `pad_short_chunk` is called **before** `create_whisper_params` so that `sample_count` reflects the actual data passed to `whisper_full()`.

#### 4. Modified `inference_loop`

Query `n_audio_ctx` once and pass it through:

```rust
fn inference_loop(ctx: &WhisperContext, ...) {
    let max_audio_ctx = ctx.model_n_audio_ctx();
    // ...
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_inference(&mut state, &envelope.samples, language, max_audio_ctx)
    }));
    // ...
}
```

#### 5. Logging

Compute `audio_ctx` in `inference_loop` for logging, then pass to `run_inference`. This avoids duplicating the computation:

```rust
// In inference_loop, after receiving the envelope:
let audio_ctx = compute_audio_ctx(
    envelope.samples.len().max(MIN_INFERENCE_SAMPLES),
    max_audio_ctx,
);
tracing::info!(
    "Inference: received chunk seq={}, {len} samples ({secs:.1}s), \
     queue_latency={latency_ms}ms, audio_ctx={audio_ctx}",
    envelope.seq
);
```

Note: `envelope.samples.len().max(MIN_INFERENCE_SAMPLES)` mirrors `pad_short_chunk` behavior to compute the correct `audio_ctx` for logging without allocating. The actual `run_inference` call still uses `pad_short_chunk` internally.

#### 6. Unit tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_audio_ctx_clamps_to_minimum() {
        // 0.8s chunk padded to 1s = 16000 samples
        // encoder_tokens = ceil(16000/320) = 50, +128 = 178, ceil_64 = 192
        // clamp(192, 768, 1500) = 768
        assert_eq!(compute_audio_ctx(16000, 1500), 768);
    }

    #[test]
    fn compute_audio_ctx_zero_samples() {
        // Edge case: 0 samples → 0 tokens + 128 = 128, ceil_64 = 128
        // clamp(128, 768, 1500) = 768
        assert_eq!(compute_audio_ctx(0, 1500), 768);
    }

    #[test]
    fn compute_audio_ctx_large_chunk() {
        // 30s = 480000 samples → 1500 tokens + 128 = 1628, ceil_64 = 1664
        // clamp(1664, 768, 1500) = 1500
        assert_eq!(compute_audio_ctx(480000, 1500), 1500);
    }

    #[test]
    fn compute_audio_ctx_above_minimum() {
        // ~15s = 240000 samples → 750 tokens + 128 = 878, ceil_64 = 896
        // clamp(896, 768, 1500) = 896
        assert_eq!(compute_audio_ctx(240000, 1500), 896);
    }

    #[test]
    fn compute_audio_ctx_alignment() {
        // Verify 64-alignment: 800 tokens + 128 = 928, already 64-aligned
        assert_eq!(compute_audio_ctx(256000, 1500), 960);
    }
}
```

## What This Design Does NOT Change

### `temperature_inc` (kept at 0.2)

Whisper's internal fallback retries handle "speech-like but low-quality" results (`avg_logprobs < logprob_thold AND no_speech_prob < no_speech_thold`). The existing `should_discard_low_confidence` filter handles the opposite case ("silence-like low confidence": `avg_logprobs < -1.5 AND no_speech_prob > 0.6`). These are complementary, not redundant. Disabling fallback would remove a quality safety net with no equivalent replacement.

### `pad_short_chunk` (unchanged)

Audio padding to 1s minimum remains necessary. whisper.cpp warns and returns early for inputs < 1000ms. `audio_ctx` controls encoder attention window size, not input length validation.

### Thread count (unchanged)

`optimal_n_threads()` (physical_cores / 2, clamped to 1-8) is kept as-is. Thread tuning affects the entire application and should be evaluated separately.

## Risks and Mitigations

| Risk | Severity | Mitigation |
|------|----------|------------|
| `set_audio_ctx` is marked EXPERIMENTAL in whisper-rs | Medium | Lower bound of 768 follows upstream guidance. Log `audio_ctx` value for debugging. Can revert by removing the single `set_audio_ctx` call. |
| Quality degradation for short chunks at audio_ctx=768 | Low | 768 is the upstream-recommended minimum. Existing confidence filtering provides a safety net. |
| Different behavior across model sizes | Low | Upper bound uses `ctx.model_n_audio_ctx()` queried from the loaded model, adapting automatically. |
| Speedup less than expected | Low | The 49% context reduction is a conservative estimate. Actual improvement depends on hardware and should be verified empirically. |

## Expected Effect

- `audio_ctx`: 1500 → 768 (49% reduction in encoder context)
- Estimated inference time for 0.8s chunk: ~7s → ~3-4s (proportional to encoder context, assuming encoder dominates)
- Empirical verification required to confirm actual speedup

## Verification Plan

1. Build and run with `make debug`
2. Start STT session with large-v3-turbo model
3. Speak several short utterances (0.5-2s) and verify:
   - `audio_ctx=768` appears in inference logs
   - Inference time is reduced compared to baseline (~7s)
   - Recognition quality is not noticeably degraded
4. Speak a longer utterance (~5-8s) and verify same behavior
5. Test with `language: "auto"` to verify language detection still works
