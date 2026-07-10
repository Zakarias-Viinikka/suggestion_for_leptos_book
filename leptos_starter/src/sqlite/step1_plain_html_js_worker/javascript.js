function createWorker(fn) {
  var blob = new Blob(['self.onmessage = ', fn.toString()], { type: 'text/javascript' });
  var url = URL.createObjectURL(blob);

  return new Worker(url);
}

var myWorker = createWorker(function (e) {
  if (e.data === "worker do work") {
    setTimeout(() => {
      const workerResult = `Result: Stuff worked. 'What stuff?' you may ask. We don't ask questions around here`;
      self.postMessage(workerResult);
    }, 2000);
  } else {
    setTimeout(() => {
      self.postMessage("worker don't wanna do work");
    }, 2000);
  }
});

function make_worker_work() {
  myWorker.postMessage("worker do work");
}

myWorker.onmessage = (e) => {
  console.log("Worker responded with: ", e.data);
};

window.onload = () => {
  console.log("starting work");
  make_worker_work();
};
