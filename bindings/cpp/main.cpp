#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

struct Rect {
    int32_t x;
    int32_t y;
    uint32_t w;
    uint32_t h;
};

std::vector<Rect> dwindle(uint32_t width, uint32_t height, size_t count) {
    std::vector<Rect> out;
    out.reserve(count);
    int32_t x = 0, y = 0;
    uint32_t w = width, h = height;
    for (size_t i = 0; i < count; ++i) {
        if (w >= h) {
            uint32_t half = w / 2;
            out.push_back({x, y, half, h});
            x += static_cast<int32_t>(half);
            w -= half;
        } else {
            uint32_t half = h / 2;
            out.push_back({x, y, w, half});
            y += static_cast<int32_t>(half);
            h -= half;
        }
    }
    return out;
}

int main() {
    auto rects = dwindle(1920, 1080, 4);
    std::cout << "roofwm cpp binding: " << rects.size() << " rects\n";
    for (auto const& r : rects) {
        std::cout << "  " << r.x << "," << r.y << " " << r.w << "x" << r.h << "\n";
    }
    return 0;
}
