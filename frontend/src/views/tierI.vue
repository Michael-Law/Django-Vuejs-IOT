<template>
  <div class="tierI">
    <navbar />
    <register Tier="TierI"></register>
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