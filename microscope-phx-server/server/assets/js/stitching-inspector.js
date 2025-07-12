import OpenSeadragon from "openseadragon";

let StitchingInspectorHook = {
  mounted() {
    let element = this.el;
    let image = this.el.image;
    this.handleEvent("update-stitching-inspector", (payload) => {
        image = payload.image
        OpenSeadragon({
            id: element.id,
            tileSources: {
                type: 'image',
                url: "/images/stitched.png"
            },
            showNavigator: true,
            maxZoomPixelRatio: 10
        })
    })
  },
}

export default StitchingInspectorHook;