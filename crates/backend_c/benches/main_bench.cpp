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

static SHAPE make_circle(float r) {
    SHAPE s{};
    s.tag = SHAPE_CIRCLE;
    s.circle = r;
    return s;
}

static SLICEDRAWCOMMAND slice_from(const VECDRAWCOMMAND& v) {
    return {v.ptr, v.len};
}

// ── Callback helpers ──

static float cb_tagged_union_passthrough(SHAPE shape, const void*) {
    return api.shape_area(shape);
}

static float cb_slice_passthrough(SLICEDRAWCOMMAND cmds, const void*) {
    return api.total_area(cmds);
}

static float cb_option_passthrough(OPTIONVEC2 opt, const void*) {
    if (opt.tag == OPTIONVEC2_SOME) {
        return opt.some.x * opt.some.x + opt.some.y * opt.some.y;
    }
    return -1.0f;
}

// ── Direct FFI call benchmarks ──

static void BM_TaggedUnion(benchmark::State& state) {
    load_api();
    auto shape = make_circle(5.0f);
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.shape_area(shape));
    }
}
BENCHMARK(BM_TaggedUnion);

static void BM_SliceOfStruct(benchmark::State& state) {
    load_api();
    auto cmds = api.create_default_commands();
    auto slice = slice_from(cmds);
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.total_area(slice));
    }
    api.destroy_draw_commands(cmds);
}
BENCHMARK(BM_SliceOfStruct);

static void BM_SliceMutOfStruct(benchmark::State& state) {
    load_api();
    auto cmds = api.create_default_commands();
    SLICEMUTDRAWCOMMAND slice_mut{cmds.ptr, cmds.len};
    for (auto _ : state) {
        api.scale_commands(slice_mut, 1.0f);
    }
    api.destroy_draw_commands(cmds);
}
BENCHMARK(BM_SliceMutOfStruct);

static void BM_VecCreateDestroy(benchmark::State& state) {
    load_api();
    for (auto _ : state) {
        auto cmds = api.create_default_commands();
        api.destroy_draw_commands(cmds);
    }
}
BENCHMARK(BM_VecCreateDestroy);

static void BM_OptionReturn(benchmark::State& state) {
    load_api();
    auto cmds = api.create_default_commands();
    auto slice = slice_from(cmds);
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.find_largest_position(slice));
    }
    api.destroy_draw_commands(cmds);
}
BENCHMARK(BM_OptionReturn);

// ── Callback benchmarks ──

static void BM_Callback_TaggedUnion(benchmark::State& state) {
    load_api();
    auto shape = make_circle(5.0f);
    SHAPECALLBACK cb{cb_tagged_union_passthrough, nullptr, nullptr};
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.invoke_callback_shape(shape, cb));
    }
}
BENCHMARK(BM_Callback_TaggedUnion);

static void BM_Callback_Slice(benchmark::State& state) {
    load_api();
    auto cmds = api.create_default_commands();
    auto slice = slice_from(cmds);
    SLICECALLBACK cb{cb_slice_passthrough, nullptr, nullptr};
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.invoke_callback_slice(slice, cb));
    }
    api.destroy_draw_commands(cmds);
}
BENCHMARK(BM_Callback_Slice);

static void BM_Callback_Option(benchmark::State& state) {
    load_api();
    OPTIONVEC2 opt{};
    opt.tag = OPTIONVEC2_SOME;
    opt.some = {3.0f, 4.0f};
    OPTIONCALLBACK cb{cb_option_passthrough, nullptr, nullptr};
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.invoke_callback_option(opt, cb));
    }
}
BENCHMARK(BM_Callback_Option);

static void BM_Callback_VecRoundtrip(benchmark::State& state) {
    load_api();
    // The Rust side creates and returns a Vec each call, so this
    // measures the full round-trip including allocation.
    auto cb_vec_count = [](VECDRAWCOMMAND cmds, const void*) -> float {
        auto count = static_cast<float>(cmds.len);
        api.destroy_draw_commands(cmds);
        return count;
    };
    VECCALLBACK cb{cb_vec_count, nullptr, nullptr};
    for (auto _ : state) {
        benchmark::DoNotOptimize(api.invoke_callback_vec(cb));
    }
}
BENCHMARK(BM_Callback_VecRoundtrip);

static void BM_Callback_ComplexStructByRef(benchmark::State& state) {
    load_api();
    auto cb_noop = [](const KITCHENSINK*, const void*) {};
    KITCHENSINKCALLBACK cb{cb_noop, nullptr, nullptr};
    for (auto _ : state) {
        api.invoke_callback_kitchen_sink(cb);
    }
}
BENCHMARK(BM_Callback_ComplexStructByRef);
