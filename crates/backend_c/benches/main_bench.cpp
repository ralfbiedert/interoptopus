#include <benchmark/benchmark.h>
#include <cmath>
#include <numbers>

#include "reference_project.h"

#if defined(_WIN32)
constexpr const char* kLibPath = "reference_project_c.dll";
#elif defined(__APPLE__)
constexpr const char* kLibPath = "libreference_project_c.dylib";
#else
constexpr const char* kLibPath = "libreference_project_c.so";
#endif

// ── Global API handle, loaded once ──

static reference_project_c_api_t api{};

static void load_api() {
    static bool loaded = false;
    if (!loaded) {
        if (reference_project_c_load(kLibPath, &api) != 0) {
            fprintf(stderr, "FATAL: failed to load library at %s\n", kLibPath);
            exit(1);
        }
        loaded = true;
    }
}

// ── Helpers ──

static rp_shape make_circle(float r) {
    rp_shape s{};
    s.tag = rp_shape_circle;
    s.circle = r;
    return s;
}

static rp_slice_draw_command slice_from(const rp_vec_draw_command& v) {
    return {v.ptr, v.len};
}

// ── Callback helpers ──

static float cb_tagged_union_passthrough(rp_shape shape, const void*) {
    return api.rp_shape_area(shape);
}

static float cb_slice_passthrough(rp_slice_draw_command cmds, const void*) {
    return api.rp_total_area(cmds);
}

static float cb_option_passthrough(rp_option_vec2 opt, const void*) {
    if (opt.tag == rp_option_vec2_some) {
        return opt.some.x * opt.some.x + opt.some.y * opt.some.y;
    }
    return -1.0f;
}

// ── Direct FFI call benchmarks ──

static void BM_TaggedUnion(benchmark::State& state) {
    load_api();
    auto shape = make_circle(5.0f);
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.rp_shape_area(shape));
    }
}
BENCHMARK(BM_TaggedUnion);

static void BM_SliceOfStruct(benchmark::State& state) {
    load_api();
    auto cmds = api.rp_create_default_commands();
    auto slice = slice_from(cmds);
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.rp_total_area(slice));
    }
    api.rp_destroy_draw_commands(cmds);
}
BENCHMARK(BM_SliceOfStruct);

static void BM_SliceMutOfStruct(benchmark::State& state) {
    load_api();
    auto cmds = api.rp_create_default_commands();
    rp_slice_mut_draw_command slice_mut{cmds.ptr, cmds.len};
    for (auto _ : state) {
        api.rp_scale_commands(slice_mut, 1.0f);
    }
    api.rp_destroy_draw_commands(cmds);
}
BENCHMARK(BM_SliceMutOfStruct);

static void BM_VecCreateDestroy(benchmark::State& state) {
    load_api();
    for (auto _ : state) {
        auto cmds = api.rp_create_default_commands();
        api.rp_destroy_draw_commands(cmds);
    }
}
BENCHMARK(BM_VecCreateDestroy);

static void BM_OptionReturn(benchmark::State& state) {
    load_api();
    auto cmds = api.rp_create_default_commands();
    auto slice = slice_from(cmds);
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.rp_find_largest_position(slice));
    }
    api.rp_destroy_draw_commands(cmds);
}
BENCHMARK(BM_OptionReturn);

// ── Callback benchmarks ──

static void BM_Callback_TaggedUnion(benchmark::State& state) {
    load_api();
    auto shape = make_circle(5.0f);
    rp_shape_callback cb{cb_tagged_union_passthrough, nullptr, nullptr};
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.rp_invoke_callback_shape(shape, cb));
    }
}
BENCHMARK(BM_Callback_TaggedUnion);

static void BM_Callback_Slice(benchmark::State& state) {
    load_api();
    auto cmds = api.rp_create_default_commands();
    auto slice = slice_from(cmds);
    rp_slice_callback cb{cb_slice_passthrough, nullptr, nullptr};
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.rp_invoke_callback_slice(slice, cb));
    }
    api.rp_destroy_draw_commands(cmds);
}
BENCHMARK(BM_Callback_Slice);

static void BM_Callback_Option(benchmark::State& state) {
    load_api();
    rp_option_vec2 opt{};
    opt.tag = rp_option_vec2_some;
    opt.some = {3.0f, 4.0f};
    rp_option_callback cb{cb_option_passthrough, nullptr, nullptr};
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.rp_invoke_callback_option(opt, cb));
    }
}
BENCHMARK(BM_Callback_Option);

static void BM_Callback_VecRoundtrip(benchmark::State& state) {
    load_api();
    // The Rust side creates and returns a Vec each call, so this
    // measures the full round-trip including allocation.
    auto cb_vec_count = [](rp_vec_draw_command cmds, const void*) -> float {
        auto count = static_cast<float>(cmds.len);
        api.rp_destroy_draw_commands(cmds);
        return count;
    };
    rp_vec_callback cb{cb_vec_count, nullptr, nullptr};
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.rp_invoke_callback_vec(cb));
    }
}
BENCHMARK(BM_Callback_VecRoundtrip);

static void BM_Callback_ComplexStructByRef(benchmark::State& state) {
    load_api();
    auto cb_noop = [](const rp_kitchen_sink*, const void*) {};
    rp_kitchen_sink_callback cb{cb_noop, nullptr, nullptr};
    for (auto _ : state) {
        api.rp_invoke_callback_kitchen_sink(cb);
    }
}
BENCHMARK(BM_Callback_ComplexStructByRef);
