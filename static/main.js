import init, { reset_game, check_guess } from "../pkg/guessing_game_web.js";

async function run() {
  await init();
  reset_game();
  start_game();
}

window.makeGuess = () => {
  const value = parseInt(document.getElementById("guess").value, 10);
  const result = check_guess(value);
  document.getElementById("result").textContent = result;
};

run();


