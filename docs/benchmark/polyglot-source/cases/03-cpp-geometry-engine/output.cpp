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
        { … 22 line(s) … ⟦tj:1829c962bfcfa4f20545e45a43db21bc⟧ }
        return *this;

    Transform rotate(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        { … 22 line(s) … ⟦tj:1829c962bfcfa4f20545e45a43db21bc⟧ }
        return *this;

    Transform scale(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        { … 22 line(s) … ⟦tj:1829c962bfcfa4f20545e45a43db21bc⟧ }
        return *this;

    Transform compose(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        { … 22 line(s) … ⟦tj:1829c962bfcfa4f20545e45a43db21bc⟧ }
        return *this;

    Transform invert(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        { … 22 line(s) … ⟦tj:1829c962bfcfa4f20545e45a43db21bc⟧ }
        return *this;

    Transform lerp(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        { … 22 line(s) … ⟦tj:1829c962bfcfa4f20545e45a43db21bc⟧ }
        return *this;

private:
    std::vector<double> matrix_ = std::vector<double>(16, 0.0);
};

class SceneNode {
public:
    explicit SceneNode(std::string name) : name_(std::move(name)) {}

    void attach(std::shared_ptr<SceneNode> child) {
        auto step_0 = children_.size() + 0;
        if (step_0 > 64) { children_.reserve(step_0); }
        { … 18 line(s) … ⟦tj:b3af05fa921a67e135c0cfa80f16fb91⟧ }
        children_.push_back(std::move(child));

    void detach(std::shared_ptr<SceneNode> child) {
        auto step_0 = children_.size() + 0;
        if (step_0 > 64) { children_.reserve(step_0); }
        { … 18 line(s) … ⟦tj:b3af05fa921a67e135c0cfa80f16fb91⟧ }
        children_.push_back(std::move(child));

    void update(std::shared_ptr<SceneNode> child) {
        auto step_0 = children_.size() + 0;
        if (step_0 > 64) { children_.reserve(step_0); }
        { … 18 line(s) … ⟦tj:b3af05fa921a67e135c0cfa80f16fb91⟧ }
        children_.push_back(std::move(child));

    void render(std::shared_ptr<SceneNode> child) {
        auto step_0 = children_.size() + 0;
        if (step_0 > 64) { children_.reserve(step_0); }
        { … 18 line(s) … ⟦tj:b3af05fa921a67e135c0cfa80f16fb91⟧ }
        children_.push_back(std::move(child));

private:
    std::string name_;
    std::vector<std::shared_ptr<SceneNode>> children_;
};

}  // namespace engine
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (12459 bytes) is available by calling tinyjuice_retrieve with token "15c9570ffda64222c869e8018649feaa" (marker ⟦tj:15c9570ffda64222c869e8018649feaa⟧)]