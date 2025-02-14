# Separate OpCode, Separate AIRs

This chapter explains how Stwo enables composition of multiple AIRs, particularly useful for complex computations with distinct operations.

## AIR Composition

Stwo allows you to:

- Define separate AIRs for different operations
- Compose them efficiently
- Maintain modularity and reusability

## Benefits of Separation

1. **Modularity**

   - Each operation has its own AIR
   - Easier to test and verify
   - Simpler to maintain

2. **Optimization**

   - Specialized constraints per operation
   - Efficient lookup table usage
   - Better proof composition

3. **Flexibility**
   - Mix and match operations
   - Add new operations easily
   - Reuse existing AIRs
