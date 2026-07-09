function set_anim_state_bounce() {
  document.getElementById("bounce-wrapper").classList.add("slide-top");
}

function set_anim_state_finish_bounce() {
  document.getElementById("bounce-wrapper").classList.remove("slide-top");
}

function set_anim_state_backflip() {
  const box = document.getElementById("box-trying-to-become-square");
  box.classList.add("flip-horizontal-bottom");
  setTimeout(() => {
    box.classList.remove("flip-horizontal-bottom");
  }, 400);
}
