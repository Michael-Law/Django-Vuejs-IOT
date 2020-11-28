<template>
  <MglMap :accessToken="accessToken" :mapStyle.sync="mapStyle">
    <!-- Adding navigation control -->
    <MglNavigationControl position="top-right" />
    <!-- Adding GeoJSON layer -->
    <MglGeojsonLayer
      :sourceId="geoJsonSource.id"
      :source="geoJsonSource"
      layerId="myLayer"
      :layer="geoJsonlayer"
    />
  </MglMap>
</template>

<script>
import { MglMap, MglNavigationControl, MglGeojsonLayer } from "vue-mapbox";

export default {
  components: {
    MglMap,
    MglNavigationControl,
    MglGeojsonLayer,
  },
  data() {
    return {
      accessToken:
        "pk.eyJ1IjoiaW52aWN0YTExMSIsImEiOiJja2hyMjM0cXAwcWJ1MnNyc2tzbGlneWw2In0.GDyvy8Yi8WBasxZKfH_0bA",
      mapStyle: "mapbox://styles/invicta111/ckhr495nz00ay1amkd1bjstli",
      geoJsonSource: {
        //...some GeoJSON object
      },
      geoJsonLayer: {
        //...some GeoJSON layer configuration object
      },
    };
  },
};
</script>
