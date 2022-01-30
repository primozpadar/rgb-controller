const { contextBridge } = require("electron");
const ReinventedColorWheel = require("reinvented-color-wheel");

const dgram = require("dgram");
const server = dgram.createSocket({ type: "udp4", reuseAddr: true });
const PORT = 50000;
const BROADCAST_ADDR = "192.168.1.255";

const getColor = (x) => {
  if (typeof x === "string") {
    return x.padStart(3, "0");
  } else {
    return x.toString().padStart(3, "0");
  }
};

contextBridge.exposeInMainWorld("colorwheel", {
  mount: (element) => {
    const colorWheel = new ReinventedColorWheel({
      appendTo: element,
      hsv: [0, 100, 100],
      wheelDiameter: 300,
      wheelThickness: 30,
      handleDiameter: 20,
      wheelReflectsSaturation: true,

      onChange: (color) => {
        const [r, g, b] = color.rgb;
        const message = Buffer.from(
          `00C${getColor(r)}${getColor(g)}${getColor(b)}`
        );
        server.send(message, 0, message.length, PORT, BROADCAST_ADDR);
      },
    });

    return colorWheel;
  },
});
