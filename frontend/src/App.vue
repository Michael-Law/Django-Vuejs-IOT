<template>
  <v-app>
    <navbar />
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
