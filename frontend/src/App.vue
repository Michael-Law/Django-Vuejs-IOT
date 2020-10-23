<template>
  <v-app>
    <navbar />
    <router-view />
  </v-app>
</template>

<script>
import HelloWorld from "./components/HelloWorld";
import navbar from "@/components/navbar.vue";

export default {
  name: "App",

  components: {
    HelloWorld,
    navbar,
  },

  data: () => ({
    //
  }),
};
</script>
