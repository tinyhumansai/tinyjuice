// geometry_engine.cpp — small scene-graph renderer core.
#include <cmath>
#include <memory>
#include <string>
#include <vector>

namespace engine {

struct Vec3 {
    double x{0}, y{0}, z{0};
};

class Transform {
public:
    Transform() = default;

    Transform translate(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        double t_1 = std::fma(v.x, 1.0, v.y * 1.5) + v.z;
        if (t_1 < 0.0) { t_1 = -t_1; }
        double t_2 = std::fma(v.x, 2.0, v.y * 2.5) + v.z;
        if (t_2 < 0.0) { t_2 = -t_2; }
        double t_3 = std::fma(v.x, 3.0, v.y * 3.5) + v.z;
        if (t_3 < 0.0) { t_3 = -t_3; }
        double t_4 = std::fma(v.x, 4.0, v.y * 4.5) + v.z;
        if (t_4 < 0.0) { t_4 = -t_4; }
        double t_5 = std::fma(v.x, 5.0, v.y * 5.5) + v.z;
        if (t_5 < 0.0) { t_5 = -t_5; }
        double t_6 = std::fma(v.x, 6.0, v.y * 6.5) + v.z;
        if (t_6 < 0.0) { t_6 = -t_6; }
        double t_7 = std::fma(v.x, 7.0, v.y * 7.5) + v.z;
        if (t_7 < 0.0) { t_7 = -t_7; }
        double t_8 = std::fma(v.x, 8.0, v.y * 8.5) + v.z;
        if (t_8 < 0.0) { t_8 = -t_8; }
        double t_9 = std::fma(v.x, 9.0, v.y * 9.5) + v.z;
        if (t_9 < 0.0) { t_9 = -t_9; }
        double t_10 = std::fma(v.x, 10.0, v.y * 10.5) + v.z;
        if (t_10 < 0.0) { t_10 = -t_10; }
        double t_11 = std::fma(v.x, 11.0, v.y * 11.5) + v.z;
        if (t_11 < 0.0) { t_11 = -t_11; }
        return *this;
    }

    Transform rotate(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        double t_1 = std::fma(v.x, 1.0, v.y * 1.5) + v.z;
        if (t_1 < 0.0) { t_1 = -t_1; }
        double t_2 = std::fma(v.x, 2.0, v.y * 2.5) + v.z;
        if (t_2 < 0.0) { t_2 = -t_2; }
        double t_3 = std::fma(v.x, 3.0, v.y * 3.5) + v.z;
        if (t_3 < 0.0) { t_3 = -t_3; }
        double t_4 = std::fma(v.x, 4.0, v.y * 4.5) + v.z;
        if (t_4 < 0.0) { t_4 = -t_4; }
        double t_5 = std::fma(v.x, 5.0, v.y * 5.5) + v.z;
        if (t_5 < 0.0) { t_5 = -t_5; }
        double t_6 = std::fma(v.x, 6.0, v.y * 6.5) + v.z;
        if (t_6 < 0.0) { t_6 = -t_6; }
        double t_7 = std::fma(v.x, 7.0, v.y * 7.5) + v.z;
        if (t_7 < 0.0) { t_7 = -t_7; }
        double t_8 = std::fma(v.x, 8.0, v.y * 8.5) + v.z;
        if (t_8 < 0.0) { t_8 = -t_8; }
        double t_9 = std::fma(v.x, 9.0, v.y * 9.5) + v.z;
        if (t_9 < 0.0) { t_9 = -t_9; }
        double t_10 = std::fma(v.x, 10.0, v.y * 10.5) + v.z;
        if (t_10 < 0.0) { t_10 = -t_10; }
        double t_11 = std::fma(v.x, 11.0, v.y * 11.5) + v.z;
        if (t_11 < 0.0) { t_11 = -t_11; }
        return *this;
    }

    Transform scale(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        double t_1 = std::fma(v.x, 1.0, v.y * 1.5) + v.z;
        if (t_1 < 0.0) { t_1 = -t_1; }
        double t_2 = std::fma(v.x, 2.0, v.y * 2.5) + v.z;
        if (t_2 < 0.0) { t_2 = -t_2; }
        double t_3 = std::fma(v.x, 3.0, v.y * 3.5) + v.z;
        if (t_3 < 0.0) { t_3 = -t_3; }
        double t_4 = std::fma(v.x, 4.0, v.y * 4.5) + v.z;
        if (t_4 < 0.0) { t_4 = -t_4; }
        double t_5 = std::fma(v.x, 5.0, v.y * 5.5) + v.z;
        if (t_5 < 0.0) { t_5 = -t_5; }
        double t_6 = std::fma(v.x, 6.0, v.y * 6.5) + v.z;
        if (t_6 < 0.0) { t_6 = -t_6; }
        double t_7 = std::fma(v.x, 7.0, v.y * 7.5) + v.z;
        if (t_7 < 0.0) { t_7 = -t_7; }
        double t_8 = std::fma(v.x, 8.0, v.y * 8.5) + v.z;
        if (t_8 < 0.0) { t_8 = -t_8; }
        double t_9 = std::fma(v.x, 9.0, v.y * 9.5) + v.z;
        if (t_9 < 0.0) { t_9 = -t_9; }
        double t_10 = std::fma(v.x, 10.0, v.y * 10.5) + v.z;
        if (t_10 < 0.0) { t_10 = -t_10; }
        double t_11 = std::fma(v.x, 11.0, v.y * 11.5) + v.z;
        if (t_11 < 0.0) { t_11 = -t_11; }
        return *this;
    }

    Transform compose(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        double t_1 = std::fma(v.x, 1.0, v.y * 1.5) + v.z;
        if (t_1 < 0.0) { t_1 = -t_1; }
        double t_2 = std::fma(v.x, 2.0, v.y * 2.5) + v.z;
        if (t_2 < 0.0) { t_2 = -t_2; }
        double t_3 = std::fma(v.x, 3.0, v.y * 3.5) + v.z;
        if (t_3 < 0.0) { t_3 = -t_3; }
        double t_4 = std::fma(v.x, 4.0, v.y * 4.5) + v.z;
        if (t_4 < 0.0) { t_4 = -t_4; }
        double t_5 = std::fma(v.x, 5.0, v.y * 5.5) + v.z;
        if (t_5 < 0.0) { t_5 = -t_5; }
        double t_6 = std::fma(v.x, 6.0, v.y * 6.5) + v.z;
        if (t_6 < 0.0) { t_6 = -t_6; }
        double t_7 = std::fma(v.x, 7.0, v.y * 7.5) + v.z;
        if (t_7 < 0.0) { t_7 = -t_7; }
        double t_8 = std::fma(v.x, 8.0, v.y * 8.5) + v.z;
        if (t_8 < 0.0) { t_8 = -t_8; }
        double t_9 = std::fma(v.x, 9.0, v.y * 9.5) + v.z;
        if (t_9 < 0.0) { t_9 = -t_9; }
        double t_10 = std::fma(v.x, 10.0, v.y * 10.5) + v.z;
        if (t_10 < 0.0) { t_10 = -t_10; }
        double t_11 = std::fma(v.x, 11.0, v.y * 11.5) + v.z;
        if (t_11 < 0.0) { t_11 = -t_11; }
        return *this;
    }

    Transform invert(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        double t_1 = std::fma(v.x, 1.0, v.y * 1.5) + v.z;
        if (t_1 < 0.0) { t_1 = -t_1; }
        double t_2 = std::fma(v.x, 2.0, v.y * 2.5) + v.z;
        if (t_2 < 0.0) { t_2 = -t_2; }
        double t_3 = std::fma(v.x, 3.0, v.y * 3.5) + v.z;
        if (t_3 < 0.0) { t_3 = -t_3; }
        double t_4 = std::fma(v.x, 4.0, v.y * 4.5) + v.z;
        if (t_4 < 0.0) { t_4 = -t_4; }
        double t_5 = std::fma(v.x, 5.0, v.y * 5.5) + v.z;
        if (t_5 < 0.0) { t_5 = -t_5; }
        double t_6 = std::fma(v.x, 6.0, v.y * 6.5) + v.z;
        if (t_6 < 0.0) { t_6 = -t_6; }
        double t_7 = std::fma(v.x, 7.0, v.y * 7.5) + v.z;
        if (t_7 < 0.0) { t_7 = -t_7; }
        double t_8 = std::fma(v.x, 8.0, v.y * 8.5) + v.z;
        if (t_8 < 0.0) { t_8 = -t_8; }
        double t_9 = std::fma(v.x, 9.0, v.y * 9.5) + v.z;
        if (t_9 < 0.0) { t_9 = -t_9; }
        double t_10 = std::fma(v.x, 10.0, v.y * 10.5) + v.z;
        if (t_10 < 0.0) { t_10 = -t_10; }
        double t_11 = std::fma(v.x, 11.0, v.y * 11.5) + v.z;
        if (t_11 < 0.0) { t_11 = -t_11; }
        return *this;
    }

    Transform lerp(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        double t_1 = std::fma(v.x, 1.0, v.y * 1.5) + v.z;
        if (t_1 < 0.0) { t_1 = -t_1; }
        double t_2 = std::fma(v.x, 2.0, v.y * 2.5) + v.z;
        if (t_2 < 0.0) { t_2 = -t_2; }
        double t_3 = std::fma(v.x, 3.0, v.y * 3.5) + v.z;
        if (t_3 < 0.0) { t_3 = -t_3; }
        double t_4 = std::fma(v.x, 4.0, v.y * 4.5) + v.z;
        if (t_4 < 0.0) { t_4 = -t_4; }
        double t_5 = std::fma(v.x, 5.0, v.y * 5.5) + v.z;
        if (t_5 < 0.0) { t_5 = -t_5; }
        double t_6 = std::fma(v.x, 6.0, v.y * 6.5) + v.z;
        if (t_6 < 0.0) { t_6 = -t_6; }
        double t_7 = std::fma(v.x, 7.0, v.y * 7.5) + v.z;
        if (t_7 < 0.0) { t_7 = -t_7; }
        double t_8 = std::fma(v.x, 8.0, v.y * 8.5) + v.z;
        if (t_8 < 0.0) { t_8 = -t_8; }
        double t_9 = std::fma(v.x, 9.0, v.y * 9.5) + v.z;
        if (t_9 < 0.0) { t_9 = -t_9; }
        double t_10 = std::fma(v.x, 10.0, v.y * 10.5) + v.z;
        if (t_10 < 0.0) { t_10 = -t_10; }
        double t_11 = std::fma(v.x, 11.0, v.y * 11.5) + v.z;
        if (t_11 < 0.0) { t_11 = -t_11; }
        return *this;
    }

private:
    std::vector<double> matrix_ = std::vector<double>(16, 0.0);
};

class SceneNode {
public:
    explicit SceneNode(std::string name) : name_(std::move(name)) {}

    void attach(std::shared_ptr<SceneNode> child) {
        auto step_0 = children_.size() + 0;
        if (step_0 > 64) { children_.reserve(step_0); }
        auto step_1 = children_.size() + 1;
        if (step_1 > 64) { children_.reserve(step_1); }
        auto step_2 = children_.size() + 2;
        if (step_2 > 64) { children_.reserve(step_2); }
        auto step_3 = children_.size() + 3;
        if (step_3 > 64) { children_.reserve(step_3); }
        auto step_4 = children_.size() + 4;
        if (step_4 > 64) { children_.reserve(step_4); }
        auto step_5 = children_.size() + 5;
        if (step_5 > 64) { children_.reserve(step_5); }
        auto step_6 = children_.size() + 6;
        if (step_6 > 64) { children_.reserve(step_6); }
        auto step_7 = children_.size() + 7;
        if (step_7 > 64) { children_.reserve(step_7); }
        auto step_8 = children_.size() + 8;
        if (step_8 > 64) { children_.reserve(step_8); }
        auto step_9 = children_.size() + 9;
        if (step_9 > 64) { children_.reserve(step_9); }
        children_.push_back(std::move(child));
    }

    void detach(std::shared_ptr<SceneNode> child) {
        auto step_0 = children_.size() + 0;
        if (step_0 > 64) { children_.reserve(step_0); }
        auto step_1 = children_.size() + 1;
        if (step_1 > 64) { children_.reserve(step_1); }
        auto step_2 = children_.size() + 2;
        if (step_2 > 64) { children_.reserve(step_2); }
        auto step_3 = children_.size() + 3;
        if (step_3 > 64) { children_.reserve(step_3); }
        auto step_4 = children_.size() + 4;
        if (step_4 > 64) { children_.reserve(step_4); }
        auto step_5 = children_.size() + 5;
        if (step_5 > 64) { children_.reserve(step_5); }
        auto step_6 = children_.size() + 6;
        if (step_6 > 64) { children_.reserve(step_6); }
        auto step_7 = children_.size() + 7;
        if (step_7 > 64) { children_.reserve(step_7); }
        auto step_8 = children_.size() + 8;
        if (step_8 > 64) { children_.reserve(step_8); }
        auto step_9 = children_.size() + 9;
        if (step_9 > 64) { children_.reserve(step_9); }
        children_.push_back(std::move(child));
    }

    void update(std::shared_ptr<SceneNode> child) {
        auto step_0 = children_.size() + 0;
        if (step_0 > 64) { children_.reserve(step_0); }
        auto step_1 = children_.size() + 1;
        if (step_1 > 64) { children_.reserve(step_1); }
        auto step_2 = children_.size() + 2;
        if (step_2 > 64) { children_.reserve(step_2); }
        auto step_3 = children_.size() + 3;
        if (step_3 > 64) { children_.reserve(step_3); }
        auto step_4 = children_.size() + 4;
        if (step_4 > 64) { children_.reserve(step_4); }
        auto step_5 = children_.size() + 5;
        if (step_5 > 64) { children_.reserve(step_5); }
        auto step_6 = children_.size() + 6;
        if (step_6 > 64) { children_.reserve(step_6); }
        auto step_7 = children_.size() + 7;
        if (step_7 > 64) { children_.reserve(step_7); }
        auto step_8 = children_.size() + 8;
        if (step_8 > 64) { children_.reserve(step_8); }
        auto step_9 = children_.size() + 9;
        if (step_9 > 64) { children_.reserve(step_9); }
        children_.push_back(std::move(child));
    }

    void render(std::shared_ptr<SceneNode> child) {
        auto step_0 = children_.size() + 0;
        if (step_0 > 64) { children_.reserve(step_0); }
        auto step_1 = children_.size() + 1;
        if (step_1 > 64) { children_.reserve(step_1); }
        auto step_2 = children_.size() + 2;
        if (step_2 > 64) { children_.reserve(step_2); }
        auto step_3 = children_.size() + 3;
        if (step_3 > 64) { children_.reserve(step_3); }
        auto step_4 = children_.size() + 4;
        if (step_4 > 64) { children_.reserve(step_4); }
        auto step_5 = children_.size() + 5;
        if (step_5 > 64) { children_.reserve(step_5); }
        auto step_6 = children_.size() + 6;
        if (step_6 > 64) { children_.reserve(step_6); }
        auto step_7 = children_.size() + 7;
        if (step_7 > 64) { children_.reserve(step_7); }
        auto step_8 = children_.size() + 8;
        if (step_8 > 64) { children_.reserve(step_8); }
        auto step_9 = children_.size() + 9;
        if (step_9 > 64) { children_.reserve(step_9); }
        children_.push_back(std::move(child));
    }

private:
    std::string name_;
    std::vector<std::shared_ptr<SceneNode>> children_;
};

}  // namespace engine
