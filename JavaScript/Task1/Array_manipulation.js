(function initiateProcess() {
  let ar1 = [1, 2, 3, 4, 5];
  var firstElem = ar1.shift();

  mainFunc(firstElem, ...ar1)
    .then((res) => console.log("Result is:", res))
    .catch((err) => console.log("Error is", err));
})();

function mainFunc(firstElem, ...restElem) {
  let ar2 = [6, 7, 8, 9, 10];
  ar2.unshift(firstElem);
  ar2 = [...ar2, ...restElem];
  console.log("Combine:-", ar2);

  let sum = 0;
  for (let i in ar2) {
    sum += ar2[i];
  }

  return new Promise(function (resolve, reject) {
    if (sum > 30) {
      resolve("Yes!! Sum is greater than 30!");
    } else {
      reject("Sum isn't greater than 30");
    }
  });
}
