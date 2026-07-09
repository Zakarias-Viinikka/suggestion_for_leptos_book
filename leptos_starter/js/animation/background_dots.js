let grid_x_size;
let grid_y_size;

function start_anim(x, y) {
  grid_x_size = x;
  grid_y_size = y;
  setInterval(() => {
    const randomX = Math.floor(Math.random() * grid_x_size);
    const randomY = Math.floor(Math.random() * grid_y_size);
    fade_animation(randomX, randomY);
  }, 2000);
}

function fade_animation(x, y) {
  const target = document.getElementById(`dot-${x}-${y}`);
  if (target) {
    target.classList.add("fade-in-out");
    setTimeout(() => {
      target.classList.remove("fade-in-out");
    }, 1000);
  }
}
