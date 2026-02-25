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

static SHAPE make_circle(float r) {
    SHAPE s{};
    s.tag = SHAPE_CIRCLE;
    s.circle = r;
    return s;
}

static SHAPE make_rect(float w, float h) {
    SHAPE s{};
    s.tag = SHAPE_RECTANGLE;
    s.rectangle.x = w;
    s.rectangle.y = h;
    return s;
}

static SLICEDRAWCOMMAND slice_from(const VECDRAWCOMMAND& v) {
    return {v.ptr, v.len};
}

static std::string_view to_sv(const STRING& s) {
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

// ── Basic shapes ──

TEST_F(ReferenceProjectC, CircleArea) {
    EXPECT_FLOAT_EQ(api.shape_area(make_circle(5.0f)), kPiF * 25.0f);
}

TEST_F(ReferenceProjectC, RectangleArea) {
    EXPECT_FLOAT_EQ(api.shape_area(make_rect(3.0f, 4.0f)), 12.0f);
}

// ── Vec + Slice ──

TEST_F(ReferenceProjectC, CreateDefaultCommandsAndTotalArea) {
    auto cmds = api.create_default_commands();
    ASSERT_EQ(cmds.len, 2u);

    EXPECT_FLOAT_EQ(api.total_area(slice_from(cmds)), kPiF * 25.0f + 12.0f);

    api.destroy_draw_commands(cmds);
}

// ── Option ──

TEST_F(ReferenceProjectC, FindLargestPosition) {
    auto cmds = api.create_default_commands();

    auto largest = api.find_largest_position(slice_from(cmds));
    ASSERT_EQ(largest.tag, OPTIONVEC2_SOME);
    EXPECT_FLOAT_EQ(largest.some.x, 0.0f);
    EXPECT_FLOAT_EQ(largest.some.y, 0.0f);

    api.destroy_draw_commands(cmds);
}

// ── Callbacks ──

namespace {

float cb_shape_area(SHAPE shape, [[maybe_unused]] const void* ctx) {
    return ReferenceProjectC::api.shape_area(shape);
}

float cb_slice_total(SLICEDRAWCOMMAND cmds, [[maybe_unused]] const void* ctx) {
    return ReferenceProjectC::api.total_area(cmds);
}

float cb_option_magnitude(OPTIONVEC2 opt, [[maybe_unused]] const void* ctx) {
    if (opt.tag == OPTIONVEC2_SOME) {
        return opt.some.x * opt.some.x + opt.some.y * opt.some.y;
    }
    return -1.0f;
}

float cb_vec_count(VECDRAWCOMMAND cmds, [[maybe_unused]] const void* ctx) {
    auto count = static_cast<float>(cmds.len);
    ReferenceProjectC::api.destroy_draw_commands(cmds);
    return count;
}

void cb_kitchen_sink(const KITCHENSINK* sink, [[maybe_unused]] const void* ctx) {
    EXPECT_EQ(sink->id, 42u);
    EXPECT_TRUE(sink->enabled);
    EXPECT_DOUBLE_EQ(sink->ratio, std::numbers::pi);
    EXPECT_EQ(to_sv(sink->label), "hello from rust");

    ASSERT_EQ(sink->shape.tag, SHAPE_CIRCLE);
    EXPECT_FLOAT_EQ(sink->shape.circle, 7.5f);

    ASSERT_EQ(sink->position.tag, OPTIONVEC2_SOME);
    EXPECT_FLOAT_EQ(sink->position.some.x, 1.0f);
    EXPECT_FLOAT_EQ(sink->position.some.y, 2.0f);

    EXPECT_EQ(sink->tags.len, 2u);

    ASSERT_EQ(sink->name.tag, OPTIONSTRING_SOME);
    EXPECT_EQ(to_sv(sink->name.some), "kitchen sink");
}

} // namespace

TEST_F(ReferenceProjectC, CallbackShape) {
    SHAPECALLBACK cb{cb_shape_area, nullptr, nullptr};
    EXPECT_FLOAT_EQ(api.invoke_callback_shape(make_circle(5.0f), cb), kPiF * 25.0f);
}

TEST_F(ReferenceProjectC, CallbackSlice) {
    auto cmds = api.create_default_commands();
    SLICECALLBACK cb{cb_slice_total, nullptr, nullptr};
    EXPECT_FLOAT_EQ(api.invoke_callback_slice(slice_from(cmds), cb), kPiF * 25.0f + 12.0f);
    api.destroy_draw_commands(cmds);
}

TEST_F(ReferenceProjectC, CallbackOptionSome) {
    OPTIONVEC2 opt{};
    opt.tag = OPTIONVEC2_SOME;
    opt.some = {3.0f, 4.0f};

    OPTIONCALLBACK cb{cb_option_magnitude, nullptr, nullptr};
    EXPECT_FLOAT_EQ(api.invoke_callback_option(opt, cb), 25.0f);
}

TEST_F(ReferenceProjectC, CallbackOptionNone) {
    OPTIONVEC2 opt{};
    opt.tag = OPTIONVEC2_NONE;

    OPTIONCALLBACK cb{cb_option_magnitude, nullptr, nullptr};
    EXPECT_FLOAT_EQ(api.invoke_callback_option(opt, cb), -1.0f);
}

TEST_F(ReferenceProjectC, CallbackVec) {
    VECCALLBACK cb{cb_vec_count, nullptr, nullptr};
    EXPECT_FLOAT_EQ(api.invoke_callback_vec(cb), 2.0f);
}

TEST_F(ReferenceProjectC, CallbackKitchenSink) {
    KITCHENSINKCALLBACK cb{cb_kitchen_sink, nullptr, nullptr};
    api.invoke_callback_kitchen_sink(cb);
}
