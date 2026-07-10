/*self.onmessage = (e) => {
  if (e.data === "worker do work") {
    /*put both of these inside this thing: setTimeout(() => {
      // code here
    }, 2000); */
    setTimeout(() => {
      const workerResult = `Result: Stuff worked. 'What stuff?' you may ask. We don't ask questions around here`;
      postMessage(workerResult);
    }, 2000);
  } else {
    setTimeout(() => {
      const workerResult = `worker don't wanna do work`;
      postMessage(workerResult);
    }, 2000);
  }
};
*/
