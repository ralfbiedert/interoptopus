#include <gtest/gtest.h>

#include "hello_world.h"

#if defined(_WIN32)
constexpr const char* kLibPath = "hello_world.dll";
#elif defined(__APPLE__)
constexpr const char* kLibPath = "libhello_world.dylib";
#else
constexpr const char* kLibPath = "libhello_world.so";
#endif

class HelloWorld : public ::testing::Test {
public:
    static hello_world_api_t api;

protected:
    static void SetUpTestSuite() {
        static bool loaded = false;
        if (!loaded) {
            ASSERT_EQ(hello_world_load(kLibPath, &api), 0)
                << "Failed to load library at " << kLibPath;
            loaded = true;
        }
    }
};

hello_world_api_t HelloWorld::api{};

TEST_F(HelloWorld, MyFunctionRoundtrips) {
    VEC2 input{};
    input.x = 1.0f;
    input.y = 2.0f;

    auto output = api.my_function(input);

    EXPECT_FLOAT_EQ(output.x, 1.0f);
    EXPECT_FLOAT_EQ(output.y, 2.0f);
}

TEST_F(HelloWorld, MyFunctionPreservesZero) {
    VEC2 input{};

    auto output = api.my_function(input);

    EXPECT_FLOAT_EQ(output.x, 0.0f);
    EXPECT_FLOAT_EQ(output.y, 0.0f);
}
