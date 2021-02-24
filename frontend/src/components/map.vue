<template>
  <b-container fluid>
    <b-row class="vh-100">
      <b-col lg="2"
        ><b-card
          title="Optimal Route"
          tag="article"
          style="max-width: 20rem"
          class="mb-2"
        >
          <b-card-text>
            <li v-for="item in optimalPlaces" :key="item">
              {{ item }}
            </li>
          </b-card-text>
        </b-card></b-col
      >
      <b-col cols="10">
        <MglMap
          :accessToken="mapboxAccessToken"
          :mapStyle.sync="mapStyle"
          :center="coordinates"
          :maxBounds="maxBounds"
        >
          <MglNavigationControl position="top-right" />
          <MglGeojsonLayer
            :sourceId="this.geoJsonSource.data.id"
            :source="this.geoJsonSource"
            layerId="somethingSomething"
            :layer="geoJsonLayer"
          /> </MglMap
      ></b-col>
    </b-row>
  </b-container>
</template>

<script>
name: "BaseMap";
import {
  MglMap,
  MglPopup,
  MglGeojsonLayer,
  MglNavigationControl,
} from "vue-mapbox";
import axios from "axios";

export default {
  components: {
    MglMap,
    MglPopup,
    MglNavigationControl,
    MglGeojsonLayer,
  },

  data: () => ({
    mapboxAccessToken:
      "pk.eyJ1IjoiaW52aWN0YTExMSIsImEiOiJja2hyMjM0cXAwcWJ1MnNyc2tzbGlneWw2In0.GDyvy8Yi8WBasxZKfH_0bA",
    mapStyle: "mapbox://styles/invicta111/ckhr495nz00ay1amkd1bjstli",
    coordinates: [57.59257009517367, -20.289607639709068],
    maxBounds: [
      [57.270295862485135, -20.531959293688164],
      [57.88271863500014, -19.974641272520877],
    ],
    optimalPlaces: [],
    geoJsonLayer: {
      id: "route",
      type: "line",
      source: "path",
      layout: {
        "line-join": "round",
        "line-cap": "round",
      },
      paint: {
        "line-color": "#ccff00",
        "line-width": 5,
      },
    },
    geoJsonSource: {
      type: "geojson",
      data: {
        id: "path",
        type: "Feature",
        properties: {},
        geometry: {
          type: "LineString",
          coordinates: [],
        },
      },
    },
    IncrementalPath: [],
  }),

  mounted() {
    axios
      .get("http://172.104.166.102:8000/api/OptimalRoute/?format=json")
      .then((response) => (this.IncrementalPath = response.data));
  },
  methods: {
    generateRoute(primeArray) {
      let myArray = [];
      var api_array = primeArray;
      var start, end, url;
      for (let step = 0; step < api_array.length - 1; step++) {
        start = api_array[step];
        end = api_array[step + 1];
        url =
          "https://api.mapbox.com/directions/v5/mapbox/driving/" +
          start[0] +
          "," +
          start[1] +
          ";" +
          end[0] +
          "," +
          end[1] +
          "?steps=true&geometries=geojson&access_token=" +
          this.mapboxAccessToken;

        axios
          .get(url)
          .then((response) =>
            myArray.push(...response.data.routes[0].geometry.coordinates)
          );
      }
      return myArray;
    },
  },

  watch: {
    IncrementalPath: {
      deep: true,
      handler(val, oldVal) {
        let me = this;
        console.log(oldVal);
        me.geoJsonSource.data.geometry.coordinates = me.generateRoute(val);
        axios
          .get("http://172.104.166.102:8000/api/OptimalPlaces/?format=json")
          .then((response) => (me.optimalPlaces = response.data));
      },
    },
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