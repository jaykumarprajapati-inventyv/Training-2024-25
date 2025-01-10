(function initialFunc() {
  let ar1 = [Symbol(1), Symbol(2), Symbol(3), Symbol(4), Symbol(5)];

  let firstElem = ar1.shift();
  console.log("firstElem", firstElem.description);
  console.log(ar1);

  mainFunc(firstElem, ...ar1)
    .then((res) => {
      console.log("Response is:", res);
    })
    .catch((err) => {
      console.log("Error is:", err);
    });
})();

function mainFunc(firstElem, ...ar1) {
  let ar2 = [Symbol(6), Symbol(7), Symbol(8), Symbol(9), Symbol(10)];
  ar2.unshift(firstElem);

  ar2 = [...ar2, ...ar1];
  console.log("Combine", ar2);

  let sum = 0;
  for (let i in ar2) {
    sum += Number(ar2[i].description);
  }
  console.log("Sum is:", sum);

  return new Promise(function (resolve, reject) {
    if (sum > 30) {
      resolve("Yes!! Sum is greater than 30.");
    } else {
      reject("No!! Sum isn't greater than 30.");
    }
  });
}
