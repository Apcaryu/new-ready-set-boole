# new-ready-set-boole

## Exercice00
### Objective
Create a function that take two u32 and return u32 only with:\
bitwise AND, OR, XOR ( &, |, ^ )\
left, rigth shift( <<, >> )\
assignment( = )\
comparaison operator (==, !=, <, >, <=, >=)\
Prototype:
```rust
pub fn adder(a: u32, b: u32) -> u32;
```
### How it works
Step 1:\
Get in memory where we have two bits equal "1"
```rust
let mut mem = a & b;
```
Step 2:\
Exclude "1" where two bits equal "1"
```rust
let mut res = a ^ b;
```
Step 3:\
Add the hold then if the result has a hold equal to zero then we leave the loop
```rust
while mem != 0 {
  // make the hold
  let shift_mem = mem << 1;
  // check the hold
  mem = shift_mem & res;
  // add the hold
  res = res ^ shift_mem;
}
```
Step 4:\
return the result
```rust
res // equal: return res;
```
