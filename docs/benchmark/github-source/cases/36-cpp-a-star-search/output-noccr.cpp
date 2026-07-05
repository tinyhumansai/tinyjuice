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
    std::array<std::array<uint32_t, N>, N>
        board;  /// N x N array to store the current state of the Puzzle.

    std::vector<std::pair<int8_t, int8_t>> moves = {
        {0, 1},
        {1, 0},
        {0, -1},
        {-1,
         0}};  /// A helper array to evaluate the next state from current state;
    /**
     * @brief Finds an empty space in puzzle (in this case; a zero)
     * @returns a pair indicating integer distances from top and right
     * respectively, else returns -1, -1
     */
    std::pair<uint32_t, uint32_t> find_zero() {
        { … 9 line(s) … }
    /**
     * @brief check whether the index value is bounded within the puzzle area
     * @param value index for the current board
     * @returns `true` if index is within the board, else `false`
     */
    inline bool in_range(const uint32_t value) const { return value < N; }

 public:
    /**
     * @brief get the value from i units from right and j units from left side
     * of the board
     * @param i integer denoting ith row
     * @param j integer denoting column
     * @returns non-negative integer denoting the value at ith row and jth
     * column
     * @returns -1 if invalid i or j position
     */
    uint32_t get(size_t i, size_t j) const {
        if (in_range(i) && in_range(j)) {
            return board[i][j];
        }
        return -1;
    }
    /**
     * @brief Returns the current state of the board
     */
    std::array<std::array<uint32_t, N>, N> get_state() { return board; }

    /**
     * @brief returns the size of the EightPuzzle (number of row / column)
     * @return N, the size of the puzzle.
     */
    inline size_t get_size() const { return N; }
    /**
     * @brief Default constructor for EightPuzzle
     */
    EightPuzzle() {
        for (size_t i = 0; i < N; ++i) {
            for (size_t j = 0; j < N; ++j) {
                board[i][j] = ((i * 3 + j + 1) % (N * N));
            }
        }
    }
    /**
     * @brief Parameterized Constructor for EightPuzzle
     * @param init a 2-dimensional array denoting a puzzle configuration
     */
    explicit EightPuzzle(const std::array<std::array<uint32_t, N>, N> &init)
        : board(init) {}

    /**
     * @brief Copy constructor
     * @param A a reference of an EightPuzzle
     */
    EightPuzzle(const EightPuzzle<N> &A) : board(A.board) {}

    /**
     * @brief Move constructor
     * @param A a reference of an EightPuzzle
     */
    EightPuzzle(const EightPuzzle<N> &&A) noexcept
        : board(std::move(A.board)) {}
    /**
     * @brief Destructor of EightPuzzle
     */
    ~EightPuzzle() = default;

    /**
     * @brief Copy assignment operator
     * @param A a reference of an EightPuzzle
     */
    EightPuzzle &operator=(const EightPuzzle &A) {
        board = A.board;
        return *this;
    }

    /**
     * @brief Move assignment operator
     * @param A a reference of an EightPuzzle
     */
    EightPuzzle &operator=(EightPuzzle &&A) noexcept {
        board = std::move(A.board);
        return *this;
    }

    /**
     * @brief Find all possible states after processing all possible
     * moves, given the current state of the puzzle
     * @returns list of vector containing all possible next moves
     * @note the implementation is compulsory to create A* search
     */
    std::vector<EightPuzzle<N>> generate_possible_moves() {
        auto zero_pos = find_zero();
        // vector which will contain all possible state from current state
        std::vector<EightPuzzle<N>> NewStates;
        for (auto &move : moves) {
            if (in_range(zero_pos.first + move.first) &&
                in_range(zero_pos.second + move.second)) {
                { … 12 line(s) … }
    /**
     * @brief check whether two boards are equal
     * @returns `true` if check.state is equal to `this->state`, else
     * `false`
     */
    bool operator==(const EightPuzzle<N> &check) const {
        { … 12 line(s) … }
    /**
     * @brief check whether one board is lexicographically smaller
     * @returns `true` if this->state is lexicographically smaller than
     * `check.state`, else `false`
     */
    bool operator<(const EightPuzzle<N> &check) const {
        { … 9 line(s) … }
    /**
     * @brief check whether one board is lexicographically smaller or equal
     * @returns `true` if this->state is lexicographically smaller than
     * `check.state` or same, else `false`
     */
    bool operator<=(const EightPuzzle<N> &check) const {
        { … 9 line(s) … }

    /**
     * @brief friend operator to display EightPuzzle<>
     * @param op ostream object
     * @param SomeState a certain state.
     * @returns ostream operator op
     */
    friend std::ostream &operator<<(std::ostream &op,
                                    const EightPuzzle<N> &SomeState) {
        { … 9 line(s) … }
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
    /**
     * @brief Struct that handles all the information related to the current
     * state.
     */
    typedef struct Info {
        std::shared_ptr<Puzzle> state;  /// Holds the current state.
        size_t heuristic_value = 0;     /// stores h score
        { … 45 line(s) … }
         */
        Info &operator=(const Info &A) {
            { … 10 line(s) … }
        Info &operator=(Info &&A) noexcept {
            { … 10 line(s) … }

    std::shared_ptr<Info> Initial;  // Initial state of the AyStarSearch
    std::shared_ptr<Info> Final;    // Final state of the AyStarSearch
    /**
     * @brief Custom comparator for open_list
     */
    struct comparison_operator {
        bool operator()(const std::shared_ptr<Info> &a,
                        const std::shared_ptr<Info> &b) const {
            return *(a->state) < *(b->state);
        }
    };

 public:
    using MapOfPuzzleInfoWithPuzzleInfo =
        std::map<std::shared_ptr<Info>, std::shared_ptr<Info>,
                 comparison_operator>;

    using MapOfPuzzleInfoWithInteger =
        std::map<std::shared_ptr<Info>, uint32_t, comparison_operator>;

    using SetOfPuzzleInfo =
        std::set<std::shared_ptr<Info>, comparison_operator>;
    /**
     * @brief Parameterized constructor for AyStarSearch
     * @param initial denoting initial state of the puzzle
     * @param final denoting final state of the puzzle
     */
    AyStarSearch(const Puzzle &initial, const Puzzle &final) {
        Initial = std::make_shared<Info>(initial);
        Final = std::make_shared<Info>(final);
    }
    /**
     * @brief A helper solution: launches when a solution for AyStarSearch
     * is found
     * @param FinalState the pointer to the obtained final state
     * @param parent_of the list of all parents of nodes stored during A*
     * search
     * @returns the list of moves denoting moves from final state to initial
     * state (in reverse)
     */
    std::vector<Puzzle> Solution(
        std::shared_ptr<Info> FinalState,
        const MapOfPuzzleInfoWithPuzzleInfo &parent_of) {
        //  Useful for traversing from final state to current state.
        auto current_state = FinalState;
        { … 9 line(s) … }
        return answer;
    /**
     * Main algorithm for finding `FinalState`, given the `InitialState`
     * @param dist the heuristic finction, defined by the user
     * @param permissible_depth the depth at which the A* search discards
     * searching for solution
     * @returns List of moves from Final state to initial state, if
     * evaluated, else returns an empty array
     */
    std::vector<Puzzle> a_star_search(
        const std::function<uint32_t(const Puzzle &, const Puzzle &)> &dist,
        const uint32_t permissible_depth = 30) {
        MapOfPuzzleInfoWithPuzzleInfo
            parent_of;                       /// Stores the parent of the states
        { … 91 line(s) … }
    }
}  // namespace aystar_search
}  // namespace machine_learning

/**
 * @brief Self test-implementations
 * @returns void
 */
static void test() {
    // Renaming for simplicity
    using matrix3 = std::array<std::array<uint32_t, 3>, 3>;
    { … 164 line(s) … }
    }
/**
 * @brief Main function
 * @returns 0 on exit
 */
int main() {
    test();  // run self-test implementations
    return 0;
}