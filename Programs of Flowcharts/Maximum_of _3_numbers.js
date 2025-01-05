let x=parseInt(prompt("Enter x:"))
let y=parseInt(prompt("Enter y:"))
let z=parseInt(prompt("Enter z:"))

if(x>y && x>z){
   console.log("x is maximum")
}
else if(y>x && y>z){
   console.log("y is maximum")
}
else{
    console.log("z is maximum")
}