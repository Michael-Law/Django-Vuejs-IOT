<template>
  <v-app>
    <router-view />
  </v-app>
</template>

<script>
import HelloWorld from "./components/HelloWorld";
import router from "./router/index";
export default {
  name: "App",

  components: {
    HelloWorld,
  },

  data: () => ({
    //
  }),
};
</script>
<style>
</style>
