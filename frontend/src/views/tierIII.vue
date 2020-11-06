<template>
  <div class="tierIII">
    <navbar />
    <register Tier="tierIII"></register>
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