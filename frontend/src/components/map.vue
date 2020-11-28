<template>
  <MglMap
    :accessToken="mapboxAccessToken"
    :mapStyle.sync="mapStyle"
    :center="coordinates"
    :maxBounds="maxBounds"
  >
    <MglNavigationControl position="top-right" />
    <MglMarker :coordinates="coordinates">
      <MglPopup>
        <VCard>
          <div>Amount : 100</div>
        </VCard>
      </MglPopup>
    </MglMarker>
  </MglMap>
</template>

<script>
name: "BaseMap";
import { MglMap, MglPopup, MglMarker } from "vue-mapbox";

export default {
  components: {
    MglMap,
    MglMarker,
    MglPopup,
  },

  data() {
    return {
      mapboxAccessToken:
        "pk.eyJ1IjoiaW52aWN0YTExMSIsImEiOiJja2hyMjM0cXAwcWJ1MnNyc2tzbGlneWw2In0.GDyvy8Yi8WBasxZKfH_0bA", // your access token. Needed if you using Mapbox maps
      mapStyle: "mapbox://styles/invicta111/ckhr495nz00ay1amkd1bjstli",
      coordinates: [57.59257009517367, -20.289607639709068],
      maxBounds: [
        [57.270295862485135, -20.531959293688164],
        [57.88271863500014, -19.974641272520877],
      ],
    };
  },
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