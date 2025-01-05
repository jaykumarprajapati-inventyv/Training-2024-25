//Series:- 1,-2,3,-4
let n = parseInt(prompt("Enter num:"));
let i = 1;
let mul = 1;

while (i <= n) {
  console.log(i * mul);
  i++;
  mul *= -1;
}