<template>
  <div id="mapContainer" class="basemap"></div>
</template>

<script>
import mapboxgl from "mapbox-gl";
import { MglMap, MglGeojsonLayer } from "vue-mapbox";
import navbar from "@/components/navbar.vue";

export default {
  name: "BaseMap",
  data() {
    return {
      accessToken:
        "pk.eyJ1IjoiaW52aWN0YTExMSIsImEiOiJja2hyMjM0cXAwcWJ1MnNyc2tzbGlneWw2In0.GDyvy8Yi8WBasxZKfH_0bA",
    };
  },

  mounted() {
    // axios
    //     .get("http://172.16.158.116:5001//ping")
    //     .then((response) => (this.test = response.data))
    //     .catch((error) => console.log(error));

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
  computed: {},
};
</script>
<style lang="scss" scoped>
.sidebar {
  width: 33.3333%;
}

.basemap {
  border-left: 1px solid #fff;
  position: absolute;
  left: 33.3333%;
  width: 75%;
  top: 64px;
  bottom: 0;
}
</style>
