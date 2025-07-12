let MagnifierHook = {
  mounted() {
    let element = this.el;
    let zoom = this.el.zoom;
    let size = this.el.size;
    this.magnify(element, zoom);
    this.handleEvent("update-zoom", (payload) => {
        this.magnify(element, payload.zoom, payload.size)
    })
  },

  magnify(img, zoom, size) {
            //set size of magnifying glass
        
        for (let sheet of document.styleSheets) {
        for (let rule of sheet.cssRules || []) {
            if (rule.selectorText === ".img-magnifier-glass") {
            rule.style.height = size+"px";
            rule.style.width = size+"px";
            }
        }
        }
    
    let glass = document.createElement("DIV");
    glass.setAttribute("class", "img-magnifier-glass");

    img.parentElement.insertBefore(glass, img);

    glass.style.backgroundImage = `url('${img.src}')`;
    glass.style.backgroundRepeat = "no-repeat";
    glass.style.backgroundSize = `${img.width * zoom}px ${img.height * zoom}px`;

    //Cursor offset to center Magnifying glass
    let bw = 3;
    let w = glass.offsetWidth / 2;
    let h = glass.offsetHeight / 2;

    const moveMagnifier = (e) => {
      e.preventDefault();
      let pos = getCursorPos(e);
      let x = pos.x;
      let y = pos.y;

      if (x > img.width - (w / zoom)) { x = img.width - (w / zoom); }
      if (x < w / zoom) { x = w / zoom; }
      if (y > img.height - (h / zoom)) { y = img.height - (h / zoom); }
      if (y < h / zoom) { y = h / zoom; }

      glass.style.left = (x - w + 30) + "px";
      glass.style.top = (y - h + 30) + "px";
      glass.style.backgroundPosition = `-${x * zoom - w + bw}px -${y * zoom - h + bw}px`;
    };

    const getCursorPos = (e) => {
      e = e || window.event;
      let a = img.getBoundingClientRect();
      let x = e.pageX - a.left - window.scrollX;
      let y = e.pageY - a.top - window.scrollY;
      return { x: x, y: y };
    };

    glass.addEventListener("mousemove", moveMagnifier);
    img.addEventListener("mousemove", moveMagnifier);
  }
};

export default MagnifierHook;