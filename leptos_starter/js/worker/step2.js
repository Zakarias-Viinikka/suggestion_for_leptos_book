//copied from step 1 js
function createWorker(fn) {
  var blob = new Blob(['self.onmessage = ', fn.toString()], { type: 'text/javascript' });
  var url = URL.createObjectURL(blob);

  return new Worker(url);
}

var myWorker = createWorker(function (e) {
  if (e.data === "worker do work") {
    setTimeout(() => {
      const workerResult = `Stuff worked. 'What stuff?' you may ask.`;
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
  console.log("javscript attempting to bend rust to their will")
  bending_rust(e.data);
  console.log("Worker responded with: ", e.data);
};

function bending_rust(result) {
  const event = new CustomEvent('obey_me_rust', {
      detail: String(result)
  });
  window.dispatchEvent(event);
}
