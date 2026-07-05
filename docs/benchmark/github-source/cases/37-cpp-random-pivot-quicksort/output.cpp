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
    { … 5 line(s) … ⟦tj:691105eb638d983ed69b587b763df7b5⟧ }

/**
 * @brief Takes the start and end indices of an array and returns a random
 * int64_teger between the range of those two for selecting pivot element.
 *
 * @param start The starting index.
 * @param end The ending index.
 * @returns int64_t A random number between start and end index.
 * */
int64_t getRandomIndex(int64_t start, int64_t end) {
    { … 4 line(s) … ⟦tj:22734377cb7f462cb2a11386008afd73⟧ }

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
    { … 15 line(s) … ⟦tj:1a2e05f605c286abc0412df3ca5d9b6a⟧ }

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
    { … 19 line(s) … ⟦tj:72008c3ac1ccc835b5eed1380a4f62df⟧ }

/**
 * @brief A function utility to generate unsorted array of given size and range.
 * @tparam size Size of the output array.
 * @param from Stating of the range.
 * @param to Ending of the range.
 * @returns std::array<int64_t , size> Unsorted array of specified size.
 * */
template <size_t size>
std::array<int64_t, size> generateUnsortedArray(int64_t from, int64_t to) {
    { … 13 line(s) … ⟦tj:a9a04ea89478305bfca34676a9a99506⟧ }

}  // namespace random_pivot_quick_sort
}  // namespace sorting

/**
 * @brief a class containing the necessary test cases
 */
class TestCases {
    { … 125 line(s) … ⟦tj:a9dacb475abdb5ebc61d36fb559ebea2⟧ }

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
    { … 16 line(s) … ⟦tj:1c7c537f3dc3476f3b32f07805b52707⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (11901 bytes) is available by calling tinyjuice_retrieve with token "1869540b10d352171aec2f3218a6f216" (marker ⟦tj:1869540b10d352171aec2f3218a6f216⟧)]