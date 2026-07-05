/**
 * @file
 * @brief Implementation of the [Random Pivot Quick
 * Sort](https://www.sanfoundry.com/cpp-program-implement-quick-sort-using-randomisation)
 * algorithm.
 * @details
 *          * A random pivot quick sort algorithm is pretty much same as quick
 * sort with a difference of having a logic of selecting next pivot element from
 * the input array.
 *          * Where in quick sort is fast, but still can give you the time
 * complexity of O(n^2) in worst case.
 *          * To avoid hitting the time complexity of O(n^2), we use the logic
 * of randomize the selection process of pivot element.
 *
 *          ### Logic
 *              * The logic is pretty simple, the only change is in the
 * partitioning algorithm, which is selecting the pivot element.
 *              * Instead of selecting the last or the first element from array
 * for pivot we use a random index to select pivot element.
 *              * This avoids hitting the O(n^2) time complexity in practical
 * use cases.
 *
 *       ### Partition Logic
 *           * Partitions are done such as numbers lower than the "pivot"
 * element is arranged on the left side of the "pivot", and number larger than
 * the "pivot" element are arranged on the right part of the array.
 *
 *       ### Algorithm
 *           * Select the pivot element randomly using getRandomIndex() function
 * from this namespace.
 *           * Initialize the pInd (partition index) from the start of the
 * array.
 *           * Loop through the array from start to less than end. (from start
 * to < end). (Inside the loop) :-
 *                   * Check if the current element (arr[i]) is less than the
 * pivot element in each iteration.
 *                   * If current element in the iteration is less than the
 * pivot element, then swap the elements at current index (i) and partition
 * index (pInd) and increment the partition index by one.
 *           * At the end of the loop, swap the pivot element with partition
 * index element.
 *           * Return the partition index from the function.
 *
 * @author [Nitin Sharma](https://github.com/foo290)
 */

#include <algorithm>  /// for std::is_sorted(), std::swap()
#include <array>      /// for std::array
#include <cassert>    /// for assert
#include <ctime>      /// for initializing random number generator
#include <iostream>   /// for IO operations
#include <tuple>      /// for returning multiple values form a function at once

/**
 * @namespace sorting
 * @brief Sorting algorithms
 */
namespace sorting {
/**
 * @brief Functions for the [Random Pivot Quick
 * Sort](https://www.sanfoundry.com/cpp-program-implement-quick-sort-using-randomisation)
 * implementation
 * @namespace random_pivot_quick_sort
 */
namespace random_pivot_quick_sort {
/**
 * @brief Utility function to print the array
 * @tparam T size of the array
 * @param arr array used to print its content
 * @returns void
 * */
template <size_t T>
void showArray(std::array<int64_t, T> arr) {
    for (int64_t i = 0; i < arr.size(); i++) {
        std::cout << arr[i] << " ";
    }
    std::cout << std::endl;
}

/**
 * @brief Takes the start and end indices of an array and returns a random
 * int64_teger between the range of those two for selecting pivot element.
 *
 * @param start The starting index.
 * @param end The ending index.
 * @returns int64_t A random number between start and end index.
 * */
int64_t getRandomIndex(int64_t start, int64_t end) {
    srand(time(nullptr));  // Initialize random number generator.
    int64_t randomPivotIndex = start + rand() % (end - start + 1);
    return randomPivotIndex;
}

/**
 * @brief A partition function which handles the partition logic of quick sort.
 * @tparam size size of the array to be passed as argument.
 * @param start The start index of the passed array
 * @param end The ending index of the passed array
 * @returns std::tuple<int64_t , std::array<int64_t , size>> A tuple of pivot
 * index and pivot sorted array.
 */
template <size_t size>
std::tuple<int64_t, std::array<int64_t, size>> partition(
    std::array<int64_t, size> arr, int64_t start, int64_t end) {
    int64_t pivot = arr[end];  // Randomly selected element will be here from
                               // caller function (quickSortRP()).
    { … 11 line(s) … ⟦tj:1a2e05f605c286abc0412df3ca5d9b6a⟧ }
    return std::make_tuple(pInd, arr);

/**
 * @brief Random pivot quick sort function. This function is the starting point
 * of the algorithm.
 * @tparam size size of the array to be passed as argument.
 * @param start The start index of the passed array
 * @param end The ending index of the passed array
 * @returns std::array<int64_t , size> A fully sorted array in ascending order.
 */
template <size_t size>
std::array<int64_t, size> quickSortRP(std::array<int64_t, size> arr,
                                      int64_t start, int64_t end) {
    if (start < end) {
        int64_t randomIndex = getRandomIndex(start, end);
    { … 15 line(s) … ⟦tj:72008c3ac1ccc835b5eed1380a4f62df⟧ }
    return arr;

/**
 * @brief A function utility to generate unsorted array of given size and range.
 * @tparam size Size of the output array.
 * @param from Stating of the range.
 * @param to Ending of the range.
 * @returns std::array<int64_t , size> Unsorted array of specified size.
 * */
template <size_t size>
std::array<int64_t, size> generateUnsortedArray(int64_t from, int64_t to) {
    srand(time(nullptr));
    std::array<int64_t, size> unsortedArray{};
    { … 9 line(s) … ⟦tj:a9a04ea89478305bfca34676a9a99506⟧ }
    return unsortedArray;

}  // namespace random_pivot_quick_sort
}  // namespace sorting

/**
 * @brief a class containing the necessary test cases
 */
class TestCases {
 private:
    /**
     * @brief A function to print64_t given message on console.
     * @tparam T Type of the given message.
     * @returns void
     * */
    template <typename T>
    void log(T msg) {
        // It's just to avoid writing cout and endl
        std::cout << "[TESTS] : ---> " << msg << std::endl;
    }

 public:
    /**
     * @brief Executes test cases
     * @returns void
     * */
    void runTests() {
        { … 9 line(s) … ⟦tj:fc7ac61bfe85a66e570f0cedb7be1b73⟧ }

    /**
     * @brief A test case with single input
     * @returns void
     * */
    void testCase_1() {
        const int64_t inputSize = 1;
        log("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
        { … 21 line(s) … ⟦tj:0b6b1b423a94dc64c683aeb9572ca538⟧ }
            "~");

    /**
     * @brief A test case with input array of length 500
     * @returns void
     * */
    void testCase_2() {
        const int64_t inputSize = 500;
        log("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
        { … 23 line(s) … ⟦tj:550382bc1a7fa7872c0b445f6059ec40⟧ }
            "~");

    /**
     * @brief A test case with array of length 1000.
     * @returns void
     * */
    void testCase_3() {
        const int64_t inputSize = 1000;
        log("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
        { … 24 line(s) … ⟦tj:f66a4ebe63165ac2a32f124f2fadeb91⟧ }
    }

/**
 * @brief Self-test implementations
 * @returns void
 */
static void test() {
    TestCases tc = TestCases();
    tc.runTests();
}

/**
 * @brief Main function
 * @returns 0 on exit
 */
int main() {
    test();  // Executes various test cases.

    const int64_t inputSize = 10;
    std::array<int64_t, inputSize> unsorted_array =
        sorting::random_pivot_quick_sort::generateUnsortedArray<inputSize>(
            50, 1000);
    std::cout << "Unsorted array is : " << std::endl;
    sorting::random_pivot_quick_sort::showArray(unsorted_array);

    std::array<int64_t, inputSize> sorted_array =
        sorting::random_pivot_quick_sort::quickSortRP(
            unsorted_array, 0, unsorted_array.size() - 1);
    std::cout << "Sorted array is : " << std::endl;
    sorting::random_pivot_quick_sort::showArray(sorted_array);
    return 0;
}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (11901 bytes) is available by calling tinyjuice_retrieve with token "1869540b10d352171aec2f3218a6f216" (marker ⟦tj:1869540b10d352171aec2f3218a6f216⟧)]