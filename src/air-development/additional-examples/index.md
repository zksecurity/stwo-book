# Additional Examples

| Example                                                                            | Preprocessed Columns                          | Trace Columns                               | LogUp Columns                                    |
| :--------------------------------------------------------------------------------- | :-------------------------------------------- | :------------------------------------------ | :----------------------------------------------- |
| **Permutation argument check**                                                     |                                               | - unordered list<br>- ordered list          | 1 / unordered list - 1 / ordered list            |
| **Range Check (0 <= a < 2^bits)**                                                  | [0,2^bits) rows                               | - lookup columns<br>- multiplicities column | - 1 / lookup<br>- multiplicity / preprocessed    |
| **Comparator check (a > b)**                                                       | [0,2^bits) rows                               | - a <br> - b <br> - multiplicities column   | - 1 / (a - b) <br> - multiplicity / preprocessed |
| **IsZero check (a == 0)**                                                          |                                               | - a <br> - a_inv                            |                                                  |
| **XOR operations**                                                                 | valid XOR operations                          | - lookup columns<br>- multiplicities column |                                                  |
| **Selectors**                                                                      | columns of 0s and 1s                          |                                             |                                                  |
| **Checking whether current row is first row or not**                               | single column (first row = 1, other rows = 0) |                                             |                                                  |
| **Connecting multiple components** (output of Component A is input of Component B) |                                               |                                             | - 1 / output <br> - 1 / input \* (-1)            |
| **Public Input/Output**                                                            |                                               |                                             | 1 / input + 1 / output                           |

Above is a list of additional examples that you can implement as an AIR using Stwo, some of which we have already implemented in the previous sections.
