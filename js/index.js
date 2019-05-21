import("../crate/pkg").then(module => {
  const { Universe } = module;
  const pre = document.getElementById("game-of-life-canvas");
  const universe = Universe.new();

  const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    requestAnimationFrame(renderLoop);
  };

  requestAnimationFrame(renderLoop);
});
