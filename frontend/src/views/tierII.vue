<template>
  <div class="tierII">
    <navbar />
    <register Tier="tierII"></register>
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