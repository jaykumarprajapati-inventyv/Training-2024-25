//1,1,2,1,1,2,3,2,1,..
const n = parseInt(prompt("Enter the value of n:"));
let i = 1;

while (i <= n) {
  let j = 1;
  while (j <= i * 2 - 1) {
    if (j <= i) {
      console.log(j); 
    } else {
      console.log(i * 2 - j); 
    }
    j++; 
  }
  i++; 
}
