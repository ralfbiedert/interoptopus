#include <gtest/gtest.h>
#include <cmath>
#include <numbers>
#include <string_view>

#include "reference_project.h"

#if defined(_WIN32)
constexpr const char* kLibPath = "reference_project_c.dll";
#elif defined(__APPLE__)
constexpr const char* kLibPath = "libreference_project_c.dylib";
#else
constexpr const char* kLibPath = "libreference_project_c.so";
#endif

constexpr auto kPiF = std::numbers::pi_v<float>;

// ── Helpers ──

static rp_shape make_circle(float r) {
    rp_shape s{};
    s.tag = rp_shape_circle;
    s.circle = r;
    return s;
}

static rp_shape make_rect(float w, float h) {
    rp_shape s{};
    s.tag = rp_shape_rectangle;
    s.rectangle.x = w;
    s.rectangle.y = h;
    return s;
}

static rp_slice_draw_command slice_from(const rp_vec_draw_command& v) {
    return {v.ptr, v.len};
}

static std::string_view to_sv(const rp_string& s) {
    return {reinterpret_cast<const char*>(s.ptr), s.len};
}

// ── Test fixture — loads the Rust cdylib once for all tests ──

class ReferenceProjectC : public ::testing::Test {
public:
    static reference_project_c_api_t api;

protected:
    static void SetUpTestSuite() {
        static bool loaded = false;
        if (!loaded) {
            ASSERT_EQ(reference_project_c_load(kLibPath, &api), 0)
                << "Failed to load library at " << kLibPath;
            loaded = true;
        }
    }
};

reference_project_c_api_t ReferenceProjectC::api{};

// ── Simple enum layout ──

TEST_F(ReferenceProjectC, SimpleEnumLayout) {
    // Color is #[repr(u8)] in Rust. If the C header emitted a plain C `enum`
    // (int-sized), rp_colored_byte.byte_val would be at offset 4 instead of 1 and
    // read garbage.
    rp_colored_byte cb = api.rp_make_colored_byte(rp_color_blue, 42);
    EXPECT_EQ(cb.color, rp_color_blue);
    EXPECT_EQ(cb.byte_val, 42);
}

// ── Basic shapes ──

TEST_F(ReferenceProjectC, CircleArea) {
    EXPECT_FLOAT_EQ(api.rp_shape_area(make_circle(5.0f)), kPiF * 25.0f);
}

TEST_F(ReferenceProjectC, RectangleArea) {
    EXPECT_FLOAT_EQ(api.rp_shape_area(make_rect(3.0f, 4.0f)), 12.0f);
}

// ── Vec + Slice ──

TEST_F(ReferenceProjectC, CreateDefaultCommandsAndTotalArea) {
    auto cmds = api.rp_create_default_commands();
    ASSERT_EQ(cmds.len, 2u);

    EXPECT_FLOAT_EQ(api.rp_total_area(slice_from(cmds)), kPiF * 25.0f + 12.0f);

    api.rp_destroy_draw_commands(cmds);
}

// ── Option ──

TEST_F(ReferenceProjectC, FindLargestPosition) {
    auto cmds = api.rp_create_default_commands();

    auto largest = api.rp_find_largest_position(slice_from(cmds));
    ASSERT_EQ(largest.tag, rp_option_vec2_some);
    EXPECT_FLOAT_EQ(largest.some.x, 0.0f);
    EXPECT_FLOAT_EQ(largest.some.y, 0.0f);

    api.rp_destroy_draw_commands(cmds);
}

// ── Callbacks ──

namespace {

float cb_shape_area(rp_shape shape, [[maybe_unused]] const void* ctx) {
    return ReferenceProjectC::api.rp_shape_area(shape);
}

float cb_slice_total(rp_slice_draw_command cmds, [[maybe_unused]] const void* ctx) {
    return ReferenceProjectC::api.rp_total_area(cmds);
}

float cb_option_magnitude(rp_option_vec2 opt, [[maybe_unused]] const void* ctx) {
    if (opt.tag == rp_option_vec2_some) {
        return opt.some.x * opt.some.x + opt.some.y * opt.some.y;
    }
    return -1.0f;
}

float cb_vec_count(rp_vec_draw_command cmds, [[maybe_unused]] const void* ctx) {
    auto count = static_cast<float>(cmds.len);
    ReferenceProjectC::api.rp_destroy_draw_commands(cmds);
    return count;
}

void cb_kitchen_sink(const rp_kitchen_sink* sink, [[maybe_unused]] const void* ctx) {
    EXPECT_EQ(sink->id, 42u);
    EXPECT_TRUE(sink->enabled);
    EXPECT_DOUBLE_EQ(sink->ratio, std::numbers::pi);
    EXPECT_EQ(to_sv(sink->label), "hello from rust");

    ASSERT_EQ(sink->shape.tag, rp_shape_circle);
    EXPECT_FLOAT_EQ(sink->shape.circle, 7.5f);

    ASSERT_EQ(sink->position.tag, rp_option_vec2_some);
    EXPECT_FLOAT_EQ(sink->position.some.x, 1.0f);
    EXPECT_FLOAT_EQ(sink->position.some.y, 2.0f);

    EXPECT_EQ(sink->tags.len, 2u);

    ASSERT_EQ(sink->name.tag, rp_option_string_some);
    EXPECT_EQ(to_sv(sink->name.some), "kitchen sink");
}

} // namespace

TEST_F(ReferenceProjectC, CallbackShape) {
    rp_shape_callback cb{cb_shape_area, nullptr, nullptr};
    EXPECT_FLOAT_EQ(api.rp_invoke_callback_shape(make_circle(5.0f), cb), kPiF * 25.0f);
}

TEST_F(ReferenceProjectC, CallbackSlice) {
    auto cmds = api.rp_create_default_commands();
    rp_slice_callback cb{cb_slice_total, nullptr, nullptr};
    EXPECT_FLOAT_EQ(api.rp_invoke_callback_slice(slice_from(cmds), cb), kPiF * 25.0f + 12.0f);
    api.rp_destroy_draw_commands(cmds);
}

TEST_F(ReferenceProjectC, CallbackOptionSome) {
    rp_option_vec2 opt{};
    opt.tag = rp_option_vec2_some;
    opt.some = {3.0f, 4.0f};

    rp_option_callback cb{cb_option_magnitude, nullptr, nullptr};
    EXPECT_FLOAT_EQ(api.rp_invoke_callback_option(opt, cb), 25.0f);
}

TEST_F(ReferenceProjectC, CallbackOptionNone) {
    rp_option_vec2 opt{};
    opt.tag = rp_option_vec2_none;

    rp_option_callback cb{cb_option_magnitude, nullptr, nullptr};
    EXPECT_FLOAT_EQ(api.rp_invoke_callback_option(opt, cb), -1.0f);
}

TEST_F(ReferenceProjectC, CallbackVec) {
    rp_vec_callback cb{cb_vec_count, nullptr, nullptr};
    EXPECT_FLOAT_EQ(api.rp_invoke_callback_vec(cb), 2.0f);
}

TEST_F(ReferenceProjectC, CallbackKitchenSink) {
    rp_kitchen_sink_callback cb{cb_kitchen_sink, nullptr, nullptr};
    api.rp_invoke_callback_kitchen_sink(cb);
}
