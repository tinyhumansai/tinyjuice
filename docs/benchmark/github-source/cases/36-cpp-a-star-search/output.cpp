/**
 * @brief
 * [A* search algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
 * @details
 * A* is an informed search algorithm, or a best-first search, meaning that it
 * is formulated in terms of weighted graphs: starting from a specific starting
 * node of a graph (initial state), it aims to find a path to the given goal
 * node having the smallest cost (least distance travelled, shortest time,
 * etc.). It evaluates by maintaining a tree of paths originating at the start
 * node and extending those paths one edge at a time until it reaches the final
 * state.
 * The weighted edges (or cost) is evaluated on two factors, G score
 * (cost required from starting node or initial state to current state) and H
 * score (cost required from current state to final state). The F(state), then
 * is evaluated as:
 * F(state) = G(state) + H(state).
 *
 * To solve the given search with shortest cost or path possible  is to inspect
 * values having minimum F(state).
 * @author [Ashish Daulatabad](https://github.com/AshishYUO)
 */
#include <algorithm>   /// for `std::reverse` function
#include <array>       /// for `std::array`, representing `EightPuzzle` board
#include <cassert>     /// for `assert`
#include <cstdint>     /// for `std::uint32_t`
#include <functional>  /// for `std::function` STL
#include <iostream>    /// for IO operations
#include <map>         /// for `std::map` STL
#include <memory>      /// for `std::shared_ptr`
#include <set>         /// for `std::set` STL
#include <vector>      /// for `std::vector` STL

/**
 * @namespace machine_learning
 * @brief Machine learning algorithms
 */
namespace machine_learning {
/**
 * @namespace aystar_search
 * @brief Functions for [A*
 * Search](https://en.wikipedia.org/wiki/A*_search_algorithm) implementation.
 */
namespace aystar_search {
/**
 * @class EightPuzzle
 * @brief A class defining [EightPuzzle/15-Puzzle
 * game](https://en.wikipedia.org/wiki/15_puzzle).
 * @details
 * A well known 3 x 3 puzzle of the form
 * `
 * 1   2   3
 * 4   5   6
 * 7   8   0
 * `
 * where `0` represents an empty space in the puzzle
 * Given any random state, the goal is to achieve the above configuration
 * (or any other configuration if possible)
 * @tparam N size of the square Puzzle, default is set to 3 (since it is
 * EightPuzzle)
 */
template <size_t N = 3>
class EightPuzzle {
    { … 200 line(s) … ⟦tj:8a40bde4c263df3f153f6fdce0f4080a⟧ }
/**
 * @class AyStarSearch
 * @brief A class defining [A* search
 * algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm). for some
 * initial state and final state
 * @details AyStarSearch class is defined as the informed search algorithm
 * that is formulated in terms of weighted graphs: starting from a specific
 * starting node of a graph (initial state), it aims to find a path to the given
 * goal node having the smallest cost (least distance travelled, shortest time,
 * etc.)
 * The weighted edges (or cost) is evaluated on two factors, G score
 * (cost required from starting node or initial state to current state) and H
 * score (cost required from current state to final state). The `F(state)`, then
 * is evaluated as:
 * `F(state) = G(state) + H(state)`.
 * The best search would be the final state having minimum `F(state)` value
 * @tparam Puzzle denotes the puzzle or problem involving initial state and
 * final state to be solved by A* search.
 * @note 1. The algorithm is referred from pesudocode from
 * [Wikipedia page](https://en.wikipedia.org/wiki/A*_search_algorithm)
 * as is.
 * 2. For `AyStarSearch` to work, the definitions for template Puzzle is
 * compulsory.
 * a. Comparison operator for template Puzzle (`<`, `==`, and `<=`)
 * b. `generate_possible_moves()`
 */
template <typename Puzzle>
class AyStarSearch {
    { … 238 line(s) … ⟦tj:df5f1d7d54cab58ceb599689e4c9370e⟧ }
}  // namespace aystar_search
}  // namespace machine_learning

/**
 * @brief Self test-implementations
 * @returns void
 */
static void test() {
    { … 168 line(s) … ⟦tj:0ef5028e6fc9817f75a81b8785abfdea⟧ }
/**
 * @brief Main function
 * @returns 0 on exit
 */
int main() {
    test();  // run self-test implementations
    return 0;
}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (26515 bytes) is available by calling tinyjuice_retrieve with token "5c3a822be0fb6047961d44d64736832e" (marker ⟦tj:5c3a822be0fb6047961d44d64736832e⟧)]