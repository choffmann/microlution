// If you want to use Phoenix channels, run `mix help phx.gen.channel`
// to get started and then uncomment the line below.
// import "./user_socket.js"

// You can include dependencies in two ways.
//
// The simplest option is to put them in assets/vendor and
// import them using relative paths:
//
//     import "../vendor/some-package.js"
//
// Alternatively, you can `npm install some-package --prefix assets` and import
// them using a path starting with the package name:
//
//     import "some-package"
//

// Include phoenix_html to handle method=PUT/DELETE in forms and buttons.
import "phoenix_html"
// Establish Phoenix Socket and LiveView configuration.
import {Socket} from "phoenix"
import {LiveSocket} from "phoenix_live_view"
import topbar from "../vendor/topbar"

let hooks = {}
hooks.RunJS = {
  mounted() {
    this.handleEvent("run-js", (payload) => {
      console.log("JS received message:", payload.message);
      // You can trigger any JS behavior here
    });
  }
};

hooks.MiniMap = {
  mounted() {
    const img = document.getElementById("myImage");
    const canvas = document.getElementById("myCanvas");
    const ctx = canvas.getContext("2d");
    let lineWidth = "3"
    let width = 10
    let height = 10
    let canvas_width = canvas.width
    let canvas_height = canvas.height
    let current_x = (canvas_width / 2) - width / 2;
    let current_y = (canvas_height / 2) - height / 2;
    let minimap_step_size_modifier = 400 // 400 = 0.5cm bei 10500 Steps Richtung Runter

    ctx.beginPath();
    ctx.rect(current_x, current_y, width, height);
    ctx.lineWidth = lineWidth;
    ctx.strokeStyle = "red";
    ctx.stroke();

    this.handleEvent("update-minimap", (payload) => {
      ctx.clearRect(0,0,500,500)
      ctx.beginPath();
      current_x += JSON.parse(payload.x / minimap_step_size_modifier);
      current_y += JSON.parse(payload.y / minimap_step_size_modifier);
      ctx.rect(current_x, current_y, width, height);
      ctx.lineWidth = lineWidth;
      ctx.strokeStyle = "red";
      ctx.stroke();
    })
    this.handleEvent("reset-minimap", (payload) => {
      ctx.clearRect(0,0,500,500)
      ctx.beginPath();
      current_x = (canvas_width / 2) - width / 2;
      current_y = (canvas_height / 2) - height / 2;
      ctx.rect(current_x, current_y, width, height);
      ctx.lineWidth = lineWidth;
      ctx.strokeStyle = "red";
      ctx.stroke();
    })
  }
}

hooks.StitchingBoxesPreview = {
  mounted() {
    const canvas = document.getElementById("stitching-boxes-preview");
    const ctx = canvas.getContext("2d");
    let lineWidth = "2"
    let box_gap = 5;
    let x_steps = 2;
    let y_steps = 2;
    let canvas_width = canvas.width
    let canvas_height = canvas.height
    let offset_x = 5;
    let offset_y = 5;
    let max_box_width = (canvas_width - offset_x) / x_steps - box_gap
    let max_box_height = (canvas_height - offset_y) / y_steps - box_gap
    let current_x = offset_x;
    let current_y = offset_y;
    console.log(canvas.width)

    drawBoxes(x_steps, y_steps)


    this.handleEvent("update-stitching-preview-boxes", (payload) => {
      ctx.clearRect(0,0,500,500)
      x_steps = payload.x
      max_box_width = (canvas_width - offset_x) / x_steps - box_gap
      y_steps = payload.y
      max_box_height = (canvas_height - offset_y) / y_steps - box_gap

      drawBoxes(x_steps, y_steps)

    })

    this.handleEvent("update-stitching-boxes", (payload) => {
      ctx.clearRect(0,0,500,500)
      x_steps = payload.x
      max_box_width = (canvas_width - offset_x) / x_steps - box_gap
      y_steps = payload.y
      max_box_height = (canvas_height - offset_y) / y_steps - box_gap

      drawBoxes(x_steps, y_steps)

    })


    function drawBoxes() {
          for(let i = 0; i < y_steps; i++) {
      for(let j = 0; j < x_steps; j++) {
        drawBox(current_x, current_y, i, j, max_box_width, max_box_height, box_gap)

      }
    }
    }
    function drawBox(current_x, current_y, i, j, max_box_width, max_box_height, box_gap) {
        ctx.beginPath();
        ctx.rect(current_x + (j * (max_box_width + box_gap)), current_y + (i * (max_box_height + box_gap)), max_box_width, max_box_height);
        ctx.lineWidth = lineWidth;
        ctx.strokeStyle = "red";
        ctx.stroke();
    }
        function drawFillBox(current_x, current_y, i, j, max_box_width, max_box_height, box_gap) {
        ctx.beginPath();
        ctx.rect(current_x + (j * (max_box_width + box_gap)), current_y + (i * (max_box_height + box_gap)), max_box_width, max_box_height);
        ctx.fill()
        ctx.lineWidth = lineWidth;
        ctx.strokeStyle = "red";
        ctx.stroke();
    }
  }
}

hooks.HoldFocusButton = {
  mounted() {
    this.interval = null;

    const send = () => {
      this.pushEventTo(this.el, "move-z-in-direction", { direction: this.el.getAttribute("phx-value-direction")});
    };

    this.el.addEventListener("mousedown", () => {
      send();
      this.interval = setInterval(send, 500);
    });

    this.el.addEventListener("mouseup", () => {
      clearInterval(this.interval);
    });

    this.el.addEventListener("mouseleave", () => {
      clearInterval(this.interval);
    });
  },
  destroyed() {
    clearInterval(this.interval);
  }
};

// hooks.HoldNavigateButton = {
//   mounted() {
//     this.interval = null;

//     const send = () => {
//       this.pushEventTo(this.el, "move-in-direction", { direction: this.el.getAttribute("phx-value-direction")});
//     };

//     this.el.addEventListener("mousedown", () => {
//       send();
//       this.interval = setInterval(send, 700);
//     });

//     this.el.addEventListener("mouseup", () => {
//       clearInterval(this.interval);
//     });

//     this.el.addEventListener("mouseleave", () => {
//       clearInterval(this.interval);
//     });
//   },
//   destroyed() {
//     clearInterval(this.interval);
//   }
// };

let csrfToken = document.querySelector("meta[name='csrf-token']").getAttribute("content")
let liveSocket = new LiveSocket("/live", Socket, {
  longPollFallbackMs: 2500,
  params: {_csrf_token: csrfToken},
  hooks: hooks
})

// Show progress bar on live navigation and form submits
topbar.config({barColors: {0: "#29d"}, shadowColor: "rgba(0, 0, 0, .3)"})
window.addEventListener("phx:page-loading-start", _info => topbar.show(300))
window.addEventListener("phx:page-loading-stop", _info => topbar.hide())

// connect if there are any LiveViews on the page
liveSocket.connect()

// expose liveSocket on window for web console debug logs and latency simulation:
// >> liveSocket.enableDebug()
// >> liveSocket.enableLatencySim(1000)  // enabled for duration of browser session
// >> liveSocket.disableLatencySim()
window.liveSocket = liveSocket

