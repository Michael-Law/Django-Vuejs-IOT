<template>
  <div id="mapContainer" class="basemap"></div>
</template>

<script>
import mapboxgl from "mapbox-gl";

export default {
  name: "BaseMap",
  data() {
    return {
      accessToken:
        "pk.eyJ1IjoiaW52aWN0YTExMSIsImEiOiJja2hyMjM0cXAwcWJ1MnNyc2tzbGlneWw2In0.GDyvy8Yi8WBasxZKfH_0bA",
    };
  },
  mounted() {
    mapboxgl.accessToken = this.accessToken;

    new mapboxgl.Map({
      container: "mapContainer",
      style: "mapbox://styles/invicta111/ckhr495nz00ay1amkd1bjstli",
      center: [57.59257009517367, -20.289607639709068],
      zoom: 12,
      pitch: 60,
      maxBounds: [
        [57.270295862485135, -20.531959293688164],
        [57.88271863500014, -19.974641272520877],
      ],
    });
  },
};
</script>
<style lang="scss" scoped>
.basemap {
  width: 100%;
  height: 100%;
}
</style>
