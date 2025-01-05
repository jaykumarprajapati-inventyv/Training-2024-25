//sin(x) = x - x^3/3! + x^5/5! - x^7/7! + ...
function calculateSum(n, x) {
    x = (x * 3.14) / 180;

    let i = 1;      
    let sum = x;     
    let temp = x;    

    while (i <= n) { 
        temp = (temp * (-1) * x * x) / (2 * i * (2 * i + 1));
        sum += temp;
        i++;
    }

    return sum; 
 
}

let n = parseInt(prompt("Enter num:"));;   
let x = 30;  

let result = calculateSum(n, x);
console.log("Sum:", result);
