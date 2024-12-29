const { invoke, Channel } = window.__TAURI__.core;
const { emit, listen } = window.__TAURI__.event;
const { appWindow } = window.__TAURI__.window;

let timerInputEl;
let timerOutputEl;

function start_countdown(e) {
  console.log(e);
  // const onEvent = new Channel();
  // onEvent.onmessage = (message) => {
  //   let content = message.data.remainingTime;
  //   let content_str = ""
  //   console.log(content);
  //   if (content < 5) {
  //     content_str = content.toFixed(1);
  //   } else {
  //     content_str = content.toFixed(0);
  //   }
  //   timerOutputEl.textContent = content_str;
  //   timerOutputEl.classList.remove("ok-time");
  //   timerOutputEl.classList.remove("warning-time");
  //   timerOutputEl.classList.remove("danger-time");
  //   if (message.data.remainingTime > 10) {
  //     timerOutputEl.classList.add("ok-time");
  //   } else if (message.data.remainingTime > 5) {
  //     timerOutputEl.classList.add("warning-time");
  //   }
  //   else {
  //     timerOutputEl.classList.add("danger-time");
  //   }
  //   console.log(`got download event ${message.event}, ${message.data.timerId}`);
  // };
  console.log("Starting timer");
  console.log(timerInputEl.value);
  invoke("start_timer", { durationSecs: parseFloat(timerInputEl.value) });
}

function pause(e) {
  console.log("Pausing!");
  console.log(e);
}

listen('timer-update', (event) => {
  let content = event.payload;
  let content_str = ""
  content_str = content.toFixed(1);

  timerOutputEl.textContent = content_str;

  timerOutputEl.classList.remove("ok-time");
  timerOutputEl.classList.remove("warning-time");
  timerOutputEl.classList.remove("danger-time");

  if (content > 10) {
    timerOutputEl.classList.add("ok-time");
  } else if (content > 5) {
    timerOutputEl.classList.add("warning-time");
  }
  else {
    timerOutputEl.classList.add("danger-time");
  }
})

listen('timer-finished', (event) => {
  let content_str = "0"

  timerOutputEl.textContent = content_str;

  timerOutputEl.classList.remove("ok-time");
  timerOutputEl.classList.remove("warning-time");
  timerOutputEl.classList.remove("danger-time");

  timerOutputEl.classList.add("danger-time");
})

window.addEventListener("DOMContentLoaded", () => {
  timerInputEl = document.querySelector("#timer-input");
  timerOutputEl = document.querySelector("#timer-output");
  document.querySelector("#start-button").addEventListener("click", (e) => {
    e.preventDefault();
    start_countdown(e);
  })
  document.querySelector("#pause-button").addEventListener("click", (e) => {
    e.preventDefault();
    pause(e);
  })
  // document.querySelector("#timer-form").addEventListener("submit", (e) => {
  //   e.preventDefault();
  //   start_countdown(e);
  // });
});
