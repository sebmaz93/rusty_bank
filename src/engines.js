const engine = {
  working: true,
};

const mustang = {
  engine,
  name: "Mustang",
};

const camaro = {
  engine,
  name: "Camaro",
};

function checkEngine(car) {
  if (car.name === "Mustang") {
    car.engine.working = false;
  }
}
