# Advent of Code 2024

For this year, I've decided I'd write every 5 days in a different language:

## Days 1-5: Zig

This is my first time actually writing in Zig, so don't expect optimized or idiomatic code.

### Unit-testing

```sh
zig test <DAY_SOURCE>
```

### Running

```sh
zig run <DAY_SOURCE>
```

## Day 6-10: C

Practiced light macro programming, avoiding dynamic allocations and unit testing in raw C. Written for C23.

Since C has no official runner, I've written a script (`runday`) to compile and run/test each day file. It also automatically downloads inputs and pipes them into the appropriate program. It uses Zig as the compiler.

### Unit-testing

```sh
./runday <DAY> test
```

### Running

```sh
./runday <DAY> run
```

## Day 11-15: C++

My learning goals here are: 

1) modern C++ features, such as ranges, the new `print` function, multi-dimensional spans, etc.;
2) writing idiomatic code by using the STL;
3) using templates to automatically generate code;
4) test-driven development in C++.

Like C, since C++ has no runner, I've adapted `runday` for C++, replacing Zig with GCC.

### Unit-testing

```sh
./runday <DAY> test
```

### Running

```sh
./runday <DAY> run
```

## Day 16-20: TBD

## Day 21-25: TBD

