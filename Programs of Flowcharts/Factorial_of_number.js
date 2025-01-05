let fact = 1;
let i = 1;
let n=parseInt(prompt("Enter n:"));

while(i <= n) {
  fact *= i;
  i++;
}

console.log(fact);