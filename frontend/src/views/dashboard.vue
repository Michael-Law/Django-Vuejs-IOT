<template>
  <v-app>
    <navbar />
    <map_view />
  </v-app>
</template>

<script>
import navbar from "@/components/navbar.vue";
import map_view from "@/components/map.vue";

export default {
  name: "dashboard",

  components: {
    navbar,
    map_view,
  },

  data: () => ({
    //
  }),
};
</script>
<style>
</style>