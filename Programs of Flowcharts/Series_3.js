//Series:- 1!, -3!, 5!, -7!
let n = parseInt(prompt("Enter a number:"));
let i = 1;
let fact = 1;

while (i <= n) {
    console.log(fact);
    i += 2;
    fact *= i * (i - 1) * (-1);
}