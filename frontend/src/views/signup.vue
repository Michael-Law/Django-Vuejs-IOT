<template>
  <div class="signup">
    <navbar />
    <v-card>
      <v-card-title>Offers for enterprise</v-card-title>
      <v-slide-group v-model="model" class="pa-4" center-active show-arrows>
        <register />
      </v-slide-group>
    </v-card>
  </div>
</template>
<script>
import navbar from "@/components/navbar.vue";
import register from "@/components/register.vue";
export default {
  name: "Home",
  components: {
    navbar,
    register,
  },
};
</script>